use mlua::prelude::*;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

static FONT_STRINGS: Mutex<Vec<std::ffi::CString>> = Mutex::new(Vec::new());

static EXTERNAL_BAR_ENABLED: AtomicBool = AtomicBool::new(false);
static EXTERNAL_BAR_COMMAND: Mutex<Option<String>> = Mutex::new(None);

pub fn is_external_bar_enabled() -> bool {
    EXTERNAL_BAR_ENABLED.load(Ordering::SeqCst)
}

pub fn get_external_bar_command() -> Option<String> {
    EXTERNAL_BAR_COMMAND
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
}

pub fn reset_external_bar() {
    EXTERNAL_BAR_ENABLED.store(false, Ordering::SeqCst);
    *EXTERNAL_BAR_COMMAND
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = None;
}

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let bar = lua.create_table()?;

    bar.set(
        "fonts",
        lua.create_function(|_, font: String| {
            let c = std::ffi::CString::new(font)
                .map_err(|_| mlua::Error::runtime("invalid NUL byte in font name"))?;
            let ptr = c.as_ptr();
            FONT_STRINGS
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(c);
            unsafe {
                crate::ffi::srwm_set_font(ptr);
            }
            Ok(())
        })?,
    )?;

    bar.set(
        "tab_height",
        lua.create_function(|_, v: i32| {
            unsafe {
                crate::ffi::srwm_set_tab_height(v);
            }
            Ok(())
        })?,
    )?;

    bar.set(
        "tab_tile_vertical_padding",
        lua.create_function(|_, v: i32| {
            unsafe {
                crate::ffi::srwm_set_tab_tile_vertical_padding(v);
            }
            Ok(())
        })?,
    )?;

    bar.set(
        "tab_tile_inner_padding_horizontal",
        lua.create_function(|_, v: i32| {
            unsafe {
                crate::ffi::srwm_set_tab_tile_inner_padding_horizontal(v);
            }
            Ok(())
        })?,
    )?;

    bar.set(
        "tab_tile_outer_padding_horizontal",
        lua.create_function(|_, v: i32| {
            unsafe {
                crate::ffi::srwm_set_tab_tile_outer_padding_horizontal(v);
            }
            Ok(())
        })?,
    )?;

    bar.set(
        "tab_top",
        lua.create_function(|_, v: bool| {
            unsafe {
                crate::ffi::srwm_set_toptab(if v { 1 } else { 0 });
            }
            Ok(())
        })?,
    )?;

    bar.set(
        "theme",
        lua.create_function(|_, tbl: LuaTable| {
            let two_color_map: &[(&str, i32)] = &[("tab_selected", 0), ("tab_normal", 1)];
            for &(key, scheme) in two_color_map {
                if let Ok(t) = tbl.get::<LuaTable>(key) {
                    if let Ok(fg) = t.get::<String>(1) {
                        let c = std::ffi::CString::new(fg)
                            .map_err(|_| mlua::Error::runtime("invalid NUL byte in color"))?;
                        unsafe {
                            crate::ffi::srwm_set_color(scheme, 0, c.as_ptr());
                        }
                    }
                    if let Ok(bg) = t.get::<String>(2) {
                        let c = std::ffi::CString::new(bg)
                            .map_err(|_| mlua::Error::runtime("invalid NUL byte in color"))?;
                        unsafe {
                            crate::ffi::srwm_set_color(scheme, 1, c.as_ptr());
                        }
                    }
                }
            }
            let one_color_map: &[(&str, i32)] =
                &[("button_prev", 2), ("button_next", 3), ("button_close", 4)];
            for &(key, scheme) in one_color_map {
                if let Ok(hex) = tbl.get::<String>(key) {
                    let c = std::ffi::CString::new(hex)
                        .map_err(|_| mlua::Error::runtime("invalid NUL byte in color"))?;
                    unsafe {
                        crate::ffi::srwm_set_color(scheme, 0, c.as_ptr());
                    }
                }
            }
            Ok(())
        })?,
    )?;

    bar.set(
        "enable",
        lua.create_function(|_, (enabled, cmd): (bool, String)| {
            EXTERNAL_BAR_ENABLED.store(enabled, Ordering::SeqCst);
            *EXTERNAL_BAR_COMMAND
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = Some(cmd);
            Ok(())
        })?,
    )?;

    srwm.set("bar", bar)?;
    Ok(())
}
