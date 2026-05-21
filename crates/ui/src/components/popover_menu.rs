use std::{cell::RefCell, rc::Rc};

use gpui::{
    AnyElement, App, DismissEvent, ElementId, Entity, Focusable, ManagedView, Pixels, Point, Window,
};
use gui::Anchor;

pub struct PopoverMenuHandle<M>(Rc<RefCell<Option<PopoverMenuHandleState<M>>>>);

impl<M> Clone for PopoverMenuHandle<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<M> Default for PopoverMenuHandle<M> {
    fn default() -> Self {
        Self(Rc::default())
    }
}
struct PopoverMenuHandleState<M> {
    menu_builder: Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>>>,
    menu: Rc<RefCell<Option<Entity<M>>>>,
    on_open: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

fn show_menu<M: ManagedView>(
    builder: &Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>>>,
    menu: &Rc<RefCell<Option<Entity<M>>>>,
    on_open: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    window: &mut Window,
    cx: &mut App,
) {
    let previous_focus_handle = window.focused(cx);
    let Some(new_menu) = (builder)(window, cx) else {
        return;
    };
    let menu2 = menu.clone();

    window
        .subscribe(&new_menu, cx, move |modal, _: &DismissEvent, window, cx| {
            if modal.focus_handle(cx).contains_focused(window, cx)
                && let Some(previous_focus_handle) = previous_focus_handle.as_ref()
            {
                window.focus(previous_focus_handle);
            }
            *menu2.borrow_mut() = None;
            window.refresh();
        })
        .detach();

    // Since menus are rendered in a deferred fashion, their focus handles are
    // not linked in the dispatch tree until after the deferred draw callback
    // runs. We need to wait for that to happen before focusing it, so that
    // calling `contains_focused` on the parent's focus handle returns `true`
    // when the menu is focused. This prevents the pane's tab bar buttons from
    // flickering when opening popover menus.
    let focus_handle = new_menu.focus_handle(cx);
    window.on_next_frame(move |window, _cx| {
        window.on_next_frame(move |window, cx| {
            window.focus(&focus_handle);
        });
    });
    *menu.borrow_mut() = Some(new_menu);
    window.refresh();

    if let Some(on_open) = on_open {
        on_open(window, cx);
    }
}

pub struct PopoverMenu<M: ManagedView> {
    id: ElementId,
    child_builder: Option<
        Box<
            dyn FnOnce(
                    Rc<RefCell<Option<Entity<M>>>>,
                    Option<Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>> + 'static>>,
                ) -> AnyElement
                + 'static,
        >,
    >,
    menu_builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<M>> + 'static>>,
    anchor: Anchor, // Corner
    attach: Option<Anchor>,
    offset: Option<Point<Pixels>>,
    trigger_handle: Option<PopoverMenuHandle<M>>,
    on_open: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    full_width: bool,
}
impl<M: ManagedView> PopoverMenu<M> {
    /// Returns a new [`PopoverMenu`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            child_builder: None,
            menu_builder: None,
            anchor: Anchor::TopLeft,
            attach: None,
            offset: None,
            trigger_handle: None,
            on_open: None,
            full_width: false,
        }
    }
    pub fn menu(
        mut self,
        f: impl Fn(&mut Window, &mut App) -> Option<Entity<M>> + 'static,
    ) -> Self {
        self.menu_builder = Some(Rc::new(f));
        self
    }
}
impl<M: ManagedView> PopoverMenuHandle<M> {
    pub fn show(&self, window: &mut Window, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref() {
            show_menu(
                &state.menu_builder,
                &state.menu,
                state.on_open.clone(),
                window,
                cx,
            );
        }
    }
    pub fn hide(&self, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref()
            && let Some(menu) = state.menu.borrow().as_ref()
        {
            menu.update(cx, |_, cx| cx.emit(DismissEvent));
        }
    }
    pub fn is_deployed(&self) -> bool {
        self.0
            .borrow()
            .as_ref()
            .is_some_and(|state| state.menu.borrow().as_ref().is_some())
    }
}
