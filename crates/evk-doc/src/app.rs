use crate::actions::*;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    input::{Input, InputEvent, InputState},
    menu::AppMenuBar,
    text::TextView,
    tree::TreeState,
    ActiveTheme,
};
use std::path::PathBuf;
use title_bar::TitleBar;

const DIVIDER_W: f32 = 8.;
const MIN_PANEL_W: f32 = 200.;
const SPLIT_MIN: f32 = 0.15;
const SPLIT_MAX: f32 = 0.85;
const TREE_W: f32 = 250.;

pub struct MarkdownApp {
    pub editor: Entity<InputState>,
    pub markdown_content: SharedString,
    pub current_path: Option<PathBuf>,
    pub split_ratio: f32,
    pub is_dragging: bool,
    _subscriptions: Vec<Subscription>,
    pub tree_state: Option<Entity<TreeState>>,
    pub tree_visible: bool,
    pub pending_tree_open: Option<PathBuf>,
    pub open_folder_path: Option<PathBuf>,
    pub menu_bar: Entity<AppMenuBar>,
    pub title_bar: Entity<TitleBar>,
}

impl MarkdownApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let menu_bar = AppMenuBar::new(window, cx);

        let title_bar = cx.new(|cx| {
            TitleBar::new([menu_bar.clone().into_any_element()], window, cx)
        });

        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("markdown")
                .line_number(true)
        });

        editor.update(cx, |editor, cx| {
            editor.set_value(crate::constants::DEMO_MD, window, cx);
        });

        let markdown_content: SharedString = crate::constants::DEMO_MD.into();
        let _subscriptions = vec![cx.subscribe_in(&editor, window, {
            let editor = editor.clone();
            move |this, _, ev: &InputEvent, _window, cx| {
                if matches!(ev, InputEvent::Change) {
                    let value = editor.read(cx).value();
                    this.markdown_content = value;
                    cx.notify();
                }
            }
        })];

        Self {
            editor,
            markdown_content,
            current_path: None,
            split_ratio: 0.5,
            is_dragging: false,
            _subscriptions,
            tree_state: None,
            tree_visible: false,
            pending_tree_open: None,
            open_folder_path: None,
            menu_bar,
            title_bar,
        }
    }

    fn start_drag(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_dragging = true;
        cx.notify();
    }

    fn handle_open_file(&mut self, _: &OpenFile, window: &mut Window, cx: &mut Context<Self>) {
        crate::file_ops::open_file(self, window, cx);
    }

    fn handle_save_file(&mut self, _: &SaveFile, _window: &mut Window, cx: &mut Context<Self>) {
        crate::file_ops::save_file(self, _window, cx);
    }

    fn handle_save_file_as(
        &mut self,
        _: &SaveFileAs,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        crate::file_ops::save_file_as(self, window, cx);
    }

    fn handle_open_folder(&mut self, _: &OpenFolder, window: &mut Window, cx: &mut Context<Self>) {
        crate::tree::open_folder(window, cx);
    }

    fn handle_toggle_theme(
        &mut self,
        _: &ToggleTheme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let theme = cx.theme();
        let new_mode = if theme.is_dark() {
            gpui_component::ThemeMode::Light
        } else {
            gpui_component::ThemeMode::Dark
        };
        gpui_component::Theme::change(new_mode, Some(window), cx);
    }

    fn handle_set_theme_dark(
        &mut self,
        _: &SetThemeDark,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        gpui_component::Theme::change(gpui_component::ThemeMode::Dark, Some(window), cx);
    }

    fn handle_set_theme_light(
        &mut self,
        _: &SetThemeLight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        gpui_component::Theme::change(gpui_component::ThemeMode::Light, Some(window), cx);
    }

    fn handle_toggle_tree(&mut self, _: &ToggleTree, _window: &mut Window, cx: &mut Context<Self>) {
        crate::tree::toggle_tree(self);
        cx.notify();
    }

    fn handle_about(&mut self, _: &AboutEvk, _window: &mut Window, cx: &mut Context<Self>) {
        let bounds = Bounds::centered(None, size(px(320.), px(200.)), &**cx);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("About Evk".into()),
                    ..TitlebarOptions::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                is_resizable: false,
                is_minimizable: false,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| AboutWindow { focus: cx.focus_handle() }),
        )
        .unwrap();
    }
}

impl Render for MarkdownApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let menu_bar_element = self.menu_bar.clone().into_any_element();
        self.title_bar.update(cx, |tb, cx| {
            tb.set_children([menu_bar_element], cx);
        });

        if let Some(path) = self.pending_tree_open.take() {
            match std::fs::read_to_string(&path) {
                Ok(text) => {
                    self.editor.update(cx, |ed, cx| {
                        ed.set_value(&text, window, cx);
                    });
                    self.markdown_content = text.into();
                    self.current_path = Some(path);
                }
                Err(err) => eprintln!("Error opening file from tree: {}", err),
            }
        }

        if self.is_dragging {
            let entity = cx.entity();
            let entity2 = entity.clone();
            window.on_mouse_event(move |ev: &MouseMoveEvent, _phase, window, cx| {
                if ev.pressed_button == Some(MouseButton::Left) {
                    let total = window.bounds().size.width;
                    let ratio = f32::from(ev.position.x) / f32::from(total);
                    let _ = entity.update(cx, |this, cx| {
                        this.split_ratio = ratio.clamp(SPLIT_MIN, SPLIT_MAX);
                        cx.notify();
                    });
                }
            });

            window.on_mouse_event(move |ev: &MouseUpEvent, _phase, _window, cx| {
                if ev.button == MouseButton::Left {
                    let _ = entity2.update(cx, |this, cx| {
                        this.is_dragging = false;
                        cx.notify();
                    });
                }
            });
        }

        let bg = cx.theme().colors.background;
        let divider_color = if self.is_dragging {
            gpui_component::blue_500()
        } else {
            gpui_component::gray_400()
        };

        let total_f32: f32 = f32::from(window.bounds().size.width);
        let avail_f32 = if self.tree_visible {
            total_f32 - TREE_W
        } else {
            total_f32
        };
        let left_w = px((avail_f32 - DIVIDER_W) * self.split_ratio);
        let min_panel = px(MIN_PANEL_W);

        let tree_sidebar = if self.tree_visible {
            self.tree_state.as_ref().map(|state| {
                let entity = cx.entity();
                crate::tree::render_tree_sidebar(
                    state,
                    &self.open_folder_path,
                    move |path, cx| {
                        let _ = entity.update(cx, |this: &mut MarkdownApp, cx| {
                            this.pending_tree_open = Some(path);
                            cx.notify();
                        });
                    },
                    cx.theme().colors.sidebar,
                    cx.theme().colors.sidebar_foreground,
                )
            })
        } else {
            None
        };

        div()
            .id("root")
            .capture_key_down(cx.listener(crate::keybinds::handle_key_down))
            .on_action(cx.listener(Self::handle_open_file))
            .on_action(cx.listener(Self::handle_save_file))
            .on_action(cx.listener(Self::handle_save_file_as))
            .on_action(cx.listener(Self::handle_open_folder))
            .on_action(cx.listener(Self::handle_toggle_theme))
            .on_action(cx.listener(Self::handle_set_theme_dark))
            .on_action(cx.listener(Self::handle_set_theme_light))
            .on_action(cx.listener(Self::handle_toggle_tree))
            .on_action(cx.listener(Self::handle_about))
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg)
            .child(self.title_bar.clone().into_any_element())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .overflow_hidden()
                    .when_some(tree_sidebar, |content, sidebar| {
                        content.child(sidebar)
                    })
                    .child(
                        div()
                            .flex_1()
                            .w(left_w)
                            .min_w(min_panel)
                            .child(
                                Input::new(&self.editor)
                                    .h_full()
                                    .bordered(false)
                                    .appearance(false),
                            ),
                    )
                    .child(
                        div()
                            .id("divider")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _ev: &MouseDownEvent, window, cx| {
                                    this.start_drag(window, cx);
                                }),
                            )
                            .w(px(DIVIDER_W))
                            .h_full()
                            .bg(divider_color)
                            .hover(|s| s.cursor(CursorStyle::ResizeLeftRight)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(min_panel)
                            .p_4()
                            .child(
                                TextView::markdown(
                                    "preview",
                                    self.markdown_content.clone(),
                                    window,
                                    cx,
                                )
                                .scrollable(true),
                            ),
                    ),
            )
    }
}

pub struct AboutWindow {
    focus: FocusHandle,
}

impl Focusable for AboutWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for AboutWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("about")
            .track_focus(&self.focus)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(cx.theme().colors.background)
            .child(
                div()
                    .text_xl()
                    .child("Evk"),
            )
            .child(
                div()
                    .text_color(cx.theme().colors.muted_foreground)
                    .child(format!("Version {}", env!("CARGO_PKG_VERSION"))),
            )
    }
}
