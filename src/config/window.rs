use mlua::prelude::*;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let win = lua.create_table()?;

    win.set(
        "kill",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_action_killclient();
            }
            Ok(())
        })?,
    )?;

    win.set(
        "toggle_fullscreen",
        lua.create_function(|_, ()| {
            unsafe {
                crate::ffi::srwm_action_togglefullscr();
            }
            Ok(())
        })?,
    )?;

    win.set(
        "focus",
        lua.create_function(|_, dir: i32| {
            unsafe {
                crate::ffi::srwm_action_focusstack(dir);
            }
            Ok(())
        })?,
    )?;

    let border = lua.create_table()?;
    border.set(
        "active",
        lua.create_function(|_, hex: String| {
            let c = std::ffi::CString::new(hex).unwrap();
            unsafe {
                crate::ffi::srwm_set_border_active(c.as_ptr());
            }
            Ok(())
        })?,
    )?;
    border.set(
        "inactive",
        lua.create_function(|_, hex: String| {
            let c = std::ffi::CString::new(hex).unwrap();
            unsafe {
                crate::ffi::srwm_set_border_inactive(c.as_ptr());
            }
            Ok(())
        })?,
    )?;
    win.set("border", border)?;

    srwm.set("window", win)?;
    Ok(())
}
