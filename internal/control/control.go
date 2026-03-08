package control

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"path/filepath"

	"github.com/nixuris/srwm/internal/core"
)

// DefaultSocketPath returns the path to the Unix socket used for IPC.
func DefaultSocketPath() string {
	// Try to use XDG_RUNTIME_DIR if available, fallback to /tmp
	runtimeDir := os.Getenv("XDG_RUNTIME_DIR")
	if runtimeDir != "" {
		return filepath.Join(runtimeDir, "srwm.sock")
	}
	return "/tmp/srwm.sock"
}

// Listen starts the IPC server and listens for commands.
// It blocks until the socket can no longer be served.
func Listen(socketPath string) error {
	// Remove existing socket if it exists
	if _, err := os.Stat(socketPath); err == nil {
		if err := os.Remove(socketPath); err != nil {
			return fmt.Errorf("failed to remove existing socket: %w", err)
		}
	}

	ln, err := net.Listen("unix", socketPath)
	if err != nil {
		return fmt.Errorf("failed to listen on socket: %w", err)
	}
	defer ln.Close()
	defer os.Remove(socketPath)

	for {
		conn, err := ln.Accept()
		if err != nil {
			return err
		}
		go handleConnection(conn)
	}
}

func handleConnection(conn net.Conn) {
	defer conn.Close()
	scanner := bufio.NewScanner(conn)
	for scanner.Scan() {
		command := scanner.Text()
		switch command {
		case "shutdown":
			fmt.Println("control: received shutdown request")
			core.Quit()
			return
		case "restart":
			fmt.Println("control: received restart request")
			core.Restart()
			return
		default:
			fmt.Printf("control: unknown command received: %q\n", command)
		}
	}
}

// Send sends a command to a running srwm instance via the Unix socket.
func Send(socketPath, command string) error {
	conn, err := net.Dial("unix", socketPath)
	if err != nil {
		return fmt.Errorf("could not connect to srwm: %w (is it running?)", err)
	}
	defer conn.Close()

	_, err = fmt.Fprintln(conn, command)
	if err != nil {
		return fmt.Errorf("failed to send command: %w", err)
	}

	return nil
}
