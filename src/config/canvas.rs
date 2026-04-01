use mlua::prelude::*;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let canvas = lua.create_table()?;

    canvas.set(
        "move",
        lua.create_function(|_, dir: i32| {
            unsafe {
                crate::ffi::srwm_action_movecanvas(dir);
            }
            Ok(())
        })?,
    )?;

    canvas.set(
        "home",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_action_homecanvas();
            }
            Ok(())
        })?,
    )?;

    canvas.set(
        "center_window",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_action_centerwindowoncanvas();
            }
            Ok(())
        })?,
    )?;

    canvas.set(
        "zoom",
        lua.create_function(|_, dir: i32| {
            unsafe {
                crate::ffi::srwm_action_zoomcanvas(dir);
            }
            Ok(())
        })?,
    )?;

    canvas.set(
        "edge_autopan",
        lua.create_function(|_, enabled: bool| {
            unsafe {
                crate::ffi::srwm_set_edge_autopan(if enabled { 1 } else { 0 });
            }
            Ok(())
        })?,
    )?;

    srwm.set("canvas", canvas)?;
    Ok(())
}
