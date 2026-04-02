use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "srwm")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Simple Rust Window Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the window manager (default)
    Start {
        /// Replace an existing window manager
        #[arg(long)]
        replace: bool,
    },
    /// Print version information
    Version,
    /// Shut down the running instance
    Shutdown,
    /// Restart the running instance
    Restart,
    /// Deploy config files to ~/.config/srwm/ without starting the WM
    Kickstart {
        /// Overwrite existing config files
        #[arg(short, long)]
        force: bool,
    },
}
