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
            unsafe {
                crate::ffi::srwm_set_borderpx(v);
            }
            Ok(())
        })?,
    )?;
    srwm.set("cfg", cfg)?;

    srwm.set(
        "env",
        lua.create_function(|_lua, args: mlua::Variadic<mlua::String>| {
            if args.len() >= 2 {
                let key = args[0].to_str().unwrap().to_string();
                let value = args[1].to_str().unwrap().to_string();
                unsafe {
                    std::env::set_var(&key, &value);
                }
            }
            Ok(mlua::Value::Nil)
        })?,
    )?;

    Ok(())
}
