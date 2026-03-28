<<<<<<< HEAD
use std::io::Write;

=======
>>>>>>> d027391 (Config parsing reimplementation)
const DEFAULT_SRWMRC: &str = include_str!("../config/srwmrc.lua");
const DEFAULT_GENERAL: &str = include_str!("../config/general.lua");
const DEFAULT_CANVAS: &str = include_str!("../config/canvas.lua");
const DEFAULT_KEYBINDINGS: &str = include_str!("../config/keybindings.lua");
const DEFAULT_THEMING: &str = include_str!("../config/theming.lua");
const DEFAULT_BAR: &str = include_str!("../config/bar.lua");
const DEFAULT_STARTUP: &str = include_str!("../config/startup.lua");
const DEFAULT_ENV: &str = include_str!("../config/env.lua");

<<<<<<< HEAD
pub fn deploy_defaults() -> bool {
    let dir = crate::config::config_dir();

    if !dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("srwm: failed to create config directory: {}", e);
            return false;
        }
    }

    let mut has_existing = false;
    let files = [
        "srwmrc.lua",
        "general.lua",
        "canvas.lua",
        "keybindings.lua",
        "theming.lua",
        "bar.lua",
        "startup.lua",
        "env.lua",
    ];

    for name in files {
        let path = dir.join(name);
        if path.exists() {
            has_existing = true;
            break;
        }
    }

    if has_existing {
        println!("srwm: config already exists in ~/.config/srwm/");
        println!("      You may want to backup your config before making changes.");
        println!("      Use 'srwm kickstart --force' to overwrite existing files.");
        return false;
    }

    write_files(&dir);
    true
}

pub fn force_deploy() -> bool {
    let dir = crate::config::config_dir();

    if !dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("srwm: failed to create config directory: {}", e);
            return false;
        }
    }

    println!("srwm: this will overwrite existing config files in ~/.config/srwm/");
    print!("      Continue? [y/N]: ");
    std::io::stdout().flush().ok();

    if !confirm_yes() {
        println!("srwm: cancelled.");
        return false;
    }

    write_files(&dir);
    true
}

fn write_files(dir: &std::path::Path) {
    let files = [
        ("srwmrc.lua", DEFAULT_SRWMRC),
        ("general.lua", DEFAULT_GENERAL),
        ("canvas.lua", DEFAULT_CANVAS),
        ("keybindings.lua", DEFAULT_KEYBINDINGS),
        ("theming.lua", DEFAULT_THEMING),
        ("bar.lua", DEFAULT_BAR),
        ("startup.lua", DEFAULT_STARTUP),
        ("env.lua", DEFAULT_ENV),
    ];

    for (name, content) in files {
        let path = dir.join(name);
        if let Err(e) = std::fs::write(&path, content) {
            eprintln!("srwm: failed to write {}: {}", name, e);
        }
    }
}

fn confirm_yes() -> bool {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).is_ok() {
        line.trim().to_lowercase() == "y"
    } else {
        false
    }
=======
pub fn deploy_defaults() {
    let dir = crate::config::config_dir();
    if dir.exists() {
        return;
    }
    std::fs::create_dir_all(&dir).ok();
    std::fs::write(dir.join("srwmrc.lua"), DEFAULT_SRWMRC).ok();
    std::fs::write(dir.join("general.lua"), DEFAULT_GENERAL).ok();
    std::fs::write(dir.join("canvas.lua"), DEFAULT_CANVAS).ok();
    std::fs::write(dir.join("keybindings.lua"), DEFAULT_KEYBINDINGS).ok();
    std::fs::write(dir.join("theming.lua"), DEFAULT_THEMING).ok();
    std::fs::write(dir.join("bar.lua"), DEFAULT_BAR).ok();
    std::fs::write(dir.join("startup.lua"), DEFAULT_STARTUP).ok();
    std::fs::write(dir.join("env.lua"), DEFAULT_ENV).ok();
>>>>>>> d027391 (Config parsing reimplementation)
}
