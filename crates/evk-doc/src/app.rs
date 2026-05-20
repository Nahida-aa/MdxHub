use gpui::*;
use gpui_component::{
    input::{Input, InputEvent, InputState},
    text::TextView,
    ActiveTheme,
};

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

pub struct MarkdownApp {
    editor: Entity<InputState>,
    markdown_content: SharedString,
    _subscriptions: Vec<Subscription>,
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
            _subscriptions,
        }
    }
}

impl Render for MarkdownApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg = cx.theme().colors.background;
        let divider = gpui_component::gray_400();

        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(bg)
            .child(
                div()
                    .flex_1()
                    .min_w(px(300.))
                    .child(Input::new(&self.editor).h_full().bordered(false).appearance(false)),
            )
            .child(div().w(px(1.)).bg(divider))
            .child(
                div()
                    .flex_1()
                    .min_w(px(300.))
                    .p_4()
                    .child(
                        TextView::markdown("preview", self.markdown_content.clone(), window, cx)
                            .scrollable(true),
                    ),
            )
    }
}


