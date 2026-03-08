package config

import (
	"log"
	"os/exec"
	"strings"
	"time"

	"github.com/nixuris/srwm/internal/core"
	lua "github.com/yuin/gopher-lua"
)

// refreshChan is used to wake up the Lua bar script from its sleep
// for immediate refreshes (e.g. after volume change).
var refreshChan = make(chan struct{}, 1)

// NotifyBarRefresh triggers an immediate refresh of the status bar
// by waking up the Lua sleep timer.
func NotifyBarRefresh() {
	select {
	case refreshChan <- struct{}{}:
	default:
		// Channel already has a pending refresh request
	}
}

// RegisterBarAPI injects bar-related functions into the srwm module.
func RegisterBarAPI(L *lua.LState, srwmMod *lua.LTable) {
	L.SetField(srwmMod, "set_status", L.NewFunction(luaSetStatus))
	L.SetField(srwmMod, "sleep", L.NewFunction(luaSleep))
	L.SetField(srwmMod, "spawn", L.NewFunction(luaSpawn))
	L.SetField(srwmMod, "restart", L.NewFunction(luaRestart))
	L.SetField(srwmMod, "quit", L.NewFunction(luaQuit))
}

func luaRestart(L *lua.LState) int {
	core.Restart()
	return 0
}

func luaQuit(L *lua.LState) int {
	core.Quit()
	return 0
}

func luaSpawn(L *lua.LState) int {
	cmd := L.CheckString(1)
	go func() {
		// Simple background spawn. No shell parsing here, just execution.
		// For shell parsing, users should use os.execute("...") but this is a helper.
		// Actually, let's use sh -c to be more flexible like os.execute but with our refresh callback.
		execCmd := strings.ReplaceAll(cmd, "'", "'\\''") // basic escaping
		out, err := exec.Command("sh", "-c", execCmd).CombinedOutput()
		if err != nil {
			log.Printf("srwm.spawn error: %v, output: %s", err, string(out))
		}
		NotifyBarRefresh()
	}()
	return 0
}

func luaSetStatus(L *lua.LState) int {
	text := L.CheckString(1)
	core.SetStatus(text)
	return 0
}

func luaSleep(L *lua.LState) int {
	sec := L.CheckNumber(1)
	select {
	case <-time.After(time.Duration(float64(sec) * float64(time.Second))):
	case <-refreshChan:
	case <-L.Context().Done():
	}
	return 0
}
