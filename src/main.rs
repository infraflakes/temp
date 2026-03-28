<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> fe32d26 (DBus and XServer helper)
pub mod cli;
pub mod config;
pub mod dbus;
pub mod deploy;
pub mod ffi;
pub mod ipc;
pub mod xserver;

use crate::cli::Cli;
use clap::Parser;
<<<<<<< HEAD
use std::sync::atomic::AtomicBool;
=======
use std::sync::atomic::{AtomicBool, Ordering};
>>>>>>> fe32d26 (DBus and XServer helper)

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
<<<<<<< HEAD
        Some(crate::cli::Command::Shutdown) => {
            if let Err(e) = ipc::send_command("shutdown\n") {
=======
        Some(crate::cli::Command::Ipc { command }) => {
            let full_cmd = format!("{}\n", command);
            if let Err(e) = ipc::send_command(&full_cmd) {
>>>>>>> fe32d26 (DBus and XServer helper)
                eprintln!("srwm: {}", e);
                std::process::exit(1);
            }
        }
<<<<<<< HEAD
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
=======
>>>>>>> fe32d26 (DBus and XServer helper)
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

<<<<<<< HEAD
    let _xserver: Option<_> = if std::env::var("DISPLAY").is_err() {
=======
    let xserver: Option<_> = if std::env::var("DISPLAY").is_err() {
>>>>>>> fe32d26 (DBus and XServer helper)
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

        unsafe {
            if ffi::srwm_init_display() != 0 {
                eprintln!("srwm: cannot open display");
                std::process::exit(1);
            }

            let lua = mlua::Lua::new();
            ffi::set_lua_vm(&lua);

            if let Err(e) = config::load_config(&lua) {
                eprintln!("srwm: lua config error: {}", e);
            }

            ffi::srwm_init_setup();
            ffi::srwm_run();

            ffi::clear_lua_vm();
            ffi::srwm_cleanup();

            config::get_key_callbacks().clear();
            config::get_mouse_callbacks().clear();

            if ffi::srwm_should_restart() == 0 {
                break;
            }

            ffi::srwm_clear_keybindings();
            ffi::srwm_clear_mousebindings();
        }
    }

    ipc::set_running(false);
=======
mod ffi;

fn main() {
<<<<<<< HEAD
    println!("srwm built successfully");
>>>>>>> 9436516 (Scaffold)
=======
    // Parse args minimally — just "start" for now
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] != "start" {
        eprintln!("Usage: srwm [start]");
        std::process::exit(1);
    }

=======
mod config;
mod deploy;
mod ffi;

fn main() {
>>>>>>> d027391 (Config parsing reimplementation)
    loop {
        deploy::deploy_defaults();

        unsafe {
            if ffi::srwm_init_display() != 0 {
                eprintln!("srwm: cannot open display");
                std::process::exit(1);
            }

            let lua = mlua::Lua::new();
            ffi::set_lua_vm(&lua);

            if let Err(e) = config::load_config(&lua) {
                eprintln!("srwm: lua config error: {}", e);
            }

            ffi::srwm_init_setup();
            ffi::srwm_run();

            ffi::clear_lua_vm();
            ffi::srwm_cleanup();

            config::get_key_callbacks().clear();
            config::get_mouse_callbacks().clear();

            if ffi::srwm_should_restart() == 0 {
                break;
            }

            ffi::srwm_clear_keybindings();
            ffi::srwm_clear_mousebindings();
        }
    }
>>>>>>> 4d838fa (Minimal life cycle)
}
