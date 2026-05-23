use std::time::Duration;

use serde::Deserialize;
pub use settings::{
    // ActionName, AutosaveSetting, BottomDockLayout, EncodingDisplayOptions, InactiveOpacity,
    // PaneSplitDirectionHorizontal, PaneSplitDirectionVertical,
    RegisterSetting,
    // RestoreOnStartupBehavior,
    Settings,
};

#[derive(RegisterSetting)]
pub struct WorkspaceSettings {
    pub use_system_window_tabs: bool, // — 是否使用系统（macOS）原生窗口标签页
}

impl Settings for WorkspaceSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let workspace = &content.workspace;
        Self {
            use_system_window_tabs: workspace.use_system_window_tabs.unwrap(),
        }
    }
}

#[derive(Copy, Clone, Deserialize)]
pub struct FocusFollowsMouse {
    pub enabled: bool,
    pub debounce: Duration,
}
