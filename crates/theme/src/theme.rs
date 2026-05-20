mod default_colors;
pub mod schema;
pub mod styles;

pub use schema::*;
pub use styles::*;

use std::sync::Arc;

use gpui::{App, Global, SharedString, WindowAppearance};

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

pub(crate) fn default_theme() -> Theme {
    Theme {
        id: "aa-dark".into(),
        name: "Aa Dark".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            window_background_appearance: WindowBackgroundAppearance::Opaque,
            system: SystemColors::default(),
            colors: ThemeColors::dark(),
            accents: AccentColors::empty(),
            status: StatusColors::dark(),
            player: PlayerColors::empty(),
            syntax: SyntaxTheme::default_dark(),
        },
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

pub fn init(cx: &mut App) {
    cx.set_global(GlobalTheme {
        theme: Arc::new(default_theme()),
    });
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<GlobalTheme>().theme
    }
}
