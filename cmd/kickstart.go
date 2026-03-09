package cmd

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/nixuris/srwm/internal/config"
	"github.com/spf13/cobra"
)

var kickstartCmd = &cobra.Command{
	Use:   "kickstart",
	Short: "Deploy default Lua configuration files",
	Long: `Deploy the default srwmrc.lua and bar.lua to ~/.config/srwm/

This command will fail if a config file already exists.
Backup your existing config before running this command.`,
	Run: func(cmd *cobra.Command, args []string) {
		runKickstart()
	},
}

func init() {
	rootCmd.AddCommand(kickstartCmd)
}

func runKickstart() {
	log.SetPrefix("srwm: ")
	log.SetFlags(0)

	srwmDir := config.ResolveConfigDir()
	if srwmDir == "" {
		log.Fatal("failed to resolve config directory")
	}

	rcPath := filepath.Join(srwmDir, "srwmrc.lua")
	barLuaPath := filepath.Join(srwmDir, "bar.lua")

	if _, err := os.Stat(rcPath); err == nil {
		log.Fatalf("config already exists at %s\nbackup your config and remove it before running kickstart", rcPath)
	}

	if err := os.MkdirAll(srwmDir, 0755); err != nil {
		log.Fatalf("failed to create config dir: %v", err)
	}

	if err := os.WriteFile(rcPath, config.DefaultSrwmrcScript, 0644); err != nil {
		log.Fatalf("failed to deploy srwmrc.lua: %v", err)
	}
	fmt.Printf("deployed: %s\n", rcPath)

	if err := os.WriteFile(barLuaPath, config.DefaultBarModule, 0644); err != nil {
		log.Fatalf("failed to deploy bar.lua: %v", err)
	}
	fmt.Printf("deployed: %s\n", barLuaPath)

	fmt.Println("kickstart complete!")
}
