use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WorkspaceSettingsContent {
    /// Whether to allow windows to tab together based on the user’s tabbing preference (macOS only).
    ///
    /// Default: false
    pub use_system_window_tabs: Option<bool>,
}
