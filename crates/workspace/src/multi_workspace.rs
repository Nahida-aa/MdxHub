use gpui::{
    AnyView,
    App,
    Context,
    DragMoveEvent,
    Entity,
    EntityId,
    EventEmitter,
    FocusHandle,
    Focusable,
    ManagedView,
    MouseButton,
    Pixels,
    Render,
    Subscription,
    Task,
    // TaskExt,
    Tiling,
    WeakEntity,
    Window,
    WindowId,
    actions,
    deferred,
    px,
};
use settings::SidebarSide;

pub struct MultiWorkspace {
    window_id: WindowId,
    sidebar_open: bool,
    sidebar: Option<Box<dyn SidebarHandle>>,
}
impl MultiWorkspace {
    pub fn sidebar_open(&self) -> bool {
        self.sidebar_open
    }
    pub fn is_threads_list_view_active(&self, cx: &App) -> bool {
        self.sidebar
            .as_ref()
            .map_or(false, |s| s.is_threads_list_view_active(cx))
    }
}

pub trait SidebarHandle: 'static + Send + Sync {
    fn width(&self, cx: &App) -> Pixels;
    fn set_width(&self, width: Option<Pixels>, cx: &mut App);
    fn focus_handle(&self, cx: &App) -> FocusHandle;
    fn focus(&self, window: &mut Window, cx: &mut App);
    fn prepare_for_focus(&self, window: &mut Window, cx: &mut App);
    fn has_notifications(&self, cx: &App) -> bool;
    fn to_any(&self) -> AnyView;
    fn entity_id(&self) -> EntityId;
    fn toggle_thread_switcher(&self, select_last: bool, window: &mut Window, cx: &mut App);
    fn cycle_project(&self, forward: bool, window: &mut Window, cx: &mut App);
    fn cycle_thread(&self, forward: bool, window: &mut Window, cx: &mut App);

    fn is_threads_list_view_active(&self, cx: &App) -> bool;

    fn side(&self, cx: &App) -> SidebarSide;
    fn serialized_state(&self, cx: &App) -> Option<String>;
    fn restore_serialized_state(&self, state: &str, window: &mut Window, cx: &mut App);
}
