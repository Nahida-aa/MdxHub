use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{h_flex, ActiveTheme, Icon, IconName, InteractiveElementExt, Sizable};

const TITLE_BAR_HEIGHT: Pixels = px(34.);

pub fn title_bar_options() -> TitlebarOptions {
    TitlebarOptions {
        title: Some("Evk".into()),
        appears_transparent: true,
        traffic_light_position: Some(point(px(9.0), px(9.0))),
    }
}

pub fn render_title_bar(
    window: &mut Window,
    cx: &mut App,
    children: impl IntoIterator<Item = AnyElement>,
) -> impl IntoElement {
    let is_client = matches!(window.window_decorations(), Decorations::Client { .. });

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

    div()
        .id("title-bar")
        .flex()
        .flex_row()
        .items_center()
        .h(TITLE_BAR_HEIGHT)
        .pl(px(12.))
        .flex_shrink_0()
        .border_b_1()
        .border_color(cx.theme().title_bar_border)
        .bg(cx.theme().title_bar)
        .on_double_click(|_, window, _| {
            window.zoom_window();
        })
        .child(
            h_flex()
                .id("bar")
                .window_control_area(WindowControlArea::Drag)
                .h_full()
                .flex_1()
                .children(children),
        )
        .when(is_client, |bar| {
            bar.child(
                h_flex()
                    .id("window-controls")
                    .items_center()
                    .flex_shrink_0()
                    .h_full()
                    .child(minimize_btn)
                    .child(maximize_btn)
                    .child(close_btn),
            )
        })
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
