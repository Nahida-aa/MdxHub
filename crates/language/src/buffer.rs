use anyhow::{Context as _, Result};
use fs::MTime;
use gpui::{App, Task};
pub use lsp::DiagnosticSeverity;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Error as Error_},
};
use settings::WorktreeId;
use std::{
    any::Any,
    borrow::Cow,
    cell::Cell,
    cmp::{self, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet},
    future::Future,
    iter::{self, Iterator, Peekable},
    mem,
    num::NonZeroU32,
    ops::{Deref, Range},
    path::PathBuf,
    rc,
    sync::Arc,
    time::{Duration, Instant},
    vec,
};
use util::{paths::PathStyle, rel_path::RelPath};

/// Indicate whether a [`Buffer`] has permissions to edit.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Capability {
    /// The buffer is a mutable replica.
    ReadWrite,
    /// The buffer is a mutable replica, but toggled to be only readable.
    Read,
    /// The buffer is a read-only replica.
    ReadOnly,
}
impl Capability {
    /// Returns `true` if the capability is `ReadWrite`.
    pub fn editable(self) -> bool {
        matches!(self, Capability::ReadWrite)
    }
}

/// An in-memory representation of a source code file, including its text,
/// syntax trees, git status, and diagnostics.
pub struct Buffer {
    text: TextBuffer,
    branch_state: Option<BufferBranchState>,
    /// Filesystem state, `None` when there is no path.
    file: Option<Arc<dyn File>>,
    /// The mtime of the file when this buffer was last loaded from
    /// or saved to disk.
    saved_mtime: Option<MTime>,
    /// The version vector when this buffer was last loaded from
    /// or saved to disk.
    saved_version: clock::Global,
    preview_version: clock::Global,
    transaction_depth: usize,
    was_dirty_before_starting_transaction: Option<bool>,
    reload_task: Option<Task<Result<()>>>,
    language: Option<Arc<Language>>,
    autoindent_requests: Vec<Arc<AutoindentRequest>>,
    wait_for_autoindent_txs: Vec<oneshot::Sender<()>>,
    pending_autoindent: Option<Task<()>>,
    sync_parse_timeout: Option<Duration>,
    syntax_map: Mutex<SyntaxMap>,
    reparse: Option<Task<()>>,
    parse_status: (watch::Sender<ParseStatus>, watch::Receiver<ParseStatus>),
    non_text_state_update_count: usize,
    diagnostics: TreeMap<LanguageServerId, DiagnosticSet>,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    diagnostics_timestamp: clock::Lamport,
    completion_triggers: BTreeSet<String>,
    completion_triggers_per_language_server: HashMap<LanguageServerId, BTreeSet<String>>,
    completion_triggers_timestamp: clock::Lamport,
    deferred_ops: OperationQueue<Operation>,
    capability: Capability,
    has_conflict: bool,
    /// Memoize calls to has_changes_since(saved_version).
    /// The contents of a cell are (self.version, has_changes) at the time of a last call.
    has_unsaved_edits: Cell<(clock::Global, bool)>,
    change_bits: Vec<rc::Weak<Cell<bool>>>,
    modeline: Option<Arc<ModelineSettings>>,
    _subscriptions: Vec<gpui::Subscription>,
    tree_sitter_data: Arc<TreeSitterData>,
    encoding: &'static Encoding,
    has_bom: bool,
    reload_with_encoding_txns: HashMap<TransactionId, (&'static Encoding, bool)>,
}

/// The file associated with a buffer.
pub trait File: Send + Sync + Any {
    /// Returns the [`LocalFile`] associated with this file, if the
    /// file is local.
    fn as_local(&self) -> Option<&dyn LocalFile>;

    /// Returns whether this file is local.
    fn is_local(&self) -> bool {
        self.as_local().is_some()
    }

    /// Returns whether the file is new, exists in storage, or has been deleted. Includes metadata
    /// only available in some states, such as modification time.
    fn disk_state(&self) -> DiskState;

    /// Returns the path of this file relative to the worktree's root directory.
    fn path(&self) -> &Arc<RelPath>;

    /// Returns the path of this file relative to the worktree's parent directory (this means it
    /// includes the name of the worktree's root folder).
    fn full_path(&self, cx: &App) -> PathBuf;

    /// Returns the path style of this file.
    fn path_style(&self, cx: &App) -> PathStyle;

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name<'a>(&'a self, cx: &'a App) -> &'a str;

    /// Returns the id of the worktree to which this file belongs.
    ///
    /// This is needed for looking up project-specific settings.
    fn worktree_id(&self, cx: &App) -> WorktreeId;

    // /// Converts this file into a protobuf message.
    // fn to_proto(&self, cx: &App) -> rpc::proto::File;

    /// Return whether Zed considers this to be a private file.
    fn is_private(&self) -> bool;

    fn can_open(&self) -> bool {
        !self.is_local()
    }
}

/// The file associated with a buffer, in the case where the file is on the local disk.
pub trait LocalFile: File {
    /// Returns the absolute path of this file
    fn abs_path(&self, cx: &App) -> PathBuf;

    /// Loads the file contents from disk and returns them as a UTF-8 encoded string.
    fn load(&self, cx: &App) -> Task<Result<String>>;

    /// Loads the file's contents from disk.
    fn load_bytes(&self, cx: &App) -> Task<Result<Vec<u8>>>;
}

/// The file's storage status - whether it's stored (`Present`), and if so when it was last
/// modified. In the case where the file is not stored, it can be either `New` or `Deleted`. In the
/// UI these two states are distinguished. For example, the buffer tab does not display a deletion
/// indicator for new files.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DiskState {
    /// File created in Zed that has not been saved.
    New,
    /// File present on the filesystem.
    Present { mtime: MTime, size: u64 },
    /// Deleted file that was previously present.
    Deleted,
    /// An old version of a file that was previously present
    /// usually from a version control system. e.g. A git blob
    Historic { was_deleted: bool },
}
