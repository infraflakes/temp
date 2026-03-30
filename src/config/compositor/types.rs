use std::cell::RefCell;

#[derive(Clone)]
pub struct ShadowConfig {
    pub enable: bool,
    pub radius: i32,
    pub opacity: f64,
    pub offset_x: i32,
    pub offset_y: i32,
    pub color: String,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            enable: false,
            radius: 12,
            opacity: 0.75,
            offset_x: -15,
            offset_y: -15,
            color: "#000000".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct FadeConfig {
    pub enable: bool,
    pub step_in: f64,
    pub step_out: f64,
    pub delta: i32,
}

impl Default for FadeConfig {
    fn default() -> Self {
        Self {
            enable: false,
            step_in: 0.03,
            step_out: 0.03,
            delta: 10,
        }
    }
}

#[derive(Clone)]
pub struct BlurConfig {
    pub method: String,
    pub strength: i32,
}

impl Default for BlurConfig {
    fn default() -> Self {
        Self {
            method: "none".to_string(),
            strength: 0,
        }
    }
}

#[derive(Clone)]
pub struct AnimationConfig {
    pub preset: String,
    pub duration: f64,
    pub scale: f64,
    pub curve: String,
    pub direction: String,
    pub opacity_curve: String,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            preset: "none".to_string(),
            duration: 0.0,
            scale: 1.0,
            curve: "linear".to_string(),
            direction: "none".to_string(),
            opacity_curve: "linear".to_string(),
        }
    }
}

#[derive(Default, Clone)]
pub struct RuleConfig {
    pub match_cond: String,
    pub corner_radius: Option<i32>,
    pub shadow: Option<bool>,
    pub fade: Option<bool>,
    pub blur: Option<bool>,
    pub animate_open: Option<AnimationConfig>,
    pub animate_close: Option<AnimationConfig>,
}

#[derive(Default)]
pub struct CompositorConfig {
    pub vsync: bool,
    pub shadow: ShadowConfig,
    pub fade: FadeConfig,
    pub corner_radius: i32,
    pub blur: BlurConfig,
    pub animations: std::collections::BTreeMap<String, AnimationConfig>,
    pub rules: Vec<RuleConfig>,
}

impl CompositorConfig {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

thread_local! {
    static COMP_CONFIG: RefCell<CompositorConfig> = RefCell::new(CompositorConfig::default());
}

pub fn with_config<F, R>(f: F) -> R
where
    F: FnOnce(&RefCell<CompositorConfig>) -> R,
{
    COMP_CONFIG.with(f)
}
