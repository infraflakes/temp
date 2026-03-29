use mlua::prelude::*;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let key_table = lua.create_table()?;

    lua.globals().set("Mod1", 8u32)?;
    lua.globals().set("Mod4", 64u32)?;
    lua.globals().set("Shift", 1u32)?;
    lua.globals().set("Ctrl", 4u32)?;

    let bind_fn = lua.create_function(|lua, (mods, key, cb): (String, String, LuaFunction)| {
        let modmask = parse_modifiers(&mods);
        let keysym = string_to_keysym(&key);
        if keysym == 0 {
            return Err(mlua::Error::runtime(format!("unknown key: {}", key)));
        }

        let reg_key = lua.create_registry_value(cb)?;
        let mut cbs = crate::config::KEY_CALLBACKS.lock().unwrap();
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
            "Shift" => mask |= 1,
            "Ctrl" | "Control" => mask |= 4,
            "Mod1" | "Alt" => mask |= 8,
            "Mod4" | "Super" => mask |= 64,
            "" => {}
            other => eprintln!("srwm: unknown modifier: {}", other),
        }
    }
    mask
}

fn string_to_keysym(name: &str) -> u64 {
    let cname = std::ffi::CString::new(name).unwrap();
    unsafe { crate::ffi::srwm_string_to_keysym(cname.as_ptr()) as u64 }
}
