use crate::prelude::*;
use gpui::{
    AnyElement, App, Div, FontWeight, Rems, RenderOnce, StyleRefinement, UnderlineStyle, Window,
};
use smallvec::SmallVec;

use crate::Color;

/// Sets the size of a label
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum LabelSize {
    /// The default size of a label.
    #[default]
    Default,
    /// The large size of a label.
    Large,
    /// The small size of a label.
    Small,
    /// The extra small size of a label.
    XSmall,
    /// An arbitrary custom size specified in rems.
    Custom(Rems),
}

/// Sets the line height of a label
#[derive(Default, PartialEq, Copy, Clone)]
pub enum LineHeightStyle {
    /// The default line height style of a label,
    /// set by either the UI's default line height,
    /// or the developer's default buffer line height.
    #[default]
    TextLabel,
    /// Sets the line height to 1.
    UiLabel,
}

/// A label-like element that can be used to create a custom label when
/// prebuilt labels are not sufficient. Use this sparingly, as it is
/// unconstrained and may make the UI feel less consistent.
///
/// This is also used to build the prebuilt labels.
#[derive(IntoElement)]
pub struct LabelLike {
    pub(super) base: Div,
    size: LabelSize,
    weight: Option<FontWeight>,
    line_height_style: LineHeightStyle,
    pub(crate) color: Color,
    strikethrough: bool,
    italic: bool,
    children: SmallVec<[AnyElement; 2]>,
    alpha: Option<f32>,
    underline: bool,
    single_line: bool,
    truncate: bool,
    truncate_start: bool,
}

impl ParentElement for LabelLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for LabelLike {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut color = self.color.color(cx);
        if let Some(alpha) = self.alpha {
            color.fade_out(1.0 - alpha);
        }

        self.base
            .map(|this| match self.size {
                LabelSize::Large => this.text_ui_lg(cx),
                LabelSize::Default => this.text_ui(cx),
                LabelSize::Small => this.text_ui_sm(cx),
                LabelSize::XSmall => this.text_ui_xs(cx),
                LabelSize::Custom(size) => this.text_size(size),
            })
            .when(self.line_height_style == LineHeightStyle::UiLabel, |this| {
                this.line_height(relative(1.))
            })
            .when(self.italic, |this| this.italic())
            .when(self.underline, |mut this| {
                this.text_style()
                    .get_or_insert_with(Default::default)
                    .underline = Some(UnderlineStyle {
                    thickness: px(1.),
                    color: Some(cx.theme().colors().text_muted.opacity(0.4)),
                    wavy: false,
                });
                this
            })
            .when(self.strikethrough, |this| this.line_through())
            .when(self.single_line, |this| this.whitespace_nowrap())
            .when(self.truncate, |this| {
                this.min_w_0()
                    .overflow_x_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
            })
            .when(self.truncate_start, |this| {
                this.min_w_0()
                    .overflow_x_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
            })
            .text_color(color)
            .font_weight(
                self.weight
                    .unwrap_or(theme::theme_settings(cx).ui_font(cx).weight),
            )
            .children(self.children)
    }
}
