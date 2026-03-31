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
                if v < 0 {
                    eprintln!("srwm: warning: shadow.radius must be >= 0, got {}", v);
                } else {
                    c.shadow.radius = v as i32;
                }
            }
            if let Ok(v) = opts.get::<f64>("opacity") {
                if v < 0.0 || v > 1.0 {
                    eprintln!("srwm: warning: shadow.opacity must be 0.0..1.0, got {}", v);
                } else {
                    c.shadow.opacity = v;
                }
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
                if v < 0.0 || v > 1.0 {
                    eprintln!("srwm: warning: fade.step_in must be 0.0..1.0, got {}", v);
                } else {
                    c.fade.step_in = v;
                }
            }
            if let Ok(v) = opts.get::<f64>("step_out") {
                if v < 0.0 || v > 1.0 {
                    eprintln!("srwm: warning: fade.step_out must be 0.0..1.0, got {}", v);
                } else {
                    c.fade.step_out = v;
                }
            }
            if let Ok(v) = opts.get::<i64>("delta") {
                if v <= 0 {
                    eprintln!("srwm: warning: fade.delta must be > 0, got {}", v);
                } else {
                    c.fade.delta = v as i32;
                }
            }
        });
        Ok(())
    })?;
    compositor.set("fade", fade_fn)?;

    let corner_radius_fn = lua.create_function(|_, radius: i64| {
        if radius < 0 {
            return Err(mlua::Error::runtime(format!(
                "corner_radius must be >= 0, got {}",
                radius
            )));
        }
        types::with_config(|cfg| {
            cfg.borrow_mut().corner_radius = radius as i32;
        });
        Ok(())
    })?;
    compositor.set("corner_radius", corner_radius_fn)?;

    let blur_fn = lua.create_function(|_, opts: LuaTable| {
        let method = opts
            .get::<String>("method")
            .unwrap_or_else(|_| "dual_kawase".to_string());
        let valid_methods = ["none", "dual_kawase", "gaussian", "box", "kernel"];
        if !valid_methods.contains(&method.as_str()) {
            eprintln!(
                "srwm: warning: unknown blur method '{}', expected one of {:?}",
                method, valid_methods
            );
        }
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            c.blur.method = method;
            if let Ok(v) = opts.get::<i64>("strength") {
                if v < 0 {
                    eprintln!("srwm: warning: blur.strength must be >= 0, got {}", v);
                } else {
                    c.blur.strength = v as i32;
                }
            }
        });
        Ok(())
    })?;
    compositor.set("blur", blur_fn)?;

    let border_blur_fn = lua.create_function(|_, opts: LuaTable| {
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            if let Ok(v) = opts.get::<bool>("enable") {
                c.border_blur.enable = v;
            }
            if let Ok(v) = opts.get::<f64>("dim") {
                if !(0.0..=1.0).contains(&v) {
                    eprintln!("srwm: warning: border_blur.dim must be 0.0..1.0, got {}", v);
                } else {
                    c.border_blur.dim = v;
                }
            }
        });
        Ok(())
    })?;
    compositor.set("border_blur", border_blur_fn)?;

    let animate_fn = lua.create_function(|_, (trigger, opts): (String, LuaTable)| {
        let valid_triggers = ["open", "close", "geometry"];
        if !valid_triggers.contains(&trigger.as_str()) {
            return Err(mlua::Error::runtime(format!(
                "invalid animation trigger '{}': expected one of {:?}",
                trigger, valid_triggers
            )));
        }
        types::with_config(|cfg| {
            let mut c = cfg.borrow_mut();
            let mut anim = AnimationConfig::default();
            anim.preset = opts
                .get::<String>("preset")
                .unwrap_or_else(|_| "none".to_string());
            if let Ok(v) = opts.get::<f64>("duration") {
                if v < 0.0 {
                    eprintln!("srwm: warning: animate duration must be >= 0, got {}", v);
                } else {
                    anim.duration = v;
                }
            }
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
