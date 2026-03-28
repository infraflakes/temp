use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "srwm")]
#[command(version = "0.5.4")]
#[command(about = "Simple Rust Window Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the window manager (default)
    Start,
    /// Print version information
    Version,
<<<<<<< HEAD
    /// Shut down the running instance
    Shutdown,
    /// Restart the running instance
    Restart,
    /// Deploy config files to ~/.config/srwm/ without starting the WM
    Kickstart {
        /// Overwrite existing config files
        #[arg(short, long)]
        force: bool,
=======
    /// Send a command to a running instance via IPC
    Ipc {
        /// Command to send: shutdown, restart, or refresh
        command: String,
>>>>>>> fe32d26 (DBus and XServer helper)
    },
}
