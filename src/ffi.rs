#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> d027391 (Config parsing reimplementation)
static mut LUA_VM: Option<*const mlua::Lua> = None;

pub unsafe fn set_lua_vm(lua: &mlua::Lua) {
    unsafe {
        LUA_VM = Some(lua as *const mlua::Lua);
    }
}

pub unsafe fn clear_lua_vm() {
    unsafe {
        LUA_VM = None;
    }
}

<<<<<<< HEAD
#[unsafe(no_mangle)]
pub extern "C" fn srwm_handle_key(id: std::ffi::c_int) {
    unsafe {
        if let Some(lua_ptr) = LUA_VM {
            let lua = &*lua_ptr;
            let cbs = crate::config::get_key_callbacks();
            if let Some(reg_key) = cbs.get(id as usize) {
                if let Ok(cb) = lua.registry_value::<mlua::Function>(reg_key) {
                    if let Err(e) = cb.call::<()>(()) {
                        eprintln!("srwm: key callback {} error: {}", id, e);
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn srwm_handle_mouse(id: std::ffi::c_int) {
    unsafe {
        if let Some(lua_ptr) = LUA_VM {
            let lua = &*lua_ptr;
            let cbs = crate::config::get_mouse_callbacks();
            if let Some(reg_key) = cbs.get(id as usize) {
                if let Ok(cb) = lua.registry_value::<mlua::Function>(reg_key) {
                    if let Err(e) = cb.call::<()>(()) {
                        eprintln!("srwm: mouse callback {} error: {}", id, e);
                    }
                }
            }
        }
    }
=======
// C calls these when a dynamic key/mouse binding fires
=======
>>>>>>> d027391 (Config parsing reimplementation)
#[unsafe(no_mangle)]
pub extern "C" fn srwm_handle_key(id: std::ffi::c_int) {
    unsafe {
        if let Some(lua_ptr) = LUA_VM {
            let lua = &*lua_ptr;
            let cbs = crate::config::get_key_callbacks();
            if let Some(reg_key) = cbs.get(id as usize) {
                if let Ok(cb) = lua.registry_value::<mlua::Function>(reg_key) {
                    if let Err(e) = cb.call::<()>(()) {
                        eprintln!("srwm: key callback {} error: {}", id, e);
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn srwm_handle_mouse(id: std::ffi::c_int) {
<<<<<<< HEAD
    eprintln!("srwm: mouse callback {id} (not yet implemented)");
>>>>>>> 9436516 (Scaffold)
=======
    unsafe {
        if let Some(lua_ptr) = LUA_VM {
            let lua = &*lua_ptr;
            let cbs = crate::config::get_mouse_callbacks();
            if let Some(reg_key) = cbs.get(id as usize) {
                if let Ok(cb) = lua.registry_value::<mlua::Function>(reg_key) {
                    if let Err(e) = cb.call::<()>(()) {
                        eprintln!("srwm: mouse callback {} error: {}", id, e);
                    }
                }
            }
        }
    }
>>>>>>> d027391 (Config parsing reimplementation)
}
