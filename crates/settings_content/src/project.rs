use gpui::Rgba;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

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
