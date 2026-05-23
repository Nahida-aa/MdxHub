use futures::{
    FutureExt as _, Stream, StreamExt,
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    select_biased, stream,
    task::Poll,
};
use smallvec::{SmallVec, smallvec};
use std::{
    any::Any,
    borrow::Borrow as _,
    cmp::Ordering,
    collections::hash_map,
    convert::TryFrom,
    ffi::OsStr,
    fmt,
    future::Future,
    mem::{self},
    ops::{Deref, DerefMut, Range},
    path::{Path, PathBuf},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
    time::{Duration, Instant},
};
mod ignore;
mod worktree_settings;
use ::ignore::gitignore::{
    Gitignore,
    // GitignoreBuilder
};
use collections::{HashMap, VecDeque};
use fs::{Fs, MTime};
use fuzzy::CharBag;
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, Context, Entity, EventEmitter, Task,
};
use postage::{
    barrier,
    prelude::{Sink as _, Stream as _},
    watch,
};
use settings::WorktreeId;
use sum_tree::{SumTree, TreeMap};
use util::{
    paths::{PathStyle, SanitizedPath},
    rel_path::RelPath,
};
pub use worktree_settings::WorktreeSettings;

/// A set of local or remote files that are being opened as part of a project.
/// Responsible for tracking related FS (for local)/collab (for remote) events and corresponding updates.
/// Stores git repositories data and the diagnostics for the file(s).
///
/// Has an absolute path, and may be set to be visible in UI or not.
/// May correspond to a directory or a single file.
/// Possible examples:
/// * a drag and dropped file — may be added as an invisible, "ephemeral" entry to the current worktree
/// * a directory opened in Zed — may be added as a visible entry to the current worktree
///
/// Uses [`Entry`] to track the state of each file/directory, can look up absolute paths for entries.
pub enum Worktree {
    Local(LocalWorktree),
    // Remote(RemoteWorktree),
}

pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    scan_requests_tx: async_channel::Sender<ScanRequest>,
    path_prefixes_to_scan_tx: async_channel::Sender<PathPrefixScanRequest>,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    snapshot_subscriptions: VecDeque<(usize, oneshot::Sender<()>)>,
    _background_scanner_tasks: Vec<Task<()>>,
    update_observer: Option<UpdateObservationState>,
    fs: Arc<dyn Fs>,
    fs_case_sensitive: bool,
    visible: bool,
    next_entry_id: Arc<AtomicUsize>,
    settings: WorktreeSettings,
    share_private_files: bool,
    scanning_enabled: bool,
    force_defer_watch: bool,
}
pub struct PathPrefixScanRequest {
    path: Arc<RelPath>,
    done: SmallVec<[barrier::Sender; 1]>,
}
struct ScanRequest {
    relative_paths: Vec<Arc<RelPath>>,
    done: SmallVec<[barrier::Sender; 1]>,
}

#[derive(Clone)]
pub struct LocalSnapshot {
    snapshot: Snapshot,
    global_gitignore: Option<Arc<Gitignore>>,
    /// Exclude files for all git repositories in the worktree, indexed by their absolute path.
    /// The boolean indicates whether the gitignore needs to be updated.
    repo_exclude_by_work_dir_abs_path: HashMap<Arc<Path>, (Arc<Gitignore>, bool)>,
    /// All of the gitignore files in the worktree, indexed by their absolute path.
    /// The boolean indicates whether the gitignore needs to be updated.
    ignores_by_parent_abs_path: HashMap<Arc<Path>, (Arc<Gitignore>, bool)>,
    /// All of the git repositories in the worktree, indexed by the project entry
    /// id of their parent directory.
    git_repositories: TreeMap<ProjectEntryId, LocalRepositoryEntry>,
    /// The file handle of the worktree root
    /// (so we can find it after it's been moved)
    root_file_handle: Option<Arc<dyn fs::FileHandle>>,
}

#[derive(Clone)]
pub struct Snapshot {
    id: WorktreeId,
    /// The absolute path of the worktree root.
    abs_path: Arc<SanitizedPath>,
    path_style: PathStyle,
    root_name: Arc<RelPath>,
    root_char_bag: CharBag,
    entries_by_path: SumTree<Entry>,
    entries_by_id: SumTree<PathEntry>,
    root_repo_common_dir: Option<Arc<SanitizedPath>>,
    always_included_entries: Vec<Arc<RelPath>>,

    /// A number that increases every time the worktree begins scanning
    /// a set of paths from the filesystem. This scanning could be caused
    /// by some operation performed on the worktree, such as reading or
    /// writing a file, or by an event reported by the filesystem.
    scan_id: usize,

    /// The latest scan id that has completed, and whose preceding scans
    /// have all completed. The current `scan_id` could be more than one
    /// greater than the `completed_scan_id` if operations are performed
    /// on the worktree while it is processing a file-system event.
    completed_scan_id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: ProjectEntryId,
    pub kind: EntryKind,
    pub path: Arc<RelPath>,
    pub inode: u64,
    pub mtime: Option<MTime>,

    pub canonical_path: Option<Arc<Path>>,
    /// Whether this entry is ignored by Git.
    ///
    /// We only scan ignored entries once the directory is expanded and
    /// exclude them from searches.
    pub is_ignored: bool,

    /// Whether this entry is hidden or inside hidden directory.
    ///
    /// We only scan hidden entries once the directory is expanded.
    pub is_hidden: bool,

    /// Whether this entry is always included in searches.
    ///
    /// This is used for entries that are always included in searches, even
    /// if they are ignored by git. Overridden by file_scan_exclusions.
    pub is_always_included: bool,

    /// Whether this entry's canonical path is outside of the worktree.
    /// This means the entry is only accessible from the worktree root via a
    /// symlink.
    ///
    /// We only scan entries outside of the worktree once the symlinked
    /// directory is expanded.
    pub is_external: bool,

    /// Whether this entry is considered to be a `.env` file.
    pub is_private: bool,
    /// The entry's size on disk, in bytes.
    pub size: u64,
    pub char_bag: CharBag,
    pub is_fifo: bool,
}
impl Entry {
    pub fn is_file(&self) -> bool {
        self.kind.is_file()
    }
}
impl EntryKind {
    pub fn is_dir(&self) -> bool {
        matches!(
            self,
            EntryKind::Dir | EntryKind::PendingDir | EntryKind::UnloadedDir
        )
    }

    pub fn is_unloaded(&self) -> bool {
        matches!(self, EntryKind::UnloadedDir)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, EntryKind::File)
    }
}
impl sum_tree::Item for Entry {
    type Summary = EntrySummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        let non_ignored_count = if self.is_ignored && !self.is_always_included {
            0
        } else {
            1
        };
        let file_count;
        let non_ignored_file_count;
        if self.is_file() {
            file_count = 1;
            non_ignored_file_count = non_ignored_count;
        } else {
            file_count = 0;
            non_ignored_file_count = 0;
        }

        EntrySummary {
            max_path: self.path.clone(),
            count: 1,
            non_ignored_count,
            file_count,
            non_ignored_file_count,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectEntryId(usize);

impl ProjectEntryId {
    pub const MAX: Self = Self(usize::MAX);
    pub const MIN: Self = Self(usize::MIN);

    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(self) -> u64 {
        self.0 as u64
    }

    pub fn from_usize(id: usize) -> Self {
        ProjectEntryId(id)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    UnloadedDir,
    PendingDir,
    Dir,
    File,
}

#[derive(Debug, Clone)]
struct LocalRepositoryEntry {
    work_directory_id: ProjectEntryId,
    work_directory: WorkDirectory,
    work_directory_abs_path: Arc<Path>,
    git_dir_scan_id: usize,
    /// Absolute path to the original .git entry that caused us to create this repository.
    ///
    /// This is normally a directory, but may be a "gitfile" that points to a directory elsewhere
    /// (whose path we then store in `repository_dir_abs_path`).
    dot_git_abs_path: Arc<Path>,
    /// Absolute path to the "commondir" for this repository.
    ///
    /// This is always a directory. For a normal repository, this is the same as
    /// `dot_git_abs_path`. For a linked worktree, this is the main repo's `.git`
    /// directory (resolved from the worktree's `commondir` file). For a submodule,
    /// this equals `repository_dir_abs_path` (submodules don't have a `commondir`
    /// file).
    common_dir_abs_path: Arc<Path>,
    /// Absolute path to the directory holding the repository's state.
    ///
    /// For a normal repository, this is a directory and coincides with `dot_git_abs_path` and
    /// `common_dir_abs_path`. For a submodule or worktree, this is some subdirectory of the
    /// commondir like `/project/.git/modules/foo`.
    repository_dir_abs_path: Arc<Path>,
}

/// This path corresponds to the 'content path' of a repository in relation
/// to Zed's project root.
/// In the majority of the cases, this is the folder that contains the .git folder.
/// But if a sub-folder of a git repository is opened, this corresponds to the
/// project root and the .git folder is located in a parent directory.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WorkDirectory {
    InProject {
        relative_path: Arc<RelPath>,
    },
    AboveProject {
        absolute_path: Arc<Path>,
        location_in_repo: Arc<Path>,
    },
}

struct UpdateObservationState {
    snapshots_tx: mpsc::UnboundedSender<(LocalSnapshot, UpdatedEntriesSet)>,
    resume_updates: watch::Sender<()>,
    _maintain_remote_snapshot: Task<Option<()>>,
}

pub type UpdatedEntriesSet = Arc<[(Arc<RelPath>, ProjectEntryId, PathChange)]>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathChange {
    /// A filesystem entry was was created.
    Added,
    /// A filesystem entry was removed.
    Removed,
    /// A filesystem entry was updated.
    Updated,
    /// A filesystem entry was either updated or added. We don't know
    /// whether or not it already existed, because the path had not
    /// been loaded before the event.
    AddedOrUpdated,
    /// A filesystem entry was found during the initial scan of the worktree.
    Loaded,
}

#[derive(Clone, Debug)]
pub struct EntrySummary {
    max_path: Arc<RelPath>,
    count: usize,
    non_ignored_count: usize,
    file_count: usize,
    non_ignored_file_count: usize,
}

impl Default for EntrySummary {
    fn default() -> Self {
        Self {
            max_path: Arc::from(RelPath::empty()),
            count: 0,
            non_ignored_count: 0,
            file_count: 0,
            non_ignored_file_count: 0,
        }
    }
}

impl sum_tree::ContextLessSummary for EntrySummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, rhs: &Self) {
        self.max_path = rhs.max_path.clone();
        self.count += rhs.count;
        self.non_ignored_count += rhs.non_ignored_count;
        self.file_count += rhs.file_count;
        self.non_ignored_file_count += rhs.non_ignored_file_count;
    }
}

#[derive(Clone, Debug)]
struct PathEntry {
    id: ProjectEntryId,
    path: Arc<RelPath>,
    is_ignored: bool,
    scan_id: usize,
}

impl sum_tree::Item for PathEntry {
    type Summary = PathEntrySummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        PathEntrySummary { max_id: self.id }
    }
}

#[derive(Clone, Debug, Default)]
struct PathEntrySummary {
    max_id: ProjectEntryId,
}

impl sum_tree::ContextLessSummary for PathEntrySummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.max_id = summary.max_id;
    }
}
