mod actions;
mod app;
mod assets;
mod constants;
mod file_ops;
mod keybinds;
mod tree;

use actions::*;
use app::MarkdownApp;
use gpui::*;
use gpui_component::Root;

fn main() {
    Application::new()
        .with_assets(crate::assets::AppAssetSource)
        .run(|cx| {
            gpui_component::init(cx);
            theme::init(cx);
            theme_settings::init(cx);

            cx.on_action(|_: &Quit, cx| cx.quit());

            cx.set_menus(vec![
                Menu {
                    name: "evk".into(),
                    items: vec![
                        MenuItem::action("About Evk", AboutEvk),
                        MenuItem::separator(),
                        MenuItem::submenu(Menu {
                            name: "Theme".into(),
                            items: vec![
                                MenuItem::action("Aa Dark", SetThemeDark),
                                MenuItem::action("Aa Light", SetThemeLight),
                                MenuItem::separator(),
                                MenuItem::action("Toggle Theme", ToggleTheme),
                            ],
                        }),
                        MenuItem::separator(),
                        MenuItem::action("Quit evk", Quit),
                    ],
                },
                Menu {
                    name: "File".into(),
                    items: vec![
                        MenuItem::action("Open File", OpenFile),
                        MenuItem::action("Open Folder", OpenFolder),
                        MenuItem::separator(),
                        MenuItem::action("Save", SaveFile),
                        MenuItem::action("Save As...", SaveFileAs),
                    ],
                },
                Menu {
                    name: "View".into(),
                    items: vec![
                        MenuItem::action("File Tree", ToggleTree),
                    ],
                },
            ]);

            let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
            let window_options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(title_bar::title_bar_options()),
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| MarkdownApp::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .unwrap();
        });
}
