package main

import (
	"fmt"
	"log"
	"os"

	"github.com/nixuris/srnwm/internal/core"
)

// Version is set at build time via -ldflags.
var Version = "dev"

func main() {
	if len(os.Args) < 2 {
		printUsage()
		os.Exit(1)
	}

	switch os.Args[1] {
	case "start":
		runWM()
	case "version":
		fmt.Printf("swm %s\n", Version)
	default:
		fmt.Fprintf(os.Stderr, "swm: unknown command %q\n", os.Args[1])
		printUsage()
		os.Exit(1)
	}
}

func printUsage() {
	fmt.Fprintln(os.Stderr, "Usage: swm <command>")
	fmt.Fprintln(os.Stderr, "")
	fmt.Fprintln(os.Stderr, "Commands:")
	fmt.Fprintln(os.Stderr, "  start     Start the window manager")
	fmt.Fprintln(os.Stderr, "  version   Print version information")
}

func runWM() {
	log.SetPrefix("swm: ")
	log.SetFlags(0)

	for {
		if err := core.Init(); err != nil {
			log.Fatalf("init failed: %v", err)
		}

		log.Println("started")
		core.Run() // blocks until quit or restart
		core.Cleanup()

		if !core.ShouldRestart() {
			log.Println("exiting")
			break
		}

		log.Println("restarting...")
		// loop back → re-init
	}
}
