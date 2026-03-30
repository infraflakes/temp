use mlua::prelude::*;

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    srwm.set(
        "spawn",
        lua.create_function(|_, cmd: String| {
            std::thread::spawn(move || {
                match std::process::Command::new("sh").arg("-c").arg(&cmd).spawn() {
                    Ok(_) => {}
                    Err(e) => eprintln!("srwm: spawn '{}' failed: {}", cmd, e),
                }
            });
            Ok(())
        })?,
    )?;

    let _ = srwm.set(
        "spawn_once",
        lua.create_function(|_, cmd: String| {
            let mut spawned = crate::config::SPAWNED_ONCE.lock().unwrap();
            if spawned.contains(&cmd) {
                return Ok(());
            }
            spawned.push_back(cmd.clone());
            drop(spawned);

            std::thread::spawn(move || {
                match std::process::Command::new("sh").arg("-c").arg(&cmd).spawn() {
                    Ok(_) => {}
                    Err(e) => eprintln!("srwm: spawn '{}' failed: {}", cmd, e),
                }
            });
            Ok(())
        })?,
    );
    Ok(())
}
