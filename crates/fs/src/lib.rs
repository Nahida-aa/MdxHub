use anyhow::{Context as _, Result, anyhow};
use async_tar::Archive;
use futures::{AsyncRead, Stream, StreamExt, future::BoxFuture};
use gpui::BackgroundExecutor;
use rope::Rope;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::Component;
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use text::LineEnding;

pub trait Watcher: Send + Sync {
    fn add(&self, path: &Path) -> Result<()>;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PathEventKind {
    Removed,
    Created,
    Changed,
    Rescan,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PathEvent {
    pub path: PathBuf,
    pub kind: Option<PathEventKind>,
}

#[derive(Copy, Clone, Default)]
pub struct CreateOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}
#[derive(Copy, Clone, Default)]
pub struct CopyOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}
#[derive(Copy, Clone, Default)]
pub struct RenameOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
    /// Whether to create parent directories if they do not exist.
    pub create_parents: bool,
}
#[derive(Copy, Clone, Default)]
pub struct RemoveOptions {
    pub recursive: bool,
}

#[async_trait::async_trait]
pub trait Fs: Send + Sync {
    async fn create_dir(&self, path: &Path) -> Result<()>;
    async fn create_symlink(&self, path: &Path, target: PathBuf) -> Result<()>;
    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()>;
    async fn create_file_with(
        &self,
        path: &Path,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> Result<()>;
    async fn extract_tar_file(
        &self,
        path: &Path,
        content: Archive<Pin<&mut (dyn AsyncRead + Send)>>,
    ) -> Result<()>;
    async fn copy_file(&self, source: &Path, target: &Path, options: CopyOptions) -> Result<()>;
    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()>;

    /// Removes a directory from the filesystem.
    /// There is no expectation that the directory will be preserved in the
    /// system trash.
    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()>;

    /// Moves a file or directory to the system trash.
    /// Returns a [`TrashedEntry`] that can be used to keep track of the
    /// location of the trashed item in the system's trash.
    async fn trash(&self, path: &Path, options: RemoveOptions) -> Result<TrashedEntry>;

    /// Removes a file from the filesystem.
    /// There is no expectation that the file will be preserved in the system
    /// trash.
    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()>;

    async fn open_handle(&self, path: &Path) -> Result<Arc<dyn FileHandle>>;
    async fn open_sync(&self, path: &Path) -> Result<Box<dyn io::Read + Send + Sync>>;
    async fn load(&self, path: &Path) -> Result<String> {
        Ok(String::from_utf8(self.load_bytes(path).await?)?)
    }
    async fn load_bytes(&self, path: &Path) -> Result<Vec<u8>>;
    async fn atomic_write(&self, path: PathBuf, text: String) -> Result<()>;
    async fn save(&self, path: &Path, text: &Rope, line_ending: LineEnding) -> Result<()>;
    async fn write(&self, path: &Path, content: &[u8]) -> Result<()>;
    async fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
    async fn is_file(&self, path: &Path) -> bool;
    async fn is_dir(&self, path: &Path) -> bool;
    async fn metadata(&self, path: &Path) -> Result<Option<Metadata>>;
    async fn read_link(&self, path: &Path) -> Result<PathBuf>;
    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>>;

    async fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> (
        Pin<Box<dyn Send + Stream<Item = Vec<PathEvent>>>>,
        Arc<dyn Watcher>,
    );

    // fn open_repo( // TODO
    //     &self,
    //     abs_dot_git: &Path,
    //     system_git_binary_path: Option<&Path>,
    // ) -> Result<Arc<dyn GitRepository>>;
    async fn git_init(&self, abs_work_directory: &Path, fallback_branch_name: String)
    -> Result<()>;
    async fn git_clone(&self, abs_work_directory: &Path, repo_url: &str) -> Result<()>;
    async fn git_config(&self, abs_work_directory: &Path, args: Vec<String>) -> Result<String>;
    fn is_fake(&self) -> bool;
    async fn is_case_sensitive(&self) -> bool;
    // fn subscribe_to_jobs(&self) -> JobEventReceiver;

    /// Restores a given `TrashedEntry`, moving it from the system's trash back
    /// to the original path.
    // async fn restore(
    //     &self,
    //     trashed_entry: TrashedEntry,
    // ) -> std::result::Result<PathBuf, TrashRestoreError>;

    #[cfg(feature = "test-support")]
    fn as_fake(&self) -> Arc<FakeFs> {
        panic!("called as_fake on a real fs");
    }
}
// We use our own type rather than `trash::TrashItem` directly to avoid carrying
// over fields we don't need (e.g. `time_deleted`) and to insulate callers and
// tests from changes to that crate's API surface.
/// Represents a file or directory that has been moved to the system trash,
/// retaining enough information to restore it to its original location.
#[derive(Clone, PartialEq, Debug)]
pub struct TrashedEntry {
    /// Platform-specific identifier for the file/directory in the trash.
    ///
    /// * Freedesktop – Path to the `.trashinfo` file.
    /// * macOS & Windows – Full path to the file/directory in the system's
    /// trash.
    pub id: OsString,
    /// Name of the file/directory at the time of trashing, including extension.
    pub name: OsString,
    /// Absolute path to the parent directory at the time of trashing.
    pub original_parent: PathBuf,
}

pub trait FileHandle: Send + Sync + std::fmt::Debug {
    fn current_path(&self, fs: &Arc<dyn Fs>) -> Result<PathBuf>;
}

#[derive(Copy, Clone, Debug)]
pub struct Metadata {
    pub inode: u64,
    pub mtime: MTime,
    pub is_symlink: bool,
    pub is_dir: bool,
    pub len: u64,
    pub is_fifo: bool,
    pub is_executable: bool,
}

/// Filesystem modification time. The purpose of this newtype is to discourage use of operations
/// that do not make sense for mtimes. In particular, it is not always valid to compare mtimes using
/// `<` or `>`, as there are many things that can cause the mtime of a file to be earlier than it
/// was. See ["mtime comparison considered harmful" - apenwarr](https://apenwarr.ca/log/20181113).
///
/// Do not derive Ord, PartialOrd, or arithmetic operation traits.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MTime(SystemTime);
