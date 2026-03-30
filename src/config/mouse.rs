use mlua::prelude::*;

use crate::config::keys::parse_modifiers;

pub const BUTTON1: i32 = 1;
pub const BUTTON2: i32 = 2;
pub const BUTTON3: i32 = 3;
pub const BUTTON4: i32 = 4;
pub const BUTTON5: i32 = 5;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let mouse_table = lua.create_table()?;

    let bind_fn = lua.create_function(
        |lua, (mods, button, target, cb): (String, String, String, LuaFunction)| {
            let modmask = parse_modifiers(&mods);

            let btn = match button.to_lowercase().as_str() {
                "button1" | "left" => BUTTON1,
                "button2" | "middle" => BUTTON2,
                "button3" | "right" => BUTTON3,
                "button4" | "scrollup" => BUTTON4,
                "button5" | "scrolldown" => BUTTON5,
                _ => {
                    return Err(mlua::Error::runtime(format!("invalid button: {}", button)));
                }
            };

            let click = match target.to_lowercase().as_str() {
                "root" => crate::config::keys::CLICK_ROOT as u32,
                "client" => crate::config::keys::CLICK_CLIENT as u32,
                _ => {
                    return Err(mlua::Error::runtime(format!(
                        "invalid target: {} (expected root, client)",
                        target
                    )));
                }
            };

            let reg_key = lua.create_registry_value(cb)?;
            let mut cbs = crate::config::get_mouse_callbacks();
            let id = cbs.len() as i32;
            cbs.push(reg_key);
            drop(cbs);

            unsafe {
                crate::ffi::srwm_add_mousebinding(click, modmask, btn as u32, id);
            }
            Ok(())
        },
    )?;

    mouse_table.set("bind", bind_fn)?;
    srwm.set("mouse", mouse_table)?;
    Ok(())
}
