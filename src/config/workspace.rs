use mlua::prelude::*;

pub fn register_workspaces(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let ws = lua.create_table()?;

    ws.set(
        "set_label",
        lua.create_function(|_, input: String| {
            let parts: Vec<&str> = input.split(',').collect();
            let count = parts.len().min(9);
            for (i, part) in parts.iter().take(count).enumerate() {
                let c = std::ffi::CString::new(part.trim()).unwrap();
                unsafe {
                    crate::ffi::srwm_set_ws_label(i as i32, c.as_ptr());
                }
            }
            unsafe {
                crate::ffi::srwm_set_ws_count(count as i32);
            }
            Ok(())
        })?,
    )?;

    srwm.set("workspaces", ws)?;
    Ok(())
}

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let ws = lua.create_table()?;

    ws.set(
        "view",
        lua.create_function(|_, idx: i32| {
            unsafe {
                crate::ffi::srwm_action_view(idx - 1);
            }
            Ok(())
        })?,
    )?;

    ws.set(
        "move_window_to",
        lua.create_function(|_, idx: i32| {
            unsafe {
                crate::ffi::srwm_action_move_to_ws(idx - 1);
            }
            Ok(())
        })?,
    )?;

    ws.set(
        "shift_view",
        lua.create_function(|_, dir: i32| {
            unsafe {
                crate::ffi::srwm_action_shiftview(dir);
            }
            Ok(())
        })?,
    )?;

    ws.set(
        "view_prev",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_action_ws_to_prev();
            }
            Ok(())
        })?,
    )?;

    ws.set(
        "view_next",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_action_ws_to_next();
            }
            Ok(())
        })?,
    )?;

    ws.set(
        "move_to_monitor",
        lua.create_function(|_, dir: i32| {
            unsafe {
                crate::ffi::srwm_action_move_window_to_monitor(dir);
            }
            Ok(())
        })?,
    )?;

    srwm.set("workspace", ws)?;
    Ok(())
}
