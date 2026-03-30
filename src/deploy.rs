use std::io::Write;

const DEFAULT_SRWMRC: &str = include_str!("../config/srwmrc.lua");
const DEFAULT_CANVAS: &str = include_str!("../config/canvas.lua");
const DEFAULT_COMPOSITOR: &str = include_str!("../config/compositor.lua");
const DEFAULT_KEYBINDINGS: &str = include_str!("../config/keybindings.lua");

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
        "canvas.lua",
        "compositor.lua",
        "keybindings.lua",
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
        ("canvas.lua", DEFAULT_CANVAS),
        ("compositor.lua", DEFAULT_COMPOSITOR),
        ("keybindings.lua", DEFAULT_KEYBINDINGS),
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
}
