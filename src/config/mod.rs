pub mod bar;
pub mod canvas;
pub mod compositor;
pub mod keys;
pub mod lifecycle;
pub mod mouse;
pub mod spawn;
pub mod window;
pub mod workspace;

use mlua::prelude::*;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Mutex;

pub(crate) static KEY_CALLBACKS: Mutex<Vec<LuaRegistryKey>> = Mutex::new(Vec::new());
pub(crate) static MOUSE_CALLBACKS: Mutex<Vec<LuaRegistryKey>> = Mutex::new(Vec::new());
pub(crate) static SPAWNED_ONCE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

pub fn validate_hex_color(s: &str) -> Result<(), String> {
    let s = s.trim();
    if !s.starts_with('#') || (s.len() != 7 && s.len() != 4) {
        return Err(format!("invalid color '{}': expected #RGB or #RRGGBB", s));
    }
    if !s[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("invalid color '{}': non-hex characters", s));
    }
    Ok(())
}

pub fn load_config(lua: &Lua) -> LuaResult<()> {
    let srwm = lua.create_table()?;

    keys::register(lua, &srwm)?;
    mouse::register(lua, &srwm)?;
    window::register(lua, &srwm)?;
    workspace::register_workspaces(lua, &srwm)?;
    workspace::register(lua, &srwm)?;
    canvas::register(lua, &srwm)?;
    bar::register(lua, &srwm)?;
    spawn::register(lua, &srwm)?;
    lifecycle::register(lua, &srwm)?;
    compositor::register(lua, &srwm)?;

    lua.globals().set("srwm", srwm)?;

    register_include(lua)?;

    let config_dir = config_dir();
    let entry = config_dir.join("srwmrc.lua");
    if entry.exists() {
        let code = std::fs::read_to_string(&entry)
            .map_err(|e| mlua::Error::runtime(format!("read {}: {e}", entry.display())))?;
        lua.load(&code)
            .set_name(entry.to_string_lossy().as_ref())
            .exec()?;
    } else {
        eprintln!("srwm: warning: no config file found at {}", entry.display());
        eprintln!("srwm: run 'srwm kickstart' to generate default config");
    }
    Ok(())
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("srwm")
}

pub(crate) fn get_key_callbacks() -> std::sync::MutexGuard<'static, Vec<LuaRegistryKey>> {
    KEY_CALLBACKS.lock().unwrap_or_else(|e| e.into_inner())
}

pub(crate) fn get_mouse_callbacks() -> std::sync::MutexGuard<'static, Vec<LuaRegistryKey>> {
    MOUSE_CALLBACKS.lock().unwrap_or_else(|e| e.into_inner())
}

fn register_include(lua: &Lua) -> LuaResult<()> {
    let include = lua.create_function(|lua, name: String| {
        let path = config_dir().join(format!("{}.lua", name));
        let code = std::fs::read_to_string(&path)
            .map_err(|e| mlua::Error::runtime(format!("include {}.lua: {}", name, e)))?;
        lua.load(&code)
            .set_name(path.to_string_lossy().as_ref())
            .exec()?;
        Ok(())
    })?;
    lua.globals().set("include", include)?;
    Ok(())
}
