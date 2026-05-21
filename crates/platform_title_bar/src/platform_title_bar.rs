use gpui::prelude::*;
use gpui::*;
use gpui_component::{h_flex, ActiveTheme, Icon, IconName, InteractiveElementExt, Sizable};

pub const TITLE_BAR_HEIGHT: Pixels = px(34.);

pub fn init(_cx: &mut App) {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlatformStyle {
    Mac,
    Linux,
    Windows,
}

impl PlatformStyle {
    fn platform() -> Self {
        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            Self::Linux
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Mac
        }
    }
}

#[allow(dead_code)]
pub struct PlatformTitleBar {
    children: Vec<AnyElement>,
    should_move: bool,
    platform_style: PlatformStyle,
    id: ElementId,
}

impl PlatformTitleBar {
    pub fn new(
        id: impl Into<ElementId>,
        children: impl IntoIterator<Item = AnyElement>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id: id.into(),
            children: children.into_iter().collect(),
            should_move: false,
            platform_style: PlatformStyle::platform(),
        }
    }

    pub fn set_children(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children = children.into_iter().collect();
    }
}

impl Render for PlatformTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let decorations = window.window_decorations();
        let is_client = matches!(decorations, Decorations::Client { .. });
        let children = std::mem::take(&mut self.children);

        let close_btn = control_btn("close", IconName::WindowClose, |_, window, _| {
            window.remove_window();
        });
        let maximize_btn = control_btn(
            if window.is_maximized() {
                "restore"
            } else {
                "maximize"
            },
            IconName::WindowMaximize,
            |_, window, _| window.zoom_window(),
        );
        let minimize_btn = control_btn("minimize", IconName::WindowMinimize, |_, window, _| {
            window.minimize_window();
        });

        let mut title_bar = h_flex()
            .id(self.id.clone())
            .window_control_area(WindowControlArea::Drag)
            .w_full()
            .h(TITLE_BAR_HEIGHT);

        // 3-step drag-to-move pattern (required for Wayland)
        title_bar = title_bar.on_mouse_down_out(
            cx.listener(|this, _: &MouseDownEvent, _window, _cx| {
                this.should_move = false;
            }),
        );
        title_bar = title_bar.on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _: &MouseUpEvent, _window, _cx| {
                this.should_move = false;
            }),
        );
        title_bar = title_bar.on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _: &MouseDownEvent, _window, _cx| {
                this.should_move = true;
            }),
        );
        title_bar = title_bar.on_mouse_move(
            cx.listener(|this, _: &MouseMoveEvent, window, _| {
                if this.should_move {
                    this.should_move = false;
                    window.start_window_move();
                }
            }),
        );

        title_bar = title_bar
            .on_double_click(|_, window, _| {
                window.zoom_window();
            })
            .pl(px(12.))
            .bg(cx.theme().title_bar)
            .border_b_1()
            .border_color(cx.theme().title_bar_border)
            .child(
                h_flex()
                    .id("bar")
                    .flex_1()
                    .h_full()
                    .children(children),
            );

        if is_client {
            title_bar = title_bar.child(
                h_flex()
                    .id("window-controls")
                    .items_center()
                    .flex_shrink_0()
                    .h_full()
                    .child(minimize_btn)
                    .child(maximize_btn)
                    .child(close_btn),
            );
        }

        title_bar
    }
}

fn control_btn(
    id: &'static str,
    icon: IconName,
    click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .w(TITLE_BAR_HEIGHT)
        .h_full()
        .flex_shrink_0()
        .justify_center()
        .content_center()
        .items_center()
        .cursor(CursorStyle::Arrow)
        .on_mouse_down(MouseButton::Left, |_, window, cx| {
            window.prevent_default();
            cx.stop_propagation();
        })
        .on_click(move |ev, window, cx| {
            cx.stop_propagation();
            click(ev, window, cx);
        })
        .child(Icon::new(icon).small())
}
