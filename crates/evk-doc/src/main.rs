mod app;
mod theme;

use app::MarkdownApp;
use gpui::*;
use gpui_component::Root;

fn main() {
    Application::new().run(|cx| {
        gpui_component::init(cx);
        theme::init(cx);

        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            let view = cx.new(|cx| MarkdownApp::new(window, cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
