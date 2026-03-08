package config

import (
	"time"

	"github.com/nixuris/srwm/internal/core"
	lua "github.com/yuin/gopher-lua"
)

// refreshChan is used to wake up the Lua bar script from its
// interruptible sleep for immediate status bar refreshes (e.g. after
// a volume change via srwm.spawn).
var refreshChan = make(chan struct{}, 1)

// NotifyBarRefresh triggers an immediate refresh of the status bar
// by sending a signal on refreshChan, which unblocks luaSleep.
//
// This is safe to call from any goroutine. If a refresh is already
// pending it is a no-op (the channel is buffered with capacity 1).
func NotifyBarRefresh() {
	select {
	case refreshChan <- struct{}{}:
	default:
	}
}

// RegisterBarAPI injects status-bar-specific functions into the srwm
// Lua module table:
//
//   - srwm.set_status(text) — set the X root window name (bar text).
//   - srwm.sleep(seconds)   — interruptible sleep that wakes on bar
//     refresh requests or context cancellation.
func RegisterBarAPI(L *lua.LState, srwmMod *lua.LTable) {
	L.SetField(srwmMod, "set_status", L.NewFunction(luaSetStatus))
	L.SetField(srwmMod, "sleep", L.NewFunction(luaSleep))
}

// luaSetStatus updates the X root window name, which dwm renders as
// the status bar text.
//
// Lua signature: srwm.set_status(text: string)
func luaSetStatus(L *lua.LState) int {
	text := L.CheckString(1)
	core.SetStatus(text)
	return 0
}

// luaSleep blocks the Lua script for the given number of seconds, but
// wakes up early if:
//   - A bar refresh is requested (via NotifyBarRefresh).
//   - The Lua VM's context is cancelled (WM shutdown/restart).
//
// Lua signature: srwm.sleep(seconds: number)
func luaSleep(L *lua.LState) int {
	sec := L.CheckNumber(1)
	select {
	case <-time.After(time.Duration(float64(sec) * float64(time.Second))):
	case <-refreshChan:
	case <-L.Context().Done():
	}
	return 0
}
