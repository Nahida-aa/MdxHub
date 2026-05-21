mod editorconfig_store;
mod settings_file;
pub use settings_file::*;
mod settings_store;

pub use settings_macros::RegisterSetting;

pub mod settings_content {
    pub use ::settings_content::*;
}

#[doc(hidden)]
pub mod private {
    pub use crate::settings_store::{RegisteredSetting, SettingValue};
    pub use gpui::private::inventory;
}

pub use settings_content::*;

pub use settings_store::{
    // DefaultSemanticTokenRules,
    InvalidSettingsError,
    // LocalSettingsKind,
    LocalSettingsPath,
    // MigrationStatus,
    Settings,
    //  SettingsFile, SettingsJsonSchemaParams, SettingsKey,
    // SettingsLocation, SettingsParseResult,
    SettingsStore,
    // LSP_SETTINGS_SCHEMA_URL_PREFIX,
};

pub use editorconfig_store::{
    // Editorconfig, EditorconfigEvent, EditorconfigProperties,
    EditorconfigStore,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, serde::Serialize)]
pub struct WorktreeId(usize);
