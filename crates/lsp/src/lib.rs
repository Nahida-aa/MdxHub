use collections::HashMap;
use gpui::{App, AppContext as _, AsyncApp, BackgroundExecutor, SharedString, Task};
pub use lsp_types::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use std::path::Path;
use std::{
    any::TypeId,
    collections::BTreeSet,
    ffi::{OsStr, OsString},
    fmt,
    io::Write,
    ops::DerefMut,
    path::PathBuf,
    pin::Pin,
    sync::{
        Arc, Weak,
        atomic::{AtomicI32, Ordering::SeqCst},
    },
    task::Poll,
    time::{Duration, Instant},
};

/// A name of a language server.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, JsonSchema,
)]
#[serde(transparent)]
pub struct LanguageServerName(pub SharedString);

/// Identifies a running language server.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LanguageServerId(pub usize);

impl LanguageServerId {
    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(self) -> u64 {
        self.0 as u64
    }
}

/// Represents a launchable language server. This can either be a standalone binary or the path
/// to a runtime with arguments to instruct it to launch the actual language server file.
#[derive(Clone, Serialize)]
pub struct LanguageServerBinary {
    pub path: PathBuf,
    pub arguments: Vec<OsString>,
    pub env: Option<HashMap<String, String>>,
}
