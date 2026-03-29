pub mod cli;
pub mod config;
pub mod dbus;
pub mod deploy;
pub mod ffi;
pub mod ipc;
pub mod xserver;

use crate::cli::Cli;
use clap::Parser;
use std::sync::atomic::AtomicBool;

static SHOULD_QUIT: AtomicBool = AtomicBool::new(false);

fn main() {
    setup_signal_handlers();

    let cli = Cli::parse();
    match cli.command {
        Some(crate::cli::Command::Start) | None => {
            main_run();
        }
        Some(crate::cli::Command::Version) => {
            println!("srwm {}", env!("CARGO_PKG_VERSION"));
        }
        Some(crate::cli::Command::Shutdown) => {
            if let Err(e) = ipc::send_command("shutdown\n") {
                eprintln!("srwm: {}", e);
                std::process::exit(1);
            }
        }
        Some(crate::cli::Command::Restart) => {
            if let Err(e) = ipc::send_command("restart\n") {
                eprintln!("srwm: {}", e);
                std::process::exit(1);
            }
        }
        Some(crate::cli::Command::Kickstart { force }) => {
            if force {
                if deploy::force_deploy() {
                    println!("srwm: config deployed (forced) to ~/.config/srwm/");
                }
            } else {
                if deploy::deploy_defaults() {
                    println!("srwm: config deployed to ~/.config/srwm/");
                }
            }
        }
    }
}

fn setup_signal_handlers() {
    use signal_hook::consts::signal::*;
    use signal_hook::low_level::register;

    unsafe {
        let _ = register(SIGTERM, || {
            SHOULD_QUIT.store(true, std::sync::atomic::Ordering::SeqCst);
            crate::ffi::srwm_quit();
        });

        let _ = register(SIGINT, || {
            SHOULD_QUIT.store(true, std::sync::atomic::Ordering::SeqCst);
            crate::ffi::srwm_quit();
        });

        let _ = register(SIGHUP, || {
            crate::ffi::srwm_request_restart();
        });
    }
}

pub fn main_run() {
    let _dbus = dbus::Session::start();

    let _xserver: Option<_> = if std::env::var("DISPLAY").is_err() {
        match xserver::start() {
            Ok(srv) => Some(srv),
            Err(e) => {
                eprintln!("srwm: failed to start X server: {}", e);
                None
            }
        }
    } else {
        None
    };

    ipc::start_ipc_server();

    loop {
        deploy::deploy_defaults();

    loop {
        unsafe {
            if ffi::srwm_init_display() != 0 {
                eprintln!("srwm: cannot open display");
                std::process::exit(1);
            }

            // TODO: Phase 2B — load Lua config here
            // config::deploy_defaults();
            // config::load();

            ffi::srwm_init_setup();
            ffi::srwm_run(); // blocks until running == 0  

            ffi::srwm_cleanup();

            if ffi::srwm_should_restart() == 0 {
                break;
            }
            // TODO: Phase 2C — clear keybindings here
        }
    }

    ipc::set_running(false);
}
