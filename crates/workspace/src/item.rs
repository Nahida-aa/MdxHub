use client::{
    Client,
    // proto
};
use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use crate::{Pane, Workspace, WorkspaceId, WorkspaceSettings, toolbar::ToolbarItemLocation};
use anyhow::Result;
use gpui::{
    Action, AnyElement, AnyEntity, AnyView, App, Context, Entity, EntityId, FocusHandle, Font,
    Point, SharedString, Task, Window,
};
use language::Capability;
use project::{Project, ProjectEntryId, ProjectPath};
use smallvec::SmallVec;
use ui::{
    Color,
    Icon,
    Pixels,
    // IntoElement, Label, LabelCommon
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ItemEvent {
    CloseItem,
    UpdateTab,
    UpdateBreadcrumbs,
    Edit,
}
#[derive(Clone, Copy, Default, Debug)]
pub struct TabContentParams {
    pub detail: Option<usize>,
    pub selected: bool,
    pub preview: bool,
    /// Tab content should be deemphasized when active pane does not have focus.
    pub deemphasized: bool,
}
pub enum TabTooltipContent {
    Text(SharedString),
    Custom(Box<dyn Fn(&mut Window, &mut App) -> AnyView>),
}

#[derive(Clone, Copy, Debug)]
pub struct SaveOptions {
    pub format: bool,
    pub force_format: bool,
    pub autosave: bool,
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self {
            format: true,
            force_format: false,
            autosave: false,
        }
    }
}

pub trait ItemHandle: 'static + Send {
    fn item_focus_handle(&self, cx: &App) -> FocusHandle;
    fn subscribe_to_item_events(
        &self,
        window: &mut Window,
        cx: &mut App,
        handler: Box<dyn Fn(ItemEvent, &mut Window, &mut App)>,
    ) -> gpui::Subscription;
    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> AnyElement;
    fn tab_content_text(&self, detail: usize, cx: &App) -> SharedString;
    fn suggested_filename(&self, cx: &App) -> SharedString;
    fn tab_icon(&self, window: &Window, cx: &App) -> Option<Icon>;
    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString>;
    fn tab_tooltip_content(&self, cx: &App) -> Option<TabTooltipContent>;
    fn telemetry_event_text(&self, cx: &App) -> Option<&'static str>;
    fn dragged_tab_content(
        &self,
        params: TabContentParams,
        window: &Window,
        cx: &App,
    ) -> AnyElement;
    fn project_path(&self, cx: &App) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &App) -> SmallVec<[ProjectEntryId; 3]>;
    fn project_paths(&self, cx: &App) -> SmallVec<[ProjectPath; 3]>;
    fn project_item_model_ids(&self, cx: &App) -> SmallVec<[EntityId; 3]>;
    fn for_each_project_item(
        &self,
        _: &App,
        _: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    );
    fn buffer_kind(&self, cx: &App) -> ItemBufferKind;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn can_split(&self, cx: &App) -> bool;
    fn clone_on_split(
        &self,
        workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Option<Box<dyn ItemHandle>>>;
    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    );
    fn deactivated(&self, window: &mut Window, cx: &mut App);
    fn on_removed(&self, cx: &mut App);
    fn workspace_deactivated(&self, window: &mut Window, cx: &mut App);
    fn navigate(&self, data: Arc<dyn Any + Send>, window: &mut Window, cx: &mut App) -> bool;
    fn item_id(&self) -> EntityId;
    fn to_any_view(&self) -> AnyView;
    fn is_dirty(&self, cx: &App) -> bool;
    fn capability(&self, cx: &App) -> Capability;
    fn toggle_read_only(&self, window: &mut Window, cx: &mut App);
    fn has_deleted_file(&self, cx: &App) -> bool;
    fn has_conflict(&self, cx: &App) -> bool;
    fn can_save(&self, cx: &App) -> bool;
    fn can_save_as(&self, cx: &App) -> bool;
    fn save(
        &self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn save_as(
        &self,
        project: Entity<Project>,
        path: ProjectPath,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn reload(
        &self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn act_as_type(&self, type_id: TypeId, cx: &App) -> Option<AnyEntity>;
    fn to_followable_item_handle(&self, cx: &App) -> Option<Box<dyn FollowableItemHandle>>;
    fn to_serializable_item_handle(&self, cx: &App) -> Option<Box<dyn SerializableItemHandle>>;
    fn on_release(
        &self,
        cx: &mut App,
        callback: Box<dyn FnOnce(&mut App) + Send>,
    ) -> gpui::Subscription;
    fn to_searchable_item_handle(&self, cx: &App) -> Option<Box<dyn SearchableItemHandle>>;
    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation;
    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)>;
    fn breadcrumb_prefix(&self, window: &mut Window, cx: &mut App) -> Option<gpui::AnyElement>;
    fn show_toolbar(&self, cx: &App) -> bool;
    fn pixel_position_of_cursor(&self, cx: &App) -> Option<Point<Pixels>>;
    fn downgrade_item(&self) -> Box<dyn WeakItemHandle>;
    fn workspace_settings<'a>(&self, cx: &'a App) -> &'a WorkspaceSettings;
    fn preserve_preview(&self, cx: &App) -> bool;
    fn include_in_nav_history(&self) -> bool;
    fn relay_action(&self, action: Box<dyn Action>, window: &mut Window, cx: &mut App);
    fn handle_drop(
        &self,
        active_pane: &Pane,
        dropped: &dyn Any,
        window: &mut Window,
        cx: &mut App,
    ) -> bool;
    fn tab_extra_context_menu_actions(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<(SharedString, Box<dyn Action>)>;
    fn can_autosave(&self, cx: &App) -> bool {
        let is_deleted = self.project_entry_ids(cx).is_empty();
        self.is_dirty(cx) && !self.has_conflict(cx) && self.can_save(cx) && !is_deleted
    }
}

pub trait WeakItemHandle: Send + Sync {
    fn id(&self) -> EntityId;
    fn boxed_clone(&self) -> Box<dyn WeakItemHandle>;
    fn upgrade(&self) -> Option<Box<dyn ItemHandle>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectItemKind(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemBufferKind {
    Multibuffer,
    Singleton,
    None,
}

pub trait FollowableItemHandle: ItemHandle {
    fn remote_id(&self, client: &Arc<Client>, window: &mut Window, cx: &mut App) -> Option<ViewId>;
    fn downgrade(&self) -> Box<dyn WeakFollowableItemHandle>;
    fn set_leader_id(
        &self,
        leader_peer_id: Option<CollaboratorId>,
        window: &mut Window,
        cx: &mut App,
    );
    fn to_state_proto(&self, window: &mut Window, cx: &mut App) -> Option<proto::view::Variant>;
    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool;
    fn to_follow_event(&self, event: &dyn Any) -> Option<FollowEvent>;
    fn apply_update_proto(
        &self,
        project: &Entity<Project>,
        message: proto::update_view::Variant,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>>;
    fn is_project_item(&self, window: &mut Window, cx: &mut App) -> bool;
    fn dedup(
        &self,
        existing: &dyn FollowableItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Dedup>;
    fn update_agent_location(&self, location: language::Anchor, window: &mut Window, cx: &mut App);
}
