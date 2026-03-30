pub mod generate;
pub mod types;

use mlua::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

use types::{AnimationConfig, RuleConfig};

static COMPOSITOR_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let compositor = lua.create_table()?;

    let enable_fn = lua.create_function(|_, enabled: Option<bool>| {
        COMPOSITOR_ENABLED.store(enabled.unwrap_or(true), Ordering::SeqCst);
        Ok(())
    })?;
    compositor.set("enable", enable_fn)?;

    let vsync_fn = lua.create_function(|_, enabled: bool| {
        types::with_config(|cfg| {
            cfg.borrow_mut().vsync = enabled;
        });
        Ok(())
    })?;
    compositor.set("vsync", vsync_fn)?;

    let shadow_fn = lua.create_function(|_, opts: LuaTable| {
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            if let Ok(v) = opts.get::<bool>("enable") {
                c.shadow.enable = v;
            }
            if let Ok(v) = opts.get::<i64>("radius") {
                c.shadow.radius = v as i32;
            }
            if let Ok(v) = opts.get::<f64>("opacity") {
                c.shadow.opacity = v;
            }
            if let Ok(v) = opts.get::<i64>("offset_x") {
                c.shadow.offset_x = v as i32;
            }
            if let Ok(v) = opts.get::<i64>("offset_y") {
                c.shadow.offset_y = v as i32;
            }
            if let Ok(v) = opts.get::<String>("color") {
                c.shadow.color = v;
            }
        });
        Ok(())
    })?;
    compositor.set("shadow", shadow_fn)?;

    let fade_fn = lua.create_function(|_, opts: LuaTable| {
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            if let Ok(v) = opts.get::<bool>("enable") {
                c.fade.enable = v;
            }
            if let Ok(v) = opts.get::<f64>("step_in") {
                c.fade.step_in = v;
            }
            if let Ok(v) = opts.get::<f64>("step_out") {
                c.fade.step_out = v;
            }
            if let Ok(v) = opts.get::<i64>("delta") {
                c.fade.delta = v as i32;
            }
        });
        Ok(())
    })?;
    compositor.set("fade", fade_fn)?;

    let corner_radius_fn = lua.create_function(|_, radius: i64| {
        types::with_config(|cfg| {
            cfg.borrow_mut().corner_radius = radius as i32;
        });
        Ok(())
    })?;
    compositor.set("corner_radius", corner_radius_fn)?;

    let blur_fn = lua.create_function(|_, opts: LuaTable| {
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            c.blur.method = opts
                .get::<String>("method")
                .unwrap_or_else(|_| "dual_kawase".to_string());
            if let Ok(v) = opts.get::<i64>("strength") {
                c.blur.strength = v as i32;
            }
        });
        Ok(())
    })?;
    compositor.set("blur", blur_fn)?;

    let animate_fn = lua.create_function(|_, (trigger, opts): (String, LuaTable)| {
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            let mut anim = AnimationConfig::default();
            anim.preset = opts
                .get::<String>("preset")
                .unwrap_or_else(|_| "none".to_string());
            anim.duration = opts.get::<f64>("duration").unwrap_or(0.0);
            anim.scale = opts.get::<f64>("scale").unwrap_or(1.0);
            anim.curve = opts
                .get::<String>("curve")
                .unwrap_or_else(|_| "linear".to_string());
            anim.opacity_curve = opts
                .get::<String>("opacity_curve")
                .unwrap_or_else(|_| anim.curve.clone());
            anim.direction = opts
                .get::<String>("direction")
                .unwrap_or_else(|_| "none".to_string());
            c.animations.insert(trigger, anim);
        });
        Ok(())
    })?;
    compositor.set("animate", animate_fn)?;

    let rule_fn = lua.create_function(|_, (match_cond, opts): (String, LuaTable)| {
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            let mut rule = RuleConfig {
                match_cond,
                ..Default::default()
            };
            if let Ok(v) = opts.get::<i64>("corner_radius") {
                rule.corner_radius = Some(v as i32);
            }
            if let Ok(v) = opts.get::<bool>("shadow") {
                rule.shadow = Some(v);
            }
            if let Ok(v) = opts.get::<bool>("fade") {
                rule.fade = Some(v);
            }
            if let Ok(v) = opts.get::<bool>("blur") {
                rule.blur = Some(v);
            }
            if let Ok(v) = opts.get::<LuaTable>("animate_open") {
                let mut anim = AnimationConfig::default();
                anim.preset = v
                    .get::<String>("preset")
                    .unwrap_or_else(|_| "slide-in".to_string());
                anim.direction = v
                    .get::<String>("direction")
                    .unwrap_or_else(|_| "none".to_string());
                anim.duration = v.get::<f64>("duration").unwrap_or(0.2);
                anim.scale = v.get::<f64>("scale").unwrap_or(0.5);
                rule.animate_open = Some(anim);
            }
            if let Ok(v) = opts.get::<LuaTable>("animate_close") {
                let mut anim = AnimationConfig::default();
                anim.preset = v
                    .get::<String>("preset")
                    .unwrap_or_else(|_| "slide-out".to_string());
                anim.direction = v
                    .get::<String>("direction")
                    .unwrap_or_else(|_| "none".to_string());
                anim.duration = v.get::<f64>("duration").unwrap_or(0.1);
                anim.scale = v.get::<f64>("scale").unwrap_or(0.5);
                rule.animate_close = Some(anim);
            }
            c.rules.push(rule);
        });
        Ok(())
    })?;
    compositor.set("rule", rule_fn)?;

    srwm.set("compositor", compositor)?;
    Ok(())
}

pub fn is_enabled() -> bool {
    COMPOSITOR_ENABLED.load(Ordering::SeqCst)
}

pub fn reset() {
    COMPOSITOR_ENABLED.store(false, Ordering::SeqCst);
    types::with_config(|cfg| {
        cfg.borrow_mut().reset();
    });
}

pub fn generate_config() -> String {
    types::with_config(|cfg| generate::generate(&cfg.borrow()))
}
