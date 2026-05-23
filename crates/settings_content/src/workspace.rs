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

/// Controls how semantic tokens from language servers are used for syntax highlighting.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
    strum::EnumMessage,
)]
#[serde(rename_all = "snake_case")]
pub enum SemanticTokens {
    /// Do not request semantic tokens from language servers.
    #[default]
    Off,
    /// Use LSP semantic tokens together with tree-sitter highlighting.
    Combined,
    /// Use LSP semantic tokens exclusively, replacing tree-sitter highlighting.
    Full,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum DocumentFoldingRanges {
    /// Do not request folding ranges from language servers; use tree-sitter and indent-based folding.
    #[default]
    Off,
    /// Use LSP folding wherever possible, falling back to tree-sitter and indent-based folding when no results were returned by the server.
    On,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "snake_case")]
pub enum DocumentSymbols {
    /// Use tree-sitter queries to compute document symbols for outlines and breadcrumbs (default).
    #[default]
    #[serde(alias = "tree_sitter")]
    Off,
    /// Use the language server's `textDocument/documentSymbol` LSP response for outlines and
    /// breadcrumbs. When enabled, tree-sitter is not used for document symbols.
    #[serde(alias = "language_server")]
    On,
}
