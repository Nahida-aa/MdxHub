use std::ops::Range;

use crate::{LabelLike, prelude::*};
use gpui::{HighlightStyle, StyleRefinement};
use gui::StyledText;

/// A struct representing a label element in the UI.
///
/// The `Label` struct stores the label text and common properties for a label element.
/// It provides methods for modifying these properties.
///
/// # Examples
///
/// ```
/// use ui::prelude::*;
///
/// Label::new("Hello, World!");
/// ```
///
/// **A colored label**, for example labeling a dangerous action:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Delete").color(Color::Error);
/// ```
///
/// **A label with a strikethrough**, for example labeling something that has been deleted:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Deleted").strikethrough();
/// ```
#[derive(
    IntoElement,
    // RegisterComponent
)]
pub struct Label {
    base: LabelLike,
    label: SharedString,
    render_code_spans: bool,
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.render_code_spans {
            if let Some((stripped, code_ranges)) = parse_backtick_spans(&self.label) {
                let buffer_font_family = theme::theme_settings(cx).buffer_font(cx).family.clone();
                let background_color = cx.theme().colors().element_background;

                let highlights = code_ranges.iter().map(|range| {
                    (
                        range.clone(),
                        HighlightStyle {
                            background_color: Some(background_color),
                            ..Default::default()
                        },
                    )
                });

                let font_overrides = code_ranges
                    .iter()
                    .map(|range| (range.clone(), buffer_font_family.clone()));

                return self.base.child(
                    StyledText::new(stripped)
                        .with_highlights(highlights)
                        .with_font_family_overrides(font_overrides),
                );
            }
        }
        self.base.child(self.label)
    }
}

/// Parses backtick-delimited code spans from a string.
///
/// Returns `None` if there are no matched backtick pairs.
/// Otherwise returns the text with backticks stripped and the byte ranges
/// of the code spans in the stripped string.
fn parse_backtick_spans(text: &str) -> Option<(SharedString, Vec<Range<usize>>)> {
    if !text.contains('`') {
        return None;
    }

    let mut stripped = String::with_capacity(text.len());
    let mut code_ranges = Vec::new();
    let mut in_code = false;
    let mut code_start = 0;

    for ch in text.chars() {
        if ch == '`' {
            if in_code {
                code_ranges.push(code_start..stripped.len());
            } else {
                code_start = stripped.len();
            }
            in_code = !in_code;
        } else {
            stripped.push(ch);
        }
    }

    if code_ranges.is_empty() {
        return None;
    }

    Some((SharedString::from(stripped), code_ranges))
}
