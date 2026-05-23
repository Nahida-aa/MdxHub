pub mod item;
mod multi_workspace;
pub mod pane;
pub mod pane_group;
pub mod searchable;
mod toolbar;
pub use item::{
    // FollowableItem, FollowableItemHandle, Item, ItemHandle, ItemSettings, PreviewTabsSettings,
    // ProjectItem, SerializableItem, SerializableItemHandle,
    WeakItemHandle,
};
pub use pane_group::{
    // ActivePaneDecorator, HANDLE_HITBOX_SIZE, Member, PaneAxis, PaneGroup, PaneRenderContext,
    SplitDirection,
};
mod workspace_settings;
use gpui::{App, Entity, FocusHandle, Focusable, WeakEntity};
pub use pane::*;
use project::Project;
pub use toolbar::{
    // PaneSearchBarCallbacks,
    Toolbar,
    // ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
};
pub use workspace_settings::{
    // AutosaveSetting, BottomDockLayout, EncodingDisplayOptions, FocusFollowsMouse,
    // RestoreOnStartupBehavior, StatusBarSettings, TabBarSettings,
    WorkspaceSettings,
};
pub struct Workspace {
    weak_self: WeakEntity<Self>,
    project: Entity<Project>,
    active_pane: Entity<Pane>,
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

impl Focusable for Workspace {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane.focus_handle(cx)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct WorkspaceId(i64);
