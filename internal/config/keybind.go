package config

import (
	"log"
	"strings"

	"github.com/nixuris/srwm/internal/core"
	lua "github.com/yuin/gopher-lua"
)

// RegisterKeybindAPI injects key-related functions and constants into the srwm module.
func RegisterKeybindAPI(L *lua.LState, srwmMod *lua.LTable) {
	// Modifiers (keep raw constants available)
	L.SetField(srwmMod, "Mod1", lua.LNumber(8))  // Mod1Mask (Alt)
	L.SetField(srwmMod, "Mod4", lua.LNumber(64)) // Mod4Mask (Super)
	L.SetField(srwmMod, "Shift", lua.LNumber(1)) // ShiftMask
	L.SetField(srwmMod, "Ctrl", lua.LNumber(4))  // ControlMask

	// Key namespace
	keyTable := L.NewTable()
	L.SetField(keyTable, "set", L.NewFunction(luaKeySet))   // hex version
	L.SetField(keyTable, "bind", L.NewFunction(luaKeyBind)) // string version
	L.SetField(srwmMod, "key", keyTable)
}

// luaKeyBind registers a dynamic keybinding using strings.
// srwm.key.bind("Mod4+Shift", "Return", callback)
func luaKeyBind(L *lua.LState) int {
	modStr := L.CheckString(1)
	keyName := L.CheckString(2)
	callback := L.CheckFunction(3)

	mod := parseModifiers(modStr)
	keysym := core.StringToKeysym(keyName)

	if keysym == 0 {
		L.RaiseError("invalid keysym name: %s", keyName)
		return 0
	}

	registerBinding(L, uint(mod), keysym, callback)
	return 0
}

// luaKeySet registers a dynamic keybinding using hex codes.
// srwm.key.set(mod, keysym, callback)
func luaKeySet(L *lua.LState) int {
	mod := L.CheckNumber(1)
	keysym := L.CheckNumber(2)
	callback := L.CheckFunction(3)

	registerBinding(L, uint(mod), uint(keysym), callback)
	return 0
}

func registerBinding(L *lua.LState, mod uint, keysym uint, callback *lua.LFunction) {
	// Capture the Lua state thread locally for this callback.
	co, _ := L.NewThread()

	goCb := func() {
		err := L.CallByParam(lua.P{
			Fn:      callback,
			NRet:    0,
			Protect: true,
		}, co)
		if err != nil {
			log.Printf("srwm.key callback error: %v", err)
		}
	}

	core.AddKeybinding(mod, keysym, goCb)
	core.GrabKeys() // Commit grabbed keys to X11 immediately.
}

func parseModifiers(s string) int {
	var mod int
	parts := strings.Split(s, "+")
	for _, p := range parts {
		p = strings.TrimSpace(strings.ToLower(p))
		switch p {
		case "mod1", "alt":
			mod |= 8
		case "mod4", "super", "win":
			mod |= 64
		case "shift":
			mod |= 1
		case "ctrl", "control":
			mod |= 4
		}
	}
	return mod
}
