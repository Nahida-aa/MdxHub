use gpui::*;
use gpui_component::{Theme, ThemeConfig, ThemeMode};
use std::rc::Rc;

pub fn init(cx: &mut App) {
    let theme_json = include_str!("../../../themes/aa/aa.json");
    let theme_set: gpui_component::ThemeSet =
        serde_json::from_str(theme_json).expect("Failed to parse themes/aa/aa.json");

    let mut dark_config: Option<Rc<ThemeConfig>> = None;
    let mut light_config: Option<Rc<ThemeConfig>> = None;

    for t in theme_set.themes {
        let rc = Rc::new(t);
        match rc.mode {
            ThemeMode::Dark => dark_config = Some(rc),
            ThemeMode::Light => light_config = Some(rc),
        }
    }

    let theme = Theme::global_mut(cx);
    if let Some(dark) = dark_config {
        theme.dark_theme = dark;
    }
    if let Some(light) = light_config {
        theme.light_theme = light;
    }

    let mode = theme.mode;
    if mode.is_dark() {
        theme.apply_config(&theme.dark_theme.clone());
    } else {
        theme.apply_config(&theme.light_theme.clone());
    }
}
