use crate::app::MarkdownApp;
use gpui::*;
use gpui_component::{ActiveTheme, Theme, ThemeMode};

fn toggle_theme(window: &mut Window, cx: &mut Context<MarkdownApp>) {
    let theme = cx.theme();
    let new_mode = if theme.is_dark() {
        ThemeMode::Light
    } else {
        ThemeMode::Dark
    };
    Theme::change(new_mode, Some(window), cx);
}

pub fn handle_key_down(
    this: &mut MarkdownApp,
    event: &KeyDownEvent,
    window: &mut Window,
    cx: &mut Context<MarkdownApp>,
) {
    if this.is_dragging {
        return;
    }

    if event.keystroke.modifiers.control && !event.keystroke.modifiers.shift {
        match event.keystroke.key.as_str() {
            "o" => crate::file_ops::open_file(this, window, cx),
            "s" => crate::file_ops::save_file(this, window, cx),
            "t" => toggle_theme(window, cx),
            "b" => {
                crate::tree::toggle_tree(this);
                cx.notify();
            }
            _ => {}
        }
    } else if event.keystroke.modifiers.control && event.keystroke.modifiers.shift {
        match event.keystroke.key.as_str() {
            "s" => crate::file_ops::save_file_as(this, window, cx),
            "O" => crate::tree::open_folder(window, cx),
            _ => {}
        }
    }
}
