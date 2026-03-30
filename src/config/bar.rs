use mlua::prelude::*;
use std::sync::Mutex;

static FONT_STRINGS: Mutex<Vec<std::ffi::CString>> = Mutex::new(Vec::new());

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

    srwm.set("bar", bar)?;
    Ok(())
}
