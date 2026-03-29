use mlua::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

static COMPOSITOR_ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(Default, Clone)]
struct ShadowConfig {
    enable: bool,
    radius: i32,
    opacity: f64,
    offset_x: i32,
    offset_y: i32,
    color: String,
}

#[derive(Default, Clone)]
struct FadeConfig {
    enable: bool,
    step_in: f64,
    step_out: f64,
    delta: i32,
}

#[derive(Default, Clone)]
struct BlurConfig {
    method: String,
    strength: i32,
}

#[derive(Default, Clone)]
struct AnimationConfig {
    preset: String,
    duration: f64,
    scale: f64,
    curve: String,
    direction: String,
}

#[derive(Default, Clone)]
struct RuleConfig {
    match_cond: String,
    corner_radius: Option<i32>,
    shadow: Option<bool>,
    fade: Option<bool>,
    blur: Option<bool>,
    animate_open: Option<AnimationConfig>,
    animate_close: Option<AnimationConfig>,
}

#[derive(Default)]
struct CompositorConfig {
    vsync: bool,
    shadow: ShadowConfig,
    fade: FadeConfig,
    corner_radius: i32,
    blur: BlurConfig,
    animations: HashMap<String, AnimationConfig>,
    rules: Vec<RuleConfig>,
}

thread_local! {
    static COMP_CONFIG: RefCell<CompositorConfig> = RefCell::new(CompositorConfig::default());
}

pub fn register(lua: &Lua, srwm: &LuaTable) -> LuaResult<()> {
    let compositor = lua.create_table()?;

    let enable_fn = lua.create_function(|_, enabled: Option<bool>| {
        COMPOSITOR_ENABLED.store(enabled.unwrap_or(true), Ordering::SeqCst);
        Ok(())
    })?;
    compositor.set("enable", enable_fn)?;

    let vsync_fn = lua.create_function(|_, enabled: bool| {
        COMP_CONFIG.with(|cfg| {
            cfg.borrow_mut().vsync = enabled;
        });
        Ok(())
    })?;
    compositor.set("vsync", vsync_fn)?;

    let shadow_fn = lua.create_function(|_, opts: LuaTable| {
        COMP_CONFIG.with(|cfg| {
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
        COMP_CONFIG.with(|cfg| {
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
        COMP_CONFIG.with(|cfg| {
            cfg.borrow_mut().corner_radius = radius as i32;
        });
        Ok(())
    })?;
    compositor.set("corner_radius", corner_radius_fn)?;

    let blur_fn = lua.create_function(|_, opts: LuaTable| {
        COMP_CONFIG.with(|cfg| {
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
        COMP_CONFIG.with(|cfg| {
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
            anim.direction = opts
                .get::<String>("direction")
                .unwrap_or_else(|_| "none".to_string());
            c.animations.insert(trigger, anim);
        });
        Ok(())
    })?;
    compositor.set("animate", animate_fn)?;

    let rule_fn = lua.create_function(|_, (match_cond, opts): (String, LuaTable)| {
        COMP_CONFIG.with(|cfg| {
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
    COMP_CONFIG.with(|cfg| {
        *cfg.borrow_mut() = CompositorConfig::default();
    });
}

pub fn generate_config() -> String {
    COMP_CONFIG.with(|cfg| {
        let c = cfg.borrow();
        let mut out = String::new();

        out.push_str("backend = \"egl\";\n");
        out.push_str(&format!("detect-client-opacity = true;\n"));
        out.push_str(&format!("use-damage = true;\n"));
        out.push_str(&format!("vsync = {};\n", c.vsync));

        out.push_str(&format!("shadow = {};\n", c.shadow.enable));
        out.push_str(&format!("shadow-radius = {};\n", c.shadow.radius));
        out.push_str(&format!("shadow-opacity = {};\n", c.shadow.opacity));
        out.push_str(&format!("shadow-offset-x = {};\n", c.shadow.offset_x));
        out.push_str(&format!("shadow-offset-y = {};\n", c.shadow.offset_y));
        out.push_str(&format!("shadow-color = \"{}\"\n", c.shadow.color));

        out.push_str(&format!("fading = {};\n", c.fade.enable));
        out.push_str(&format!("fade-in-step = {};\n", c.fade.step_in));
        out.push_str(&format!("fade-out-step = {};\n", c.fade.step_out));
        out.push_str(&format!("fade-delta = {};\n", c.fade.delta));

        out.push_str(&format!("corner-radius = {}\n", c.corner_radius));

        if c.blur.method != "none" {
            out.push_str("blur-background = true;\n");
            out.push_str("blur: {\n");
            out.push_str(&format!("  method = \"{}\";\n", c.blur.method));
            out.push_str(&format!("  strength = {};\n", c.blur.strength));
            out.push_str("  background = true;\n");
            out.push_str("  background-fixed = true;\n");
            out.push_str("}\n");
        }

        if !c.animations.is_empty() || !c.rules.is_empty() {
            out.push_str("rules: (\n");

            if !c.animations.is_empty() {
                out.push_str("  {\n    match = \"window_type = 'normal'\";\n    animations = (\n");
                for (trigger, anim) in &c.animations {
                    if trigger == "geometry" {
                        out.push_str(&format!(
                            "      {{\n        triggers = [\"{}\"];\n        scale-x = {{\n          curve = \"{}\";\n          duration = {};\n          start = \"window-width-before / window-width\";\n          end = 1;\n        }};\n        scale-x-reverse = {{\n          curve = \"{}\";\n          duration = {};\n          start = \"window-width / window-width-before\";\n          end = 1;\n        }};\n        scale-y = {{\n          curve = \"{}\";\n          duration = {};\n          start = \"window-height-before / window-height\";\n          end = 1;\n        }};\n        scale-y-reverse = {{\n          curve = \"{}\";\n          duration = {};\n          start = \"window-height / window-height-before\";\n          end = 1;\n        }};\n        offset-x = {{\n          curve = \"{}\";\n          duration = {};\n          start = \"window-x-before - window-x\";\n          end = 0;\n        }};\n        offset-y = {{\n          curve = \"{}\";\n          duration = {};\n          start = \"window-y-before - window-y\";\n          end = 0;\n        }};\n        shadow-scale-x = \"scale-x\";\n        shadow-scale-y = \"scale-y\";\n        shadow-offset-x = \"offset-x\";\n        shadow-offset-y = \"offset-y\";\n      }},\n",
                            trigger, anim.curve, anim.duration,
                            anim.curve, anim.duration,
                            anim.curve, anim.duration,
                            anim.curve, anim.duration,
                            anim.curve, anim.duration,
                            anim.curve, anim.duration
                        ));
                    } else {
                        let (start_opacity, end_opacity, start_scale, end_scale) = if trigger == "open" {
                            ("0.5", "1", "0.5", "1")
                        } else {
                            ("1", "0", "1", "0.9")
                        };
                        out.push_str(&format!(
                            "      {{\n        triggers = [\"{}\"];\n        opacity = {{\n          curve = \"{}\";\n          duration = {};\n          start = {};\n          end = {};\n        }};\n        blur-opacity = \"opacity\";\n        shadow-opacity = \"opacity\";\n\n        scale-x = {{\n          curve = \"{}\";\n          duration = {};\n          start = {};\n          end = {};\n        }};\n        scale-y = \"scale-x\";\n\n        offset-x = \"(1 - scale-x) / 2 * window-width\";\n        offset-y = \"(1 - scale-y) / 2 * window-height\";\n\n        shadow-scale-x = \"scale-x\";\n        shadow-scale-y = \"scale-y\";\n        shadow-offset-x = \"offset-x\";\n        shadow-offset-y = \"offset-y\";\n      }},\n",
                            trigger, anim.curve, anim.duration, start_opacity, end_opacity,
                            anim.curve, anim.duration, start_scale, end_scale
                        ));
                    }
                }
                out.push_str("    );\n  },\n");
            }
            for rule in &c.rules {
                out.push_str(&format!("  {{\n    match = \"{}\";\n", rule.match_cond));
                if let Some(r) = rule.corner_radius {
                    out.push_str(&format!("    corner-radius = {};\n", r));
                }
                if let Some(s) = rule.shadow {
                    out.push_str(&format!("    shadow = {};\n", s));
                }
                if let Some(f) = rule.fade {
                    out.push_str(&format!("    fade = {};\n", f));
                }
                if let Some(b) = rule.blur {
                    out.push_str(&format!("    blur-background = {};\n", b));
                }

                if rule.animate_open.is_some() || rule.animate_close.is_some() {
                    out.push_str("    animations = (\n");
                    if let Some(ref a) = rule.animate_open {
                        out.push_str(&format!(
                            "      {{\n        triggers = [\"open\"];\n        preset = \"{}\";\n        direction = \"{}\";\n        duration = {};\n        scale = {};\n      }},\n",
                            a.preset, a.direction, a.duration, a.scale
                        ));
                    }
                    if let Some(ref a) = rule.animate_close {
                        out.push_str(&format!(
                            "      {{\n        triggers = [\"close\"];\n        preset = \"{}\";\n        direction = \"{}\";\n        duration = {};\n        scale = {};\n      }},\n",
                            a.preset, a.direction, a.duration, a.scale
                        ));
                    }
                    out.push_str("    );\n");
                }

                out.push_str("  },\n");
            }
            out.push_str(")\n");
        }

        out
    })
}
