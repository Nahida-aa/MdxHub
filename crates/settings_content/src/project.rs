use collections::{BTreeMap, HashMap};
use gpui::Rgba;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, RegisterSetting, with_fallible_options};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::serde::default_true;

use crate::{
    AllLanguageSettingsContent, ExtendingVec, ProjectTerminalSettingsContent, SaturatingBool,
};

#[with_fallible_options]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct LspSettingsMap(pub HashMap<Arc<str>, LspSettings>);

#[with_fallible_options]
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ProjectSettingsContent {
    #[serde(flatten)]
    pub all_languages: AllLanguageSettingsContent,

    #[serde(flatten)]
    pub worktree: WorktreeSettingsContent,

    /// Configuration for language servers.
    ///
    /// The following settings can be overridden for specific language servers:
    /// - initialization_options
    ///
    /// To override settings for a language, add an entry for that language server's
    /// name to the lsp value.
    /// Default: null
    #[serde(default)]
    pub lsp: LspSettingsMap,

    pub terminal: Option<ProjectTerminalSettingsContent>,

    // /// Configuration for Debugger-related features
    // #[serde(default)]
    // pub dap: HashMap<Arc<str>, DapSettingsContent>,
    /// Settings for context servers used for AI-related features.
    #[serde(default)]
    pub context_servers: HashMap<Arc<str>, ContextServerSettingsContent>,

    /// Default timeout in seconds for context server tool calls.
    /// Can be overridden per-server in context_servers configuration.
    ///
    /// Default: 60
    pub context_server_timeout: Option<u64>,

    /// Configuration for how direnv configuration should be loaded
    pub load_direnv: Option<DirenvSettings>,

    /// The list of custom Git hosting providers.
    pub git_hosting_providers: Option<ExtendingVec<GitHostingProviderConfig>>,

    /// Whether to disable all AI features in Zed.
    ///
    /// Default: false
    pub disable_ai: Option<SaturatingBool>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct WorktreeSettingsContent {
    /// Whether to prevent this project from being shared in public channels.
    ///
    /// Default: false
    #[serde(default)]
    pub prevent_sharing_in_public_channels: bool,

    /// Completely ignore files matching globs from `file_scan_exclusions`. Overrides
    /// `file_scan_inclusions`.
    ///
    /// Default: [
    ///   "**/.git",
    ///   "**/.svn",
    ///   "**/.hg",
    ///   "**/.jj",
    ///   "**/CVS",
    ///   "**/.DS_Store",
    ///   "**/Thumbs.db",
    ///   "**/.classpath",
    ///   "**/.settings"
    /// ]
    pub file_scan_exclusions: Option<Vec<String>>,

    /// Always include files that match these globs when scanning for files, even if they're
    /// ignored by git. This setting is overridden by `file_scan_exclusions`.
    /// Default: [
    ///  ".env*",
    ///  "docker-compose.*.yml",
    /// ]
    pub file_scan_inclusions: Option<Vec<String>>,

    /// Treat the files matching these globs as `.env` files.
    /// Default: ["**/.env*", "**/*.pem", "**/*.key", "**/*.cert", "**/*.crt", "**/secrets.yml"]
    pub private_files: Option<ExtendingVec<String>>,

    /// Treat the files matching these globs as hidden files. You can hide hidden files in the project panel.
    /// Default: ["**/.*"]
    pub hidden_files: Option<Vec<String>>,

    /// Treat the files matching these globs as read-only. These files can be opened and viewed,
    /// but cannot be edited. This is useful for generated files, build outputs, or files from
    /// external dependencies that should not be modified directly.
    /// Default: []
    pub read_only_files: Option<Vec<String>>,
}

#[with_fallible_options]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash)]
#[serde(rename_all = "snake_case")]
pub struct LspSettings {
    pub binary: Option<BinarySettings>,
    /// Options passed to the language server at startup.
    ///
    /// Ref: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize
    ///
    /// Consult the documentation for the specific language server to see which settings are supported.
    pub initialization_options: Option<serde_json::Value>,
    /// Language server settings.
    ///
    /// Ref: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_configuration
    ///
    /// Consult the documentation for the specific language server to see which settings are supported.
    pub settings: Option<serde_json::Value>,
    /// If the server supports sending tasks over LSP extensions,
    /// this setting can be used to enable or disable them in Zed.
    /// Default: true
    #[serde(default = "default_true")]
    pub enable_lsp_tasks: bool,
    pub fetch: Option<FetchSettings>,
}

impl Default for LspSettings {
    fn default() -> Self {
        Self {
            binary: None,
            initialization_options: None,
            settings: None,
            enable_lsp_tasks: true,
            fetch: None,
        }
    }
}

#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
pub struct FetchSettings {
    // Whether to consider pre-releases for fetching
    pub pre_release: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SemanticTokenRule {
    pub token_type: Option<String>,
    #[serde(default)]
    pub token_modifiers: Vec<String>,
    #[serde(default)]
    pub style: Vec<String>,
    pub foreground_color: Option<Rgba>,
    pub background_color: Option<Rgba>,
    pub underline: Option<SemanticTokenColorOverride>,
    pub strikethrough: Option<SemanticTokenColorOverride>,
    pub font_weight: Option<SemanticTokenFontWeight>,
    pub font_style: Option<SemanticTokenFontStyle>,
}

#[with_fallible_options]
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, JsonSchema, MergeFrom, Hash,
)]
pub struct BinarySettings {
    pub path: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub env: Option<BTreeMap<String, String>>,
    pub ignore_system_version: Option<bool>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom)]
#[serde(untagged)]
pub enum SemanticTokenColorOverride {
    InheritForeground(bool),
    Replace(Rgba),
}
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum SemanticTokenFontWeight {
    #[default]
    Normal,
    Bold,
}
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum SemanticTokenFontStyle {
    #[default]
    Normal,
    Italic,
}
/// Custom rules for rendering LSP semantic tokens.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(transparent)]
pub struct SemanticTokenRules {
    pub rules: Vec<SemanticTokenRule>,
}

#[with_fallible_options]
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom)]
pub struct ContextServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
    /// Timeout for tool calls in seconds. Defaults to 60 if not specified.
    pub timeout: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom, Debug)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ContextServerSettingsContent {
    Stdio {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// Whether to run the context server on the remote server when using remote development.
        ///
        /// If this is false, the context server will always run on the local machine.
        ///
        /// Default: false
        #[serde(default)]
        remote: bool,
        #[serde(flatten)]
        command: ContextServerCommand,
    },
    Http {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// The URL of the remote context server.
        url: String,
        /// Optional headers to send.
        #[serde(skip_serializing_if = "HashMap::is_empty", default)]
        headers: HashMap<String, String>,
        /// Timeout for tool calls in seconds. Defaults to global context_server_timeout if not specified.
        timeout: Option<u64>,
        /// Pre-registered OAuth client credentials for authorization servers that
        /// require out-of-band client registration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        oauth: Option<OAuthClientSettings>,
    },
    Extension {
        /// Whether the context server is enabled.
        #[serde(default = "default_true")]
        enabled: bool,
        /// Whether to run the context server on the remote server when using remote development.
        ///
        /// If this is false, the context server will always run on the local machine.
        ///
        /// Default: false
        #[serde(default)]
        remote: bool,
        /// The settings for this context server specified by the extension.
        ///
        /// Consult the documentation for the context server to see what settings
        /// are supported.
        settings: serde_json::Value,
    },
}

/// Pre-registered OAuth client credentials for MCP servers that don't support
/// Dynamic Client Registration.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, MergeFrom, Debug)]
pub struct OAuthClientSettings {
    /// The OAuth client ID obtained from out-of-band registration with the
    /// authorization server.
    pub client_id: String,
    /// The OAuth client secret, if this is a confidential client. For security,
    /// prefer providing this interactively; we will prompt and store it in
    /// the system keychain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum DirenvSettings {
    /// Load direnv configuration through a shell hook
    ShellHook,
    /// Load direnv configuration directly using `direnv export json`
    #[default]
    Direct,
    /// Do not load direnv configuration
    Disabled,
}

impl std::fmt::Debug for ContextServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filtered_env = self.env.as_ref().map(|env| {
            env.iter()
                .map(|(k, v)| {
                    (
                        k,
                        if util::redact::should_redact(k) {
                            "[REDACTED]"
                        } else {
                            v
                        },
                    )
                })
                .collect::<Vec<_>>()
        });

        f.debug_struct("ContextServerCommand")
            .field("path", &self.path)
            .field("args", &self.args)
            .field("env", &filtered_env)
            .finish()
    }
}

/// A custom Git hosting provider.
#[with_fallible_options]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct GitHostingProviderConfig {
    /// The type of the provider.
    ///
    /// Must be one of `github`, `gitlab`, `bitbucket`, `gitea`, `forgejo`, or `source_hut`.
    pub provider: GitHostingProviderKind,

    /// The base URL for the provider (e.g., "https://code.corp.big.com").
    pub base_url: String,

    /// The display name for the provider (e.g., "BigCorp GitHub").
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum GitHostingProviderKind {
    Github,
    Gitlab,
    Bitbucket,
    Gitea,
    Forgejo,
    SourceHut,
}
