#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::cell::RefCell;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

thread_local! {
    static LUA_VM: RefCell<Option<*const mlua::Lua>> = RefCell::new(None);
}

pub fn set_lua_vm(lua: &mlua::Lua) {
    LUA_VM.with(|vm| {
        *vm.borrow_mut() = Some(lua as *const mlua::Lua);
    });
}

pub fn clear_lua_vm() {
    LUA_VM.with(|vm| {
        *vm.borrow_mut() = None;
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn srwm_handle_key(id: std::ffi::c_int) {
    LUA_VM.with(|lua_vm| {
        if let Some(lua_ptr) = *lua_vm.borrow() {
            let lua = unsafe { &*lua_ptr };
            let cbs = crate::config::get_key_callbacks();
            if let Some(reg_key) = cbs.get(id as usize) {
                if let Ok(cb) = lua.registry_value::<mlua::Function>(reg_key) {
                    if let Err(e) = cb.call::<()>(()) {
                        eprintln!("srwm: key callback {} error: {}", id, e);
                    }
                }
            }
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn srwm_handle_mouse(id: std::ffi::c_int) {
    LUA_VM.with(|lua_vm| {
        if let Some(lua_ptr) = *lua_vm.borrow() {
            let lua = unsafe { &*lua_ptr };
            let cbs = crate::config::get_mouse_callbacks();
            if let Some(reg_key) = cbs.get(id as usize) {
                if let Ok(cb) = lua.registry_value::<mlua::Function>(reg_key) {
                    if let Err(e) = cb.call::<()>(()) {
                        eprintln!("srwm: mouse callback {} error: {}", id, e);
                    }
                }
            }
        }
    });
}
