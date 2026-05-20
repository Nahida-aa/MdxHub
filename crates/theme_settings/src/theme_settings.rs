//! Theme settings initialization.
//!
//! Loads the bundled `aa.json` theme set and configures `gpui_component::Theme`
//! with both dark and light variants.
//!
//! # System theme detection on Linux
//!
//! `cx.window_appearance()` / `window.appearance()` always returns
//! `WindowAppearance::Light` on Linux in both the community `gpui 0.2` crate
//! and Zed's own `gpui_linux` crate — neither implementation queries the
//! xdg-desktop-portal for the color scheme during initialization. As a result
//! no reliable system-theme detection is available platform-wide, and Zed
//! exhibits the same limitation on Linux.
//!
//! We default to [`ThemeMode::Dark`] here, matching Zed's own
//! [`SystemAppearance`](https://github.com/zed-industries/zed/blob/1a7db1e0/crates/theme/src/theme.rs#L142-L146)
//! fallback. Users can toggle via the menu or key bindings at runtime.

use gpui::App;
use gpui_component::{Theme, ThemeMode, ThemeSet};
use std::rc::Rc;

pub fn init(cx: &mut App) {
    let theme_json = include_str!("../../../themes/aa/aa.json");
    let theme_set: ThemeSet =
        serde_json::from_str(theme_json).expect("Failed to parse themes/aa/aa.json");

    for t in theme_set.themes {
        let rc = Rc::new(t);
        let theme = Theme::global_mut(cx);
        match rc.mode {
            ThemeMode::Dark => theme.dark_theme = rc,
            ThemeMode::Light => theme.light_theme = rc,
        }
    }

    Theme::change(ThemeMode::Dark, None, cx);
}
