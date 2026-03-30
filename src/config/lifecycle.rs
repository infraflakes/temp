use mlua::prelude::*;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    srwm.set(
        "quit",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_quit();
            }
            Ok(())
        })?,
    )?;

    srwm.set(
        "restart",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_request_restart();
            }
            Ok(())
        })?,
    )?;

    let cfg = lua.create_table()?;
    cfg.set(
        "borderpx",
        lua.create_function(|_, v: u32| {
            if v > 50 {
                return Err(mlua::Error::runtime(format!(
                    "borderpx must be 0..50, got {}",
                    v
                )));
            }
            unsafe {
                crate::ffi::srwm_set_borderpx(v);
            }
            Ok(())
        })?,
    )?;
    srwm.set("cfg", cfg)?;

    srwm.set(
        "env",
        lua.create_function(|_lua, (key, value): (String, String)| {
            unsafe {
                std::env::set_var(&key, &value);
            }
            Ok(())
        })?,
    )?;

    Ok(())
}
