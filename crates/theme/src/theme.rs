use std::sync::Arc;

use gpui::{App, Global, SharedString, WindowAppearance};

use crate::styles::{ThemeColors, ThemeStyles};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    Light,
    Dark,
}

impl Appearance {
    pub fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }
}

impl From<WindowAppearance> for Appearance {
    fn from(value: WindowAppearance) -> Self {
        match value {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub id: String,
    pub name: SharedString,
    pub appearance: Appearance,
    pub styles: ThemeStyles,
}

impl Theme {
    pub fn colors(&self) -> &ThemeColors {
        &self.styles.colors
    }
}

#[derive(Debug, Clone)]
pub struct GlobalTheme {
    theme: Arc<Theme>,
}

impl Global for GlobalTheme {}

impl GlobalTheme {
    pub fn update_theme(cx: &mut App, theme: Arc<Theme>) {
        cx.set_global(Self { theme });
    }

    pub fn theme(cx: &App) -> Arc<Theme> {
        cx.global::<Self>().theme.clone()
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<GlobalTheme>().theme
    }
}
