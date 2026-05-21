use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, SharedString, Task, WeakEntity,
};

/// A collection of worktree trust metadata, can be accessed globally (if initialized) and subscribed to.
pub struct TrustedWorktrees(Entity<TrustedWorktreesStore>);

impl Global for TrustedWorktrees {}

// TODO
/// A collection of worktrees that are considered trusted and not trusted.
/// This can be used when checking for this criteria before enabling certain features.
///
/// Emits an event each time the worktree was checked and found not trusted,
/// or a certain worktree had been trusted.
#[derive(Debug)]
pub struct TrustedWorktreesStore {
    // worktree_stores: HashMap<WeakEntity<WorktreeStore>, StoreData>,
    // db_trusted_paths: DbTrustedPaths,
    // trusted_paths: TrustedPaths,
    // restricted: HashMap<WeakEntity<WorktreeStore>, HashSet<WorktreeId>>,
    // worktree_trust_serialization: Task<()>,
}
