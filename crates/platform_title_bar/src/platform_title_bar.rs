mod system_window_tabs;
use crate::{
    // platforms::{platform_linux, platform_windows},
    system_window_tabs::SystemWindowTabs,
};
use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName, InteractiveElementExt, Sizable, h_flex};
use gui::WindowButtonLayout;
use project::DisableAiSettings;
use settings::Settings as _;
use smallvec::SmallVec;
use ui::PlatformStyle;
use workspace::MultiWorkspace;

pub struct PlatformTitleBar {
    id: ElementId,
    platform_style: PlatformStyle,
    children: SmallVec<[AnyElement; 2]>,
    should_move: bool,
    system_window_tabs: Entity<SystemWindowTabs>,
    button_layout: Option<WindowButtonLayout>,
    multi_workspace: Option<WeakEntity<MultiWorkspace>>,
}
impl PlatformTitleBar {
    pub fn new(id: impl Into<ElementId>, cx: &mut Context<Self>) -> Self {
        let platform_style = PlatformStyle::platform();
        let system_window_tabs = cx.new(|_cx| SystemWindowTabs::new());

        Self {
            id: id.into(),
            platform_style,
            children: SmallVec::new(),
            should_move: false,
            system_window_tabs,
            button_layout: None,
            multi_workspace: None,
        }
    }
    pub fn set_children<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = AnyElement>,
    {
        self.children = children.into_iter().collect();
    }
    pub fn set_button_layout(&mut self, button_layout: Option<WindowButtonLayout>) {
        self.button_layout = button_layout;
    }

    pub fn with_multi_workspace(mut self, multi_workspace: WeakEntity<MultiWorkspace>) -> Self {
        self.multi_workspace = Some(multi_workspace);
        self
    }
    pub fn init(cx: &mut App) {
        SystemWindowTabs::init(cx);
    }
    pub fn is_multi_workspace_enabled(cx: &App) -> bool {
        !DisableAiSettings::get_global(cx).disable_ai
    }
}

pub const TITLE_BAR_HEIGHT: Pixels = px(34.);

pub fn init(_cx: &mut App) {}

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
        title_bar =
            title_bar.on_mouse_down_out(cx.listener(|this, _: &MouseDownEvent, _window, _cx| {
                this.should_move = false;
            }));
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
        title_bar = title_bar.on_mouse_move(cx.listener(|this, _: &MouseMoveEvent, window, _| {
            if this.should_move {
                this.should_move = false;
                window.start_window_move();
            }
        }));

        title_bar = title_bar
            .on_double_click(|_, window, _| {
                window.zoom_window();
            })
            .pl(px(12.))
            .bg(cx.theme().title_bar)
            .border_b_1()
            .border_color(cx.theme().title_bar_border)
            .child(h_flex().id("bar").flex_1().h_full().children(children));

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
