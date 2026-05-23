use std::sync::Arc;

use client::user::Collaborator;
use clock::ReplicaId;
use gpui::{App, Entity};
use language::Capability;
use remote::remote_client::RemoteClient;
use settings_macros::RegisterSetting;

pub mod trusted_worktrees;
pub use worktree::{
    Entry, EntryKind, FS_WATCH_LATENCY, File, LocalWorktree, PathChange, ProjectEntryId,
    UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId, WorktreeSettings,
    discover_root_repo_common_dir,
};

#[derive(Debug)]
enum ProjectClientState {
    /// Single-player mode.
    Local,
    /// Multi-player mode but still a local project.
    Shared { remote_id: u64 },
    /// Multi-player mode but working on a remote project.
    Collab {
        sharing_has_stopped: bool,
        capability: Capability,
        remote_id: u64,
        replica_id: ReplicaId,
    },
}
pub struct Project {
    remote_client: Option<Entity<RemoteClient>>,
    // todo lw explain the client_state x remote_client matrix, its super confusing
    client_state: ProjectClientState,
}
impl Project {
    // #[inline]
    // pub fn host(&self) -> Option<&Collaborator> {
    //     self.collaborators.values().find(|c| c.is_host)
    // }

    /// Whether this project is a remote server (not counting collab).
    #[inline]
    pub fn is_via_remote_server(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => {
                self.remote_client.is_some()
            }
            ProjectClientState::Collab { .. } => false,
        }
    }
    #[inline]
    pub fn is_disconnected(&self, cx: &App) -> bool {
        match &self.client_state {
            ProjectClientState::Collab {
                sharing_has_stopped,
                ..
            } => *sharing_has_stopped,
            ProjectClientState::Local if self.is_via_remote_server() => {
                self.remote_client_is_disconnected(cx)
            }
            _ => false,
        }
    }
    #[inline]
    fn remote_client_is_disconnected(&self, cx: &App) -> bool {
        self.remote_client
            .as_ref()
            .map(|remote| remote.read(cx).is_disconnected())
            .unwrap_or(false)
    }
}

/// Whether to disable all AI features in Zed.
///
/// Default: false
#[derive(Copy, Clone, Debug, RegisterSetting)]
pub struct DisableAiSettings {
    pub disable_ai: bool,
}
impl settings::Settings for DisableAiSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            disable_ai: content.project.disable_ai.unwrap().0,
        }
    }
}

// impl DisableAiSettings {
//     /// Returns whether AI is disabled for the given file.
//     ///
//     /// This checks the project-level settings for the file's worktree,
//     /// allowing `disable_ai` to be configured per-project in `.zed/settings.json`.
//     pub fn is_ai_disabled_for_buffer(buffer: Option<&Entity<Buffer>>, cx: &App) -> bool {
//         Self::is_ai_disabled_for_file(buffer.and_then(|buffer| buffer.read(cx).file()), cx)
//     }

//     pub fn is_ai_disabled_for_file(file: Option<&Arc<dyn language::File>>, cx: &App) -> bool {
//         let location = file.map(|f| settings::SettingsLocation {
//             worktree_id: f.worktree_id(cx),
//             path: f.path().as_ref(),
//         });
//         Self::get(location, cx).disable_ai
//     }
// }
