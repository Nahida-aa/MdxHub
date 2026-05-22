use std::time::Duration;

use crate::animation_ext::CommonAnimationExt as _;
use crate::component_prelude::*;
use crate::{
    ButtonLike, Color, DynamicSpacing, KeybindingPosition, LabelSize,
    clickable::Clickable,
    disableable::Disableable,
    icon::{Icon, IconSize},
    label::Label,
    prelude::*,
    toggleable::Toggleable,
};
use gpui::{
    Animation, AnimationExt, App, ClickEvent, CursorStyle, Div, IntoElement, KeyBinding,
    ParentElement, RenderOnce, SharedString, Styled as _, Transform, Window,
    prelude::FluentBuilder as _,
};
use gpui_component::{IconName, Selectable, Sizable, Size, h_flex};
use ui_macros::RegisterComponent;

/// An element that creates a button with a label and optional icons.
///
/// Common buttons:
/// - Label, Icon + Label: [`Button`] (this component)
/// - Icon only: [`IconButton`]
/// - Custom: [`ButtonLike`]
///
/// To create a more complex button than what the [`Button`] or [`IconButton`] components provide, use
/// [`ButtonLike`] directly.
///
/// # Examples
///
/// **A button with a label**, is typically used in scenarios such as a form, where the button's label
/// indicates what action will be performed when the button is clicked.
///
/// ```
/// use ui::prelude::*;
///
/// Button::new("button_id", "Click me!")
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
///
/// **A toggleable button**, is typically used in scenarios such as a toolbar,
/// where the button's state indicates whether a feature is enabled or not, or
/// a trigger for a popover menu, where clicking the button toggles the visibility of the menu.
///
/// ```
/// use ui::prelude::*;
///
/// Button::new("button_id", "Click me!")
///     .start_icon(Icon::new(IconName::Check))
///     .toggle_state(true)
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
///
/// To change the style of the button when it is selected use the [`selected_style`][Button::selected_style] method.
///
/// ```
/// use ui::prelude::*;
/// use ui::TintColor;
///
/// Button::new("button_id", "Click me!")
///     .toggle_state(true)
///     .selected_style(ButtonStyle::Tinted(TintColor::Accent))
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
/// This will create a button with a blue tinted background when selected.
///
/// **A full-width button**, is typically used in scenarios such as the bottom of a modal or form, where it occupies the entire width of its container.
/// The button's content, including text and icons, is centered by default.
///
/// ```
/// use ui::prelude::*;
///
/// let button = Button::new("button_id", "Click me!")
///     .full_width()
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
///
#[derive(
    IntoElement,
    // Documented,
    // RegisterComponent,
)]
pub struct Button {
    base: ButtonLike,
    label: SharedString,
    label_color: Option<Color>,
    label_size: Option<LabelSize>,
    selected_label: Option<SharedString>,
    selected_label_color: Option<Color>,
    start_icon: Option<Icon>,
    end_icon: Option<Icon>,
    key_binding: Option<KeyBinding>,
    key_binding_position: KeybindingPosition,
    alpha: Option<f32>,
    truncate: bool,
    loading: bool,
}

impl Toggleable for Button {
    /// Sets the selected state of the button.
    ///
    /// # Examples
    ///
    /// Create a toggleable button that changes appearance when selected:
    ///
    /// ```
    /// use ui::prelude::*;
    /// use ui::TintColor;
    ///
    /// let selected = true;
    ///
    /// Button::new("toggle_button", "Toggle Me")
    ///     .start_icon(Icon::new(IconName::Check))
    ///     .toggle_state(selected)
    ///     .selected_style(ButtonStyle::Tinted(TintColor::Accent))
    ///     .on_click(|event, window, cx| {
    ///         // Toggle the selected state
    ///     });
    /// ```
    fn toggle_state(mut self, selected: bool) -> Self {
        self.base = self.base.toggle_state(selected);
        self
    }
}

// impl Clickable for ButtonLike {
//     fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
//         self.on_click = Some(Box::new(handler));
//         self
//     }

//     fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
//         self.cursor_style = cursor_style;
//         self
//     }
// }
impl RenderOnce for Button {
    #[allow(refining_impl_trait)]
    fn render(self, _window: &mut Window, cx: &mut App) -> ButtonLike {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;

        let label = self
            .selected_label
            .filter(|_| is_selected)
            .unwrap_or(self.label);

        let label_color = if is_disabled {
            Color::Disabled
        } else if is_selected {
            self.selected_label_color.unwrap_or(Color::Selected)
        } else {
            self.label_color.unwrap_or_default()
        };

        self.base.child(
            h_flex()
                .when(self.truncate, |this| this.min_w_0().overflow_hidden())
                .gap(DynamicSpacing::Base04.rems(cx))
                .when_else(
                    self.loading,
                    |this| {
                        this.child(
                            Icon::new(IconName::LoaderCircle)
                                .size(IconSize::Small)
                                .color(Color::Muted)
                                .with_rotate_animation(2),
                        )
                    },
                    |this| {
                        this.when_some(self.start_icon, |this, icon| {
                            this.child(if is_disabled {
                                icon.color(Color::Disabled)
                            } else {
                                icon
                            })
                        })
                    },
                )
                .child(
                    h_flex()
                        .when(self.truncate, |this| this.min_w_0().overflow_hidden())
                        .when(
                            self.key_binding_position == KeybindingPosition::Start,
                            |this| this.flex_row_reverse(),
                        )
                        .gap(DynamicSpacing::Base06.rems(cx))
                        .justify_between()
                        .child(
                            Label::new(label)
                                .color(label_color)
                                .size(self.label_size.unwrap_or_default())
                                .when_some(self.alpha, |this, alpha| this.alpha(alpha))
                                .when(self.truncate, |this| this.truncate()),
                        )
                        .children(self.key_binding),
                )
                .when_some(self.end_icon, |this, icon| {
                    this.child(if is_disabled {
                        icon.color(Color::Disabled)
                    } else {
                        icon
                    })
                }),
        )
    }
}

// impl Component for Button {
//     fn scope() -> ComponentScope {
//         ComponentScope::Input
//     }

//     fn sort_name() -> &'static str {
//         "ButtonA"
//     }

//     fn description() -> Option<&'static str> {
//         Some("A button triggers an event or action.")
//     }

//     fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
//         Some(
//             v_flex()
//                 .gap_6()
//                 .children(vec![
//                     example_group_with_title(
//                         "Button Styles",
//                         vec![
//                             single_example(
//                                 "Default",
//                                 Button::new("default", "Default").into_any_element(),
//                             ),
//                             single_example(
//                                 "Filled",
//                                 Button::new("filled", "Filled")
//                                     .style(ButtonStyle::Filled)
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Subtle",
//                                 Button::new("outline", "Subtle")
//                                     .style(ButtonStyle::Subtle)
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Tinted",
//                                 Button::new("tinted_accent_style", "Accent")
//                                     .style(ButtonStyle::Tinted(TintColor::Accent))
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Transparent",
//                                 Button::new("transparent", "Transparent")
//                                     .style(ButtonStyle::Transparent)
//                                     .into_any_element(),
//                             ),
//                         ],
//                     ),
//                     example_group_with_title(
//                         "Tint Styles",
//                         vec![
//                             single_example(
//                                 "Accent",
//                                 Button::new("tinted_accent", "Accent")
//                                     .style(ButtonStyle::Tinted(TintColor::Accent))
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Error",
//                                 Button::new("tinted_negative", "Error")
//                                     .style(ButtonStyle::Tinted(TintColor::Error))
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Warning",
//                                 Button::new("tinted_warning", "Warning")
//                                     .style(ButtonStyle::Tinted(TintColor::Warning))
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Success",
//                                 Button::new("tinted_positive", "Success")
//                                     .style(ButtonStyle::Tinted(TintColor::Success))
//                                     .into_any_element(),
//                             ),
//                         ],
//                     ),
//                     example_group_with_title(
//                         "Special States",
//                         vec![
//                             single_example(
//                                 "Default",
//                                 Button::new("default_state", "Default").into_any_element(),
//                             ),
//                             single_example(
//                                 "Disabled",
//                                 Button::new("disabled", "Disabled")
//                                     .disabled(true)
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Selected",
//                                 Button::new("selected", "Selected")
//                                     .toggle_state(true)
//                                     .into_any_element(),
//                             ),
//                         ],
//                     ),
//                     example_group_with_title(
//                         "Buttons with Icons",
//                         vec![
//                             single_example(
//                                 "Start Icon",
//                                 Button::new("icon_start", "Start Icon")
//                                     .start_icon(Icon::new(IconName::Check))
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "End Icon",
//                                 Button::new("icon_end", "End Icon")
//                                     .end_icon(Icon::new(IconName::Check))
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Both Icons",
//                                 Button::new("both_icons", "Both Icons")
//                                     .start_icon(Icon::new(IconName::Check))
//                                     .end_icon(Icon::new(IconName::ChevronDown))
//                                     .into_any_element(),
//                             ),
//                             single_example(
//                                 "Icon Color",
//                                 Button::new("icon_color", "Icon Color")
//                                     .start_icon(Icon::new(IconName::Check).color(Color::Accent))
//                                     .into_any_element(),
//                             ),
//                         ],
//                     ),
//                 ])
//                 .into_any_element(),
//         )
//     }
// }
