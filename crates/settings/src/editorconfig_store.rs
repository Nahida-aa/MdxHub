use anyhow::{Context as _, Result};
use collections::{BTreeMap, BTreeSet, HashSet};
use ec4rs::{ConfigParser, PropertiesSource, Section};
use gpui::{Context, EventEmitter, Task};
use smallvec::SmallVec;
use std::{path::Path, str::FromStr, sync::Arc};
use util::{rel_path::RelPath, ResultExt as _};

use crate::{watch_config_file, InvalidSettingsError, LocalSettingsPath, WorktreeId};

#[derive(Clone)]
pub struct Editorconfig {
    pub is_root: bool,
    pub sections: SmallVec<[Section; 5]>,
}
#[derive(Default)]
struct EditorconfigWorktreeState {
    internal_configs: BTreeMap<Arc<RelPath>, (String, Option<Editorconfig>)>,
    external_config_paths: BTreeSet<Arc<Path>>,
}
#[derive(Default)]
pub struct EditorconfigStore {
    external_configs: BTreeMap<Arc<Path>, (String, Option<Editorconfig>)>,
    worktree_state: BTreeMap<WorktreeId, EditorconfigWorktreeState>,
    local_external_config_watchers: BTreeMap<Arc<Path>, Task<()>>,
    local_external_config_discovery_tasks: BTreeMap<WorktreeId, Task<()>>,
}
impl EditorconfigStore {}
