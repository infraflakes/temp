package cmd

import (
	"context"
	"log"

	"github.com/nixuris/srwm/internal/config"
	"github.com/nixuris/srwm/internal/core"
	"github.com/nixuris/srwm/internal/ipc"
	"github.com/spf13/cobra"
)

var startCmd = &cobra.Command{
	Use:   "start",
	Short: "Start the window manager",
	Run: func(cmd *cobra.Command, args []string) {
		socketPath := ipc.DefaultSocketPath()
		runWM(socketPath)
	},
}

func init() {
	rootCmd.AddCommand(startCmd)
}

func runWM(socketPath string) {
	log.SetPrefix("srwm: ")
	log.SetFlags(0)

	// Start IPC server
	go func() {
		if err := ipc.Listen(socketPath); err != nil {
			log.Printf("IPC server error: %v", err)
		}
	}()

	for {
		if err := core.Init(); err != nil {
			log.Fatalf("init failed: %v", err)
		}

		// Start Lua Configuration and Bar WITH context
		ctx, cancel := context.WithCancel(context.Background())
		config.StartConfig(ctx)

		core.Run() // blocks until `running = 0`

		// Stop Lua before cleaning up C core
		cancel()
		core.Cleanup()

		if !core.ShouldRestart() {
			break // clean exit, not restart
		}
		// loop back -> re-init for hot-reload
	}
}
