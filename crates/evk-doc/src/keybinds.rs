use crate::actions::*;
use crate::app::MarkdownApp;
use gpui::*;

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
            "o" => window.dispatch_action(Box::new(OpenFile), cx),
            "s" => window.dispatch_action(Box::new(SaveFile), cx),
            "t" => window.dispatch_action(Box::new(ToggleTheme), cx),
            "b" => window.dispatch_action(Box::new(ToggleTree), cx),
            _ => {}
        }
    } else if event.keystroke.modifiers.control && event.keystroke.modifiers.shift {
        match event.keystroke.key.as_str() {
            "s" => window.dispatch_action(Box::new(SaveFileAs), cx),
            "o" => window.dispatch_action(Box::new(OpenFolder), cx),
            _ => {}
        }
    }
}
