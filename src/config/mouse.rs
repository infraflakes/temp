use mlua::prelude::*;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let mouse_table = lua.create_table()?;

    let bind_fn = lua.create_function(
        |lua, (mods, button, target, cb): (String, String, String, LuaFunction)| {
            let modmask = crate::config::keys::parse_modifiers(&mods);

            let btn = match button.to_lowercase().as_str() {
                "button1" | "left" => 1,
                "button2" | "middle" => 2,
                "button3" | "right" => 3,
                "button4" | "scrollup" => 4,
                "button5" | "scrolldown" => 5,
                _ => {
                    return Err(mlua::Error::runtime(format!("invalid button: {}", button)));
                }
            };

            let click = match target.to_lowercase().as_str() {
                "root" => 5,
                "client" => 4,
                _ => {
                    return Err(mlua::Error::runtime(format!(
                        "invalid target: {} (expected root, client)",
                        target
                    )));
                }
            };

            let reg_key = lua.create_registry_value(cb)?;
            let mut cbs = crate::config::MOUSE_CALLBACKS.lock().unwrap();
            let id = cbs.len() as i32;
            cbs.push(reg_key);
            drop(cbs);

            unsafe {
                crate::ffi::srwm_add_mousebinding(click, modmask, btn, id);
            }
            Ok(())
        },
    )?;

    mouse_table.set("bind", bind_fn)?;
    srwm.set("mouse", mouse_table)?;
    Ok(())
}
