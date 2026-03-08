// Package core provides Go bindings to the srwm C window manager core.
//
// The C core handles all X11 state and event processing. This package
// exposes lifecycle control (Init, Run, Cleanup) and a status bar setter.
//
// IMPORTANT: Init() calls runtime.LockOSThread(). All subsequent calls
// to Run(), Cleanup(), SetStatus(), and Quit() MUST happen on the same
// goroutine that called Init(). X11 is not thread-safe.
package core

/*
#cgo CFLAGS: -std=c99 -pedantic -Wall -Wno-deprecated-declarations -O2
#cgo CFLAGS: -DXINERAMA -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_XOPEN_SOURCE=700L
#cgo pkg-config: x11 xinerama xft fontconfig freetype2
#cgo LDFLAGS: -lXrender -lImlib2

#include "bridge.h"
#include <stdlib.h>
*/
import "C"

import (
	"errors"
	"runtime"
	"unsafe"
)

// Init opens the X display, checks for other WMs, runs setup, and scans
// existing windows. Must be called before Run().
//
// Locks the current goroutine to its OS thread — all subsequent core
// calls must happen on this same goroutine.
func Init() error {
	runtime.LockOSThread()
	if C.srwm_init() != 0 {
		runtime.UnlockOSThread()
		return errors.New("srwm: failed to initialize (cannot open display?)")
	}
	return nil
}

// Run enters the blocking X11 event loop. Returns when the WM is asked
// to quit or restart (running == 0).
func Run() {
	C.srwm_run()
}

// Cleanup tears down all X resources and closes the display.
func Cleanup() {
	C.srwm_cleanup()
}

// Quit signals the C event loop to stop (clean exit, not restart).
func Quit() {
	C.srwm_quit()
}

// ShouldRestart returns true if the event loop exited due to a restart
// request (Mod+Shift+R) rather than a quit.
func ShouldRestart() bool {
	return C.srwm_should_restart() != 0
}

// Restart signals the C event loop to stop and triggers a restart.
func Restart() {
	C.srwm_request_restart()
}

// SetStatus sets the X root window name, which dwm reads as the status
// bar text. This replaces the old `xsetroot -name` approach.
func SetStatus(text string) {
	cs := C.CString(text)
	defer C.free(unsafe.Pointer(cs))
	C.srwm_set_status(cs)
}
