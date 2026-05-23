use std::{
    any::Any,
    num::NonZeroUsize,
    rc::Rc,
    sync::{Arc, atomic::AtomicUsize},
};

use crate::{
    SplitDirection, Toolbar, WeakItemHandle, Workspace,
    item::{ItemHandle, ProjectItemKind},
    workspace_settings::FocusFollowsMouse,
};
use collections::{HashMap, HashSet};
use gpui::{
    Action, Entity, EntityId, FocusHandle, ScrollHandle, Subscription, Task, WeakEntity,
    WeakFocusHandle,
};
use language::{Capability, DiagnosticSeverity};
use project::{Project, ProjectPath};
use ui::{AnyElement, App, Context, PopoverMenuHandle, Window};

/// A container for 0 to many items that are open in the workspace.
/// Treats all items uniformly via the [`ItemHandle`] trait, whether it's an editor, search results multibuffer, terminal or something else,
/// responsible for managing item tabs, focus and zoom states and drag and drop features.
/// Can be split, see `PaneGroup` for more details.
pub struct Pane {
    alternate_file_items: (
        Option<Box<dyn WeakItemHandle>>,
        Option<Box<dyn WeakItemHandle>>,
    ),
    focus_handle: FocusHandle,
    items: Vec<Box<dyn ItemHandle>>,
    activation_history: Vec<ActivationHistoryEntry>,
    next_activation_timestamp: Arc<AtomicUsize>,
    zoomed: bool,
    was_focused: bool,
    active_item_index: usize,
    preview_item_id: Option<EntityId>,
    last_focus_handle_by_item: HashMap<EntityId, WeakFocusHandle>,
    nav_history: NavHistory,
    toolbar: Entity<Toolbar>,
    pub(crate) workspace: WeakEntity<Workspace>,
    project: WeakEntity<Project>,
    pub drag_split_direction: Option<SplitDirection>,
    can_drop_predicate: Option<Arc<dyn Fn(&dyn Any, &mut Window, &mut App) -> bool>>,
    can_split_predicate:
        Option<Arc<dyn Fn(&mut Self, &dyn Any, &mut Window, &mut Context<Self>) -> bool>>,
    can_toggle_zoom: bool,
    should_display_tab_bar: Rc<dyn Fn(&Window, &mut Context<Pane>) -> bool>,
    should_display_welcome_page: bool,
    render_tab_bar_buttons: Rc<
        dyn Fn(
            &mut Pane,
            &mut Window,
            &mut Context<Pane>,
        ) -> (Option<AnyElement>, Option<AnyElement>),
    >,
    render_tab_bar: Rc<dyn Fn(&mut Pane, &mut Window, &mut Context<Pane>) -> AnyElement>,
    show_tab_bar_buttons: bool,
    max_tabs: Option<NonZeroUsize>,
    use_max_tabs: bool,
    _subscriptions: Vec<Subscription>,
    tab_bar_scroll_handle: ScrollHandle,
    /// This is set to true if a user scroll has occurred more recently than a system scroll
    /// We want to suppress certain system scrolls when the user has intentionally scrolled
    suppress_scroll: bool,
    /// Is None if navigation buttons are permanently turned off (and should not react to setting changes).
    /// Otherwise, when `display_nav_history_buttons` is Some, it determines whether nav buttons should be displayed.
    display_nav_history_buttons: Option<bool>,
    double_click_dispatch_action: Box<dyn Action>,
    save_modals_spawned: HashSet<EntityId>,
    close_pane_if_empty: bool,
    // pub new_item_context_menu_handle: PopoverMenuHandle<PopupMenu>,
    // pub split_item_context_menu_handle: PopoverMenuHandle<PopupMenu>,
    pinned_tab_count: usize,
    diagnostics: HashMap<ProjectPath, DiagnosticSeverity>,
    zoom_out_on_close: bool,
    focus_follows_mouse: FocusFollowsMouse,
    diagnostic_summary_update: Task<()>,
    /// If a certain project item wants to get recreated with specific data, it can persist its data before the recreation here.
    pub project_item_restoration_data: HashMap<ProjectItemKind, Box<dyn Any + Send>>,
    // welcome_page: Option<Entity<crate::welcome::WelcomePage>>,
    pub in_center_group: bool,
}

pub struct ActivationHistoryEntry {
    pub entity_id: EntityId,
    pub timestamp: usize,
}

#[derive(Clone)]
pub struct NavHistory(Arc<Mutex<NavHistoryState>>);
