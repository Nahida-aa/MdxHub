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
}
