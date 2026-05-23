mod buffer;
use std::sync::Arc;
mod syntax_map;
pub use buffer::*;
mod manifest;
use language_core::{Grammar, LanguageConfig, LanguageId};
pub use manifest::{ManifestDelegate, ManifestName, ManifestProvider, ManifestQuery};
// use syntax_map::{QueryCursorHandle, SyntaxSnapshot};
pub struct Language {
    pub(crate) id: LanguageId,
    pub(crate) config: LanguageConfig,
    pub(crate) grammar: Option<Arc<Grammar>>,
    // pub(crate) context_provider: Option<Arc<dyn ContextProvider>>,
    // pub(crate) toolchain: Option<Arc<dyn ToolchainLister>>,
    pub(crate) manifest_name: Option<ManifestName>,
}
// pub use syntax_map::{
//     OwnedSyntaxLayer, SyntaxLayer, SyntaxMapMatches, ToTreeSitterPoint, TreeSitterOptions,
// };
