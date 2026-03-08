package config

import (
	"context"
	_ "embed"
	"fmt"
	"log"
	"os"
	"path/filepath"

	lua "github.com/yuin/gopher-lua"
)

//go:embed srwmrc.lua
var defaultSrwmrcScript []byte

//go:embed bar.lua
var defaultBarModule []byte

// StartConfig initializes the Lua VM, injects the `srwm` API, establishes
// the config files, and runs the main script in the background.
// It returns a completion channel that closes when the VM exits.
func StartConfig(ctx context.Context) {
	go runLuaConfig(ctx)
}

func runLuaConfig(ctx context.Context) {
	// ... determine config path (omitted unchanged parts) ...
	configDir := os.Getenv("XDG_CONFIG_HOME")
	if configDir == "" {
		home, err := os.UserHomeDir()
		if err != nil {
			log.Printf("lua: cannot locate home dir: %v", err)
			return
		}
		configDir = filepath.Join(home, ".config")
	}

	srwmDir := filepath.Join(configDir, "srwm")
	rcPath := filepath.Join(srwmDir, "srwmrc.lua")
	barLuaPath := filepath.Join(srwmDir, "bar.lua")

	if err := os.MkdirAll(srwmDir, 0755); err != nil {
		log.Printf("lua: failed to create config dir: %v", err)
		return
	}
	ensureFile(rcPath, defaultSrwmrcScript)
	ensureFile(barLuaPath, defaultBarModule)

	// 3. Initialize Lua VM with context for cancellation
	L := lua.NewState()
	L.OpenLibs()
	L.SetContext(ctx)
	defer L.Close()

	// 4. Set up package.path so 'require' finds files in ~/.config/srwm/
	pkg := L.GetGlobal("package")
	path := L.GetField(pkg, "path").String()
	L.SetField(pkg, "path", lua.LString(fmt.Sprintf("%s;%s/?.lua", path, srwmDir)))

	// 5. Expose srwm global module and register APIs
	srwmMod := L.NewTable()
	L.SetGlobal("srwm", srwmMod)

	RegisterBarAPI(L, srwmMod)
	RegisterKeybindAPI(L, srwmMod)

	// Preload the internal bar module as "bar"
	// This allows the user to do: local bar = require("bar")
	L.PreloadModule("bar", func(L *lua.LState) int {
		if err := L.DoString(string(defaultBarModule)); err != nil {
			L.RaiseError("failed to load bar.lua: %v", err)
		}
		return 1 // The module returns its table
	})

	// 6. Run the main entry point
	log.Printf("lua: executing %s", rcPath)
	if err := L.DoFile(rcPath); err != nil {
		log.Printf("lua: error running %s: %v", rcPath, err)
	}
}

func ensureFile(path string, content []byte) {
	if _, err := os.Stat(path); err == nil {
		return // already exists
	}
	if err := os.WriteFile(path, content, 0644); err != nil {
		log.Printf("lua: failed to deploy %s: %v", path, err)
	}
}
