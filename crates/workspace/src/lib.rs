mod multi_workspace;
mod workspace_settings;
use gpui::{Entity, WeakEntity};
use project::Project;
pub use workspace_settings::{
    // AutosaveSetting, BottomDockLayout, EncodingDisplayOptions, FocusFollowsMouse,
    // RestoreOnStartupBehavior, StatusBarSettings, TabBarSettings,
    WorkspaceSettings,
};

pub struct Workspace {
    weak_self: WeakEntity<Self>,
    project: Entity<Project>,
}
pub use multi_workspace::{
    // CloseWorkspaceSidebar, DraggedSidebar, FocusWorkspaceSidebar, MoveProjectToNewWindow,
    MultiWorkspace,
    //  MultiWorkspaceEvent, NewThread, NextProject, NextThread, PreviousProject,
    // PreviousThread, ProjectGroup, ProjectGroupKey, SerializedProjectGroupState, Sidebar,
    // SidebarEvent, SidebarHandle, SidebarRenderState, SidebarSide, ToggleWorkspaceSidebar,
    // sidebar_side_context_menu,
};
impl Workspace {
    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    pub fn weak_handle(&self) -> WeakEntity<Self> {
        self.weak_self.clone()
    }
}
