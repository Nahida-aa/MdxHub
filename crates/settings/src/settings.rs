mod settings_store;
pub use settings_macros::RegisterSetting;
pub use settings_store::{
    // DefaultSemanticTokenRules, InvalidSettingsError, LocalSettingsKind, LocalSettingsPath,
    // MigrationStatus,
    Settings,
    //  SettingsFile, SettingsJsonSchemaParams, SettingsKey,
    // SettingsLocation, SettingsParseResult, SettingsStore, LSP_SETTINGS_SCHEMA_URL_PREFIX,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, serde::Serialize)]
pub struct WorktreeId(usize);
