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

pub struct MultiWorkspace {
    window_id: WindowId,
    sidebar_open: bool,
}
impl MultiWorkspace {
    pub fn sidebar_open(&self) -> bool {
        self.sidebar_open
    }
}
