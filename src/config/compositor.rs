use mlua::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

static COMPOSITOR_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let compositor = lua.create_table()?;
    let enabled_fn = lua.create_function(|_, enabled: Option<bool>| {
        COMPOSITOR_ENABLED.store(enabled.unwrap_or(true), Ordering::SeqCst);
        Ok(())
    })?;
    compositor.set("enable", enabled_fn)?;
    srwm.set("compositor", compositor)?;
    Ok(())
}

pub fn is_enabled() -> bool {
    COMPOSITOR_ENABLED.load(Ordering::SeqCst)
}

pub fn reset() {
    COMPOSITOR_ENABLED.store(false, Ordering::SeqCst);
}
