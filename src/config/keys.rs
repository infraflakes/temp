use mlua::prelude::*;

pub const SHIFT_MASK: u32 = 1;
pub const LOCK_MASK: u32 = 2;
pub const CONTROL_MASK: u32 = 4;
pub const MOD1_MASK: u32 = 8;
pub const MOD4_MASK: u32 = 64;

pub const CLICK_ROOT: i32 = 5;
pub const CLICK_CLIENT: i32 = 4;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let key_table = lua.create_table()?;

    lua.globals().set("Mod1", MOD1_MASK)?;
    lua.globals().set("Mod4", MOD4_MASK)?;
    lua.globals().set("Shift", SHIFT_MASK)?;
    lua.globals().set("Ctrl", CONTROL_MASK)?;

    let bind_fn = lua.create_function(|lua, (mods, key, cb): (String, String, LuaFunction)| {
        let modmask = parse_modifiers(&mods);
        let keysym = string_to_keysym(&key)
            .map_err(|_| mlua::Error::runtime(format!("unknown key: {}", key)))?;
        if keysym == 0 {
            return Err(mlua::Error::runtime(format!("unknown key: {}", key)));
        }

        let reg_key = lua.create_registry_value(cb)?;
        let mut cbs = crate::config::get_key_callbacks();
        let id = cbs.len() as i32;
        cbs.push(reg_key);
        drop(cbs);

        unsafe {
            crate::ffi::srwm_add_keybinding(modmask, keysym.into(), id);
            crate::ffi::srwm_grabkeys();
        }
        Ok(())
    })?;

    key_table.set("bind", bind_fn)?;
    srwm.set("key", key_table)?;
    Ok(())
}

pub fn parse_modifiers(s: &str) -> u32 {
    let mut mask = 0u32;
    for part in s.split('+') {
        match part.trim() {
            "Shift" => mask |= SHIFT_MASK,
            "Ctrl" | "Control" => mask |= CONTROL_MASK,
            "Mod1" | "Alt" => mask |= MOD1_MASK,
            "Mod4" | "Super" => mask |= MOD4_MASK,
            "" => {}
            other => eprintln!("srwm: unknown modifier: {}", other),
        }
    }
    mask
}

fn string_to_keysym(name: &str) -> Result<u64, std::ffi::NulError> {
    let cname = std::ffi::CString::new(name)?;
    Ok(unsafe { crate::ffi::srwm_string_to_keysym(cname.as_ptr()) as u64 })
}
