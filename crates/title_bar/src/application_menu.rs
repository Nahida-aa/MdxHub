use crate::title_bar_settings::TitleBarSettings;
use gpui::{actions, Action, App, Entity, OwnedMenu, OwnedMenuItem};

pub struct ApplicationMenu {
    entries: SmallVec<[MenuEntry; 8]>,
    pending_menu_open: Option<String>,
}

pub(crate) fn show_menus(cx: &mut App) -> bool {
    TitleBarSettings::get_global(cx).show_menus
        && (cfg!(not(target_os = "macos")) || option_env!("ZED_USE_CROSS_PLATFORM_MENU").is_some())
}
