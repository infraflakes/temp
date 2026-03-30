pub mod cli;
pub mod config;
pub mod deploy;
pub mod ffi;
pub mod ipc;
pub mod session;

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
        });

        let _ = register(SIGINT, || {
            SHOULD_QUIT.store(true, std::sync::atomic::Ordering::SeqCst);
        });

        let _ = register(SIGHUP, || {
            SHOULD_QUIT.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    }
}

pub fn main_run() {
    let _dbus = session::dbus::Session::start();

    let _xserver: Option<_> = if std::env::var("DISPLAY").is_err() {
        match session::xserver::start() {
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

    deploy::deploy_defaults();

    loop {
        unsafe {
            if ffi::srwm_init_display() != 0 {
                eprintln!("srwm: cannot open display");
                std::process::exit(1);
            }
        }

        let lua = mlua::Lua::new();
        ffi::set_lua_vm(&lua);

        if let Err(e) = config::load_config(&lua) {
            eprintln!("srwm: lua config error: {}", e);
        }

        unsafe {
            ffi::srwm_init_setup();
        }

        if config::compositor::is_enabled() {
            session::compositor::start();
        }

        if config::bar::is_external_bar_enabled() {
            if let Some(cmd) = config::bar::get_external_bar_command() {
                session::bar::start(&cmd);
            }
        }

        unsafe {
            ffi::srwm_run();
        }

        ffi::clear_lua_vm();
        unsafe {
            ffi::srwm_cleanup();
        }

        config::get_key_callbacks().clear();
        config::get_mouse_callbacks().clear();

        let should_restart: i32 = unsafe { ffi::srwm_should_restart() };
        if should_restart == 0 {
            break;
        }

        session::bar::stop();
        session::compositor::stop();
        config::compositor::reset();
        config::bar::reset_external_bar();
        config::bar::reset_tab_state();

        unsafe {
            ffi::srwm_clear_keybindings();
            ffi::srwm_clear_mousebindings();
        }
    }
    session::bar::stop();
    session::compositor::stop();
    ipc::set_running(false);
}
