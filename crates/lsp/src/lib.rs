use gpui::{App, AppContext as _, AsyncApp, BackgroundExecutor, SharedString, Task};
pub use lsp_types::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
/// A name of a language server.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, JsonSchema,
)]
#[serde(transparent)]
pub struct LanguageServerName(pub SharedString);
