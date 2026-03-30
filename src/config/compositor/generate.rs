use super::types::{AnimationConfig, CompositorConfig};

pub fn generate(c: &CompositorConfig) -> String {
    let mut out = String::new();

    out.push_str("backend = \"egl\";\n");
    out.push_str("detect-client-opacity = true;\n");
    out.push_str("use-damage = true;\n");
    out.push_str(&format!("vsync = {};\n", c.vsync));

    out.push_str(&format!("shadow = {};\n", c.shadow.enable));
    out.push_str(&format!("shadow-radius = {};\n", c.shadow.radius));
    out.push_str(&format!("shadow-opacity = {};\n", c.shadow.opacity));
    out.push_str(&format!("shadow-offset-x = {};\n", c.shadow.offset_x));
    out.push_str(&format!("shadow-offset-y = {};\n", c.shadow.offset_y));
    out.push_str(&format!("shadow-color = \"{}\";\n", c.shadow.color));

    out.push_str(&format!("fading = {};\n", c.fade.enable));
    out.push_str(&format!("fade-in-step = {};\n", c.fade.step_in));
    out.push_str(&format!("fade-out-step = {};\n", c.fade.step_out));
    out.push_str(&format!("fade-delta = {};\n", c.fade.delta));

    out.push_str(&format!("corner-radius = {};\n", c.corner_radius));

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
                    let (start_opacity, end_opacity, start_scale, end_scale) = if trigger == "open"
                    {
                        ("0.5", "1", "0.5", "1")
                    } else {
                        ("1", "0", "1", "0.9")
                    };
                    out.push_str(&format!(
                        "      {{\n        triggers = [\"{}\"];\n        opacity = {{\n          curve = \"{}\";\n          duration = {};\n          start = {};\n          end = {};\n        }};\n        blur-opacity = \"opacity\";\n        shadow-opacity = \"opacity\";\n\n        scale-x = {{\n          curve = \"{}\";\n          duration = {};\n          start = {};\n          end = {};\n        }};\n        scale-y = \"scale-x\";\n\n        offset-x = \"(1 - scale-x) / 2 * window-width\";\n        offset-y = \"(1 - scale-y) / 2 * window-height\";\n\n        shadow-scale-x = \"scale-x\";\n        shadow-scale-y = \"scale-y\";\n        shadow-offset-x = \"offset-x\";\n        shadow-offset-y = \"offset-y\";\n      }},\n",
                        trigger, anim.opacity_curve, anim.duration, start_opacity, end_opacity,
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
                        "      {{\n        triggers = [\"open\", \"show\"];\n        preset = \"{}\";\n        direction = \"{}\";\n        duration = {};\n        scale = {};\n      }},\n",
                        a.preset, a.direction, a.duration, a.scale
                    ));
                }
                if let Some(ref a) = rule.animate_close {
                    out.push_str(&format!(
                        "      {{\n        triggers = [\"close\", \"hide\"];\n        preset = \"{}\";\n        direction = \"{}\";\n        duration = {};\n        scale = {};\n      }},\n",
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
}
