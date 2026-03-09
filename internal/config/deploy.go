// Package config manages the srwm Lua runtime: it initializes the VM,
// deploys default configuration files, registers Go-backed APIs, and
// executes the user's srwmrc.lua entry point.
//
// The Lua VM runs in its own goroutine and is governed by a
// context.Context for clean cancellation during WM restarts.
package config

import (
	"context"
	_ "embed"
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/nixuris/srwm/internal/control"
	lua "github.com/yuin/gopher-lua"
)

// defaultSrwmrcScript is the embedded default srwmrc.lua, deployed to
// ~/.config/srwm/srwmrc.lua on first run so the user has a working
// starting point they can edit.
//
//go:embed lua/srwmrc.lua
var DefaultSrwmrcScript []byte

// DefaultBarModule is the embedded default bar.lua, deployed alongside
// srwmrc.lua. It provides the status bar implementation as a Lua module
// that srwmrc.lua loads via require("bar").
//
//go:embed lua/bar.lua
var DefaultBarModule []byte

// StartConfig boots the Lua runtime in a background goroutine.
//
// The provided context controls the VM lifetime: when ctx is cancelled
// (e.g. during a WM restart), the Lua sleep loop wakes up and the VM
// shuts down gracefully.
//
// Call this AFTER core.Init() to ensure the X11 display is ready before
// Lua tries to register keybindings.
func StartConfig(ctx context.Context) {
	go runLuaConfig(ctx)
}

// runLuaConfig is the main Lua configuration goroutine. It:
//  1. Resolves the config directory (~/.config/srwm/).
//  2. Requires srwmrc.lua to exist (no auto-deploy).
//  3. Creates and configures the Lua VM.
//  4. Registers the srwm.* API surface.
//  5. Executes srwmrc.lua (which blocks in the bar loop).
func runLuaConfig(ctx context.Context) {
	srwmDir := ResolveConfigDir()
	if srwmDir == "" {
		return
	}

	rcPath := filepath.Join(srwmDir, "srwmrc.lua")

	if err := os.MkdirAll(srwmDir, 0755); err != nil {
		log.Printf("lua: failed to create config dir: %v", err)
		return
	}

	if _, err := os.Stat(rcPath); os.IsNotExist(err) {
		log.Printf("lua: config not found at %s", rcPath)
		log.Printf("lua: run 'srwm kickstart' to deploy default config")
		return
	}

	L := lua.NewState()
	L.OpenLibs()
	L.SetContext(ctx)
	defer L.Close()

	// Extend package.path so require("bar") finds ~/.config/srwm/bar.lua.
	pkg := L.GetGlobal("package")
	luaPath := L.GetField(pkg, "path").String()
	L.SetField(pkg, "path", lua.LString(fmt.Sprintf("%s;%s/?.lua", luaPath, srwmDir)))

	// Build the srwm.* global module table.
	srwmMod := L.NewTable()
	L.SetGlobal("srwm", srwmMod)

	RegisterBarAPI(L, srwmMod)
	RegisterKeybindAPI(L, srwmMod)
	control.RegisterAPI(L, srwmMod)

	// Preload bar.lua as a require-able module so the user can do:
	//   local bar = require("bar")
	L.PreloadModule("bar", func(L *lua.LState) int {
		if err := L.DoString(string(DefaultBarModule)); err != nil {
			L.RaiseError("failed to load bar.lua: %v", err)
		}
		return 1
	})

	log.Printf("lua: executing %s", rcPath)
	if err := L.DoFile(rcPath); err != nil {
		log.Printf("lua: error running %s: %v", rcPath, err)
	}
}

// resolveConfigDir returns the srwm config directory path
// (~/.config/srwm), respecting XDG_CONFIG_HOME.
func ResolveConfigDir() string {
	configDir := os.Getenv("XDG_CONFIG_HOME")
	if configDir == "" {
		home, err := os.UserHomeDir()
		if err != nil {
			log.Printf("lua: cannot locate home dir: %v", err)
			return ""
		}
		configDir = filepath.Join(home, ".config")
	}
	return filepath.Join(configDir, "srwm")
}

// deployDefault writes content to path only if the file does not already
// exist. This is used to deploy default config files on first run without
// overwriting user edits.
func deployDefault(path string, content []byte) {
	if _, err := os.Stat(path); err == nil {
		return // already exists, don't overwrite
	}
	if err := os.WriteFile(path, content, 0644); err != nil {
		log.Printf("lua: failed to deploy %s: %v", path, err)
	}
}
