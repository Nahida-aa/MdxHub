use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    input::{Input, InputEvent, InputState},
    list::ListItem,
    text::TextView,
    tree::{Tree, TreeEntry, TreeItem, TreeState},
    ActiveTheme, Theme, ThemeMode,
};
use std::path::{Path, PathBuf};

const DEMO_MD: &str = "\
# Welcome to EVK Docs

A **markdown editor** built with GPUI.

## Features

- Edit markdown on the left
- See live preview on the right
- Syntax highlighting for code

```rust
fn greet(name: &str) -> String {
    format!(\"Hello, {}!\", name)
}
```

> Blockquote example

| Col A | Col B |
|-------|-------|
| 1     | 2     |

---

**Try editing the markdown on the left — the preview updates in real time!**
";

const DIVIDER_W: f32 = 8.;
const MIN_PANEL_W: f32 = 200.;
const SPLIT_MIN: f32 = 0.15;
const SPLIT_MAX: f32 = 0.85;
const TREE_W: f32 = 250.;

fn is_markdown(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("md" | "markdown")
    )
}

fn scan_directory(dir: &Path) -> Vec<TreeItem> {
    let mut items = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| {
            let dir_first = !e.path().is_dir();
            let name = e.file_name().to_string_lossy().to_string();
            (dir_first, name)
        });
        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            if path.is_dir() {
                let children = scan_directory(&path);
                items.push(TreeItem::new(path.display().to_string(), name).children(children));
            } else if is_markdown(&path) {
                items.push(TreeItem::new(path.display().to_string(), name));
            }
        }
    }
    items
}

pub struct MarkdownApp {
    editor: Entity<InputState>,
    markdown_content: SharedString,
    current_path: Option<PathBuf>,
    split_ratio: f32,
    is_dragging: bool,
    _subscriptions: Vec<Subscription>,
    tree_state: Option<Entity<TreeState>>,
    tree_visible: bool,
    pending_tree_open: Option<PathBuf>,
    open_folder_path: Option<PathBuf>,
}

impl MarkdownApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("markdown")
                .line_number(true)
        });

        editor.update(cx, |editor, cx| {
            editor.set_value(DEMO_MD, window, cx);
        });

        let markdown_content: SharedString = DEMO_MD.into();
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
        }
    }

    fn open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: None,
        });
        let app_entity = cx.entity();
        let editor = self.editor.clone();

        window
            .spawn(cx, async move |cx| {
                if let Ok(Ok(Some(paths))) = receiver.await {
                    let path = paths[0].clone();
                    match std::fs::read_to_string(&path) {
                        Ok(text) => {
                            let _ = editor.update_in(cx, |ed, w, cx| {
                                ed.set_value(&text, w, cx);
                            });
                            let _ = app_entity.update_in(cx, |app, _w, cx| {
                                app.markdown_content = text.into();
                                app.current_path = Some(path);
                                cx.notify();
                            });
                        }
                        Err(err) => eprintln!("Error reading file: {}", err),
                    }
                }
            })
            .detach();
    }

    fn save_file(&self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = self.current_path.as_ref() {
            let content = self.editor.read(cx).value().to_string();
            let _ = std::fs::write(path, &content);
        }
    }

    fn save_file_as(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.editor.read(cx).value();
        let receiver = cx.prompt_for_new_path(std::path::Path::new(""), None);
        let app_entity = cx.entity();

        window
            .spawn(cx, async move |cx| {
                if let Ok(Ok(Some(path))) = receiver.await {
                    let _ = std::fs::write(&path, content.as_str());
                    let _ = app_entity.update_in(cx, |app, _w, cx| {
                        app.current_path = Some(path);
                        cx.notify();
                    });
                }
            })
            .detach();
    }

    fn toggle_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let theme = cx.theme();
        let new_mode = if theme.is_dark() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
        Theme::change(new_mode, Some(window), cx);
    }

    fn start_drag(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_dragging = true;
        cx.notify();
    }

    fn open_folder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: None,
        });
        let app_entity = cx.entity();
        window
            .spawn(cx, async move |cx| {
                if let Ok(Ok(Some(paths))) = receiver.await {
                    let root = paths[0].clone();
                    let items = scan_directory(&root);
                    let _ = app_entity.update_in(cx, |app, _w, cx| {
                        app.open_folder_path = Some(root);
                        app.tree_state = Some(cx.new(|cx| TreeState::new(cx).items(items)));
                        app.tree_visible = true;
                        cx.notify();
                    });
                }
            })
            .detach();
    }

    fn toggle_tree(&mut self) {
        self.tree_visible = !self.tree_visible;
    }
}

impl Render for MarkdownApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Handle pending file open from tree click
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

        // Build tree sidebar
        let tree_sidebar = if self.tree_visible {
            if let Some(ref state) = self.tree_state {
                let entity = cx.entity();
                let sbg = cx.theme().colors.sidebar;
                let tree_view = Tree::new(
                    state,
                    move |ix: usize, entry: &TreeEntry, selected: bool, _window: &mut Window, _cx: &mut App| {
                        let item = entry.item();
                        let indent = px(16.) * entry.depth() as f32;
                        let mut list_item = ListItem::new(ix)
                            .selected(selected)
                            .pl(indent)
                            .child(item.label.clone());

                        if !entry.is_folder() {
                            let entity = entity.clone();
                            let path = item.id.to_string();
                            list_item = list_item.on_click(move |_, _, cx| {
                                let path = PathBuf::from(&path);
                                let _ = entity.update(cx, |this, cx| {
                                    this.pending_tree_open = Some(path);
                                    cx.notify();
                                });
                            });
                        }
                        list_item
                    },
                );

                let root_name = self
                    .open_folder_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                Some(
                    div()
                        .id("tree-sidebar")
                        .flex_none()
                        .w(px(TREE_W))
                        .h_full()
                        .flex_col()
                        .bg(sbg)
                        .child(
                            div()
                                .px_3()
                                .py_2()
                                .text_sm()
                                .text_color(cx.theme().colors.sidebar_foreground)
                                .child(root_name),
                        )
                        .child(div().flex_1().overflow_hidden().child(tree_view.h_full())),
                )
            } else {
                None
            }
        } else {
            None
        };

        div()
            .id("root")
            .capture_key_down(cx.listener(
                |this, event: &KeyDownEvent, window, cx| {
                    if this.is_dragging {
                        return;
                    }
                    if event.keystroke.modifiers.control
                        && !event.keystroke.modifiers.shift
                    {
                        match event.keystroke.key.as_str() {
                            "o" => this.open_file(window, cx),
                            "s" => this.save_file(window, cx),
                            "t" => this.toggle_theme(window, cx),
                            "b" => {
                                this.toggle_tree();
                                cx.notify();
                            }
                            _ => {}
                        }
                    } else if event.keystroke.modifiers.control
                        && event.keystroke.modifiers.shift
                    {
                        match event.keystroke.key.as_str() {
                            "s" => this.save_file_as(window, cx),
                            "O" => this.open_folder(window, cx),
                            _ => {}
                        }
                    }
                },
            ))
            .flex()
            .flex_row()
            .size_full()
            .bg(bg)
            .when_some(tree_sidebar, |root, sidebar| root.child(sidebar))
            .child(
                div()
                    .flex_1()
                    .w(left_w)
                    .min_w(min_panel)
                    .child(Input::new(&self.editor).h_full().bordered(false).appearance(false)),
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
                        TextView::markdown("preview", self.markdown_content.clone(), window, cx)
                            .scrollable(true),
                    ),
            )
    }
}
