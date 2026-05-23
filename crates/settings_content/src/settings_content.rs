mod agent;
mod editor;
mod fallible_options;
pub mod merge_from;
mod project;
pub use agent::*;
pub use editor::*;
mod title_bar;
mod workspace;
pub use fallible_options::*;
pub use merge_from::MergeFrom as MergeFromTrait;
pub use project::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};
pub use title_bar::*;
pub use workspace::*;
mod language;
pub use language::*;
mod terminal;
pub use terminal::*;
mod theme;
pub use theme::*;
mod serde_helper;
pub use serde_helper::{
    serialize_f32_with_two_decimal_places, serialize_optional_f32_with_two_decimal_places,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseStatus {
    /// Settings were parsed successfully
    Success,
    /// Settings file was not changed, so no parsing was performed
    Unchanged,
    /// Settings failed to parse
    Failed { error: String },
}

// #[with_fallible_options]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct SettingsContent {
    #[serde(flatten)]
    pub project: ProjectSettingsContent,

    //     #[serde(flatten)]
    //     pub theme: Box<ThemeSettingsContent>,

    //     #[serde(flatten)]
    //     pub extension: ExtensionSettingsContent,
    #[serde(flatten)]
    pub workspace: WorkspaceSettingsContent,

    //     #[serde(flatten)]
    //     pub editor: EditorSettingsContent,

    //     #[serde(flatten)]
    //     pub remote: RemoteSettingsContent,

    //     /// Settings related to the file finder.
    //     pub file_finder: Option<FileFinderSettingsContent>,

    //     pub git_panel: Option<GitPanelSettingsContent>,

    //     pub tabs: Option<ItemSettingsContent>,
    //     pub tab_bar: Option<TabBarSettingsContent>,
    //     pub status_bar: Option<StatusBarSettingsContent>,

    //     pub preview_tabs: Option<PreviewTabsSettingsContent>,

    //     pub agent: Option<AgentSettingsContent>,
    //     pub agent_servers: Option<AllAgentServersSettings>,

    //     /// Configuration of audio in Zed.
    //     pub audio: Option<AudioSettingsContent>,

    //     /// Whether or not to automatically check for updates.
    //     ///
    //     /// Default: true
    //     pub auto_update: Option<bool>,

    //     /// This base keymap settings adjusts the default keybindings in Zed to be similar
    //     /// to other common code editors. By default, Zed's keymap closely follows VSCode's
    //     /// keymap, with minor adjustments, this corresponds to the "VSCode" setting.
    //     ///
    //     /// Default: VSCode
    //     pub base_keymap: Option<BaseKeymapContent>,

    //     /// Configuration for the collab panel visual settings.
    //     pub collaboration_panel: Option<PanelSettingsContent>,

    //     pub debugger: Option<DebuggerSettingsContent>,

    //     /// Configuration for Diagnostics-related features.
    //     pub diagnostics: Option<DiagnosticsSettingsContent>,

    //     /// Configuration for Git-related features
    //     pub git: Option<GitSettings>,

    //     /// Common language server settings.
    //     pub global_lsp_settings: Option<GlobalLspSettingsContent>,

    //     /// The settings for the image viewer.
    //     pub image_viewer: Option<ImageViewerSettingsContent>,

    //     pub repl: Option<ReplSettingsContent>,

    //     /// Whether or not to enable Helix mode.
    //     ///
    //     /// Default: false
    //     pub helix_mode: Option<bool>,

    //     /// Determines when the mouse cursor should be hidden in response to
    //     /// keyboard input. Applies globally across all input surfaces (editors,
    //     /// terminals, palettes, etc.).
    //     ///
    //     /// Default: on_typing_and_action
    //     pub hide_mouse: Option<HideMouseMode>,

    //     pub journal: Option<JournalSettingsContent>,

    //     /// A map of log scopes to the desired log level.
    //     /// Useful for filtering out noisy logs or enabling more verbose logging.
    //     ///
    //     /// Example: {"log": {"client": "warn"}}
    //     pub log: Option<HashMap<String, String>>,

    //     pub line_indicator_format: Option<LineIndicatorFormat>,

    //     pub language_models: Option<AllLanguageModelSettingsContent>,

    //     pub outline_panel: Option<OutlinePanelSettingsContent>,

    //     pub project_panel: Option<ProjectPanelSettingsContent>,

    //     /// Configuration for the Message Editor
    //     pub message_editor: Option<MessageEditorSettings>,

    //     /// Configuration for Node-related features
    //     pub node: Option<NodeBinarySettings>,

    //     pub proxy: Option<String>,

    //     /// The URL of the Zed server to connect to.
    //     pub server_url: Option<String>,

    //     /// The URL used as the key for credential storage.
    //     ///
    //     /// When set, credentials are stored under this URL instead of `server_url`.
    //     /// This allows running multiple Zed instances side by side without them
    //     /// overwriting each other's keychain entries.
    //     pub credentials_url: Option<String>,

    //     /// Configuration for session-related features
    //     pub session: Option<SessionSettingsContent>,
    //     /// Control what info is collected by Zed.
    //     pub telemetry: Option<TelemetrySettingsContent>,

    //     /// Configuration of the terminal in Zed.
    //     pub terminal: Option<TerminalSettingsContent>,
    pub title_bar: Option<TitleBarSettingsContent>,
    //     /// Whether or not to enable Vim mode.
    //     ///
    //     /// Default: false
    //     pub vim_mode: Option<bool>,

    //     // Settings related to calls in Zed
    //     pub calls: Option<CallSettingsContent>,

    //     /// Settings for the which-key popup.
    //     pub which_key: Option<WhichKeySettingsContent>,

    //     /// Settings related to Vim mode in Zed.
    //     pub vim: Option<VimSettingsContent>,

    //     /// Number of lines to search for modelines at the beginning and end of files.
    //     /// Modelines contain editor directives (e.g., vim/emacs settings) that configure
    //     /// the editor behavior for specific files.
    //     ///
    //     /// Default: 5
    //     pub modeline_lines: Option<usize>,

    //     /// Local overrides for feature flags, keyed by flag name.
    //     pub feature_flags: Option<FeatureFlagsMap>,

    //     /// Settings for developer-oriented instrumentation tools (profilers,
    //     /// tracers, etc.) that can be toggled at runtime.
    //     pub instrumentation: Option<InstrumentationSettingsContent>,
}

#[with_fallible_options]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, JsonSchema, MergeFrom)]
pub struct SshPortForwardOption {
    pub local_host: Option<String>,
    pub local_port: u16,
    pub remote_host: Option<String>,
    pub remote_port: u16,
}

// A SaturatingBool in the settings can only ever be set to true,
// later attempts to set it to false will be ignored.
//
// Used by `disable_ai`.
#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SaturatingBool(pub bool);

impl From<bool> for SaturatingBool {
    fn from(value: bool) -> Self {
        SaturatingBool(value)
    }
}

impl From<SaturatingBool> for bool {
    fn from(value: SaturatingBool) -> bool {
        value.0
    }
}

impl merge_from::MergeFrom for SaturatingBool {
    fn merge_from(&mut self, other: &Self) {
        self.0 |= other.0
    }
}

// An ExtendingVec in the settings can only accumulate new values.
//
// This is useful for things like private files where you only want
// to allow new values to be added.
//
// Consider using a HashMap<String, bool> instead of this type
// (like auto_install_extensions) so that user settings files can both add
// and remove values from the set.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExtendingVec<T>(pub Vec<T>);

impl<T> Into<Vec<T>> for ExtendingVec<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
impl<T> From<Vec<T>> for ExtendingVec<T> {
    fn from(vec: Vec<T>) -> Self {
        ExtendingVec(vec)
    }
}

impl<T: Clone> merge_from::MergeFrom for ExtendingVec<T> {
    fn merge_from(&mut self, other: &Self) {
        self.0.extend_from_slice(other.0.as_slice());
    }
}
