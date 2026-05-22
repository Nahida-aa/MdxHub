use gpui::Hsla;
use std::sync::Arc;

use super::syntax::SyntaxTheme;
use super::{AccentColors, PlayerColors, StatusColors, SystemColors};

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeStyles {
    pub window_background_appearance: WindowBackgroundAppearance,
    pub system: SystemColors,
    pub colors: ThemeColors,
    pub accents: AccentColors,
    pub status: StatusColors,
    pub player: PlayerColors,
    pub syntax: Arc<SyntaxTheme>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowBackgroundAppearance {
    #[default]
    Opaque,
    Transparent,
    Blurred,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeColors {
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_selected: Hsla,
    pub border_transparent: Hsla,
    pub border_disabled: Hsla,
    pub elevated_surface_background: Hsla,
    pub surface_background: Hsla,
    pub background: Hsla,
    pub element_background: Hsla,
    pub element_hover: Hsla,
    pub element_active: Hsla,
    pub element_selected: Hsla,
    pub element_disabled: Hsla,
    pub drop_target_background: Hsla,
    pub ghost_element_background: Hsla,
    pub ghost_element_hover: Hsla,
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,
    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_disabled: Hsla,
    pub icon_placeholder: Hsla,
    pub icon_accent: Hsla,
    /// Color used to accent some debugger elements
    /// Is used by breakpoints
    pub debugger_accent: Hsla,

    pub status_bar_background: Hsla,
    pub title_bar_background: Hsla,
    pub title_bar_inactive_background: Hsla,
    pub toolbar_background: Hsla,
    pub tab_bar_background: Hsla,
    pub tab_inactive_background: Hsla,
    pub tab_active_background: Hsla,
    pub search_match_background: Hsla,
    pub search_active_match_background: Hsla,
    pub panel_background: Hsla,
    pub panel_focused_border: Hsla,
    pub pane_focused_border: Hsla,
    pub scrollbar_thumb_background: Hsla,
    pub scrollbar_thumb_hover_background: Hsla,
    pub scrollbar_thumb_border: Hsla,
    pub scrollbar_track_background: Hsla,
    pub scrollbar_track_border: Hsla,
    pub editor_foreground: Hsla,
    pub editor_background: Hsla,
    pub editor_gutter_background: Hsla,
    pub editor_subheader_background: Hsla,
    pub editor_active_line_background: Hsla,
    pub editor_highlighted_line_background: Hsla,
    pub editor_line_number: Hsla,
    pub editor_active_line_number: Hsla,
    pub editor_hover_line_number: Hsla,
    pub editor_invisible: Hsla,
    pub editor_wrap_guide: Hsla,
    pub editor_active_wrap_guide: Hsla,
    pub editor_document_highlight_read_background: Hsla,
    pub editor_document_highlight_write_background: Hsla,
    pub terminal_background: Hsla,
    pub terminal_foreground: Hsla,
    pub terminal_bright_foreground: Hsla,
    pub terminal_dim_foreground: Hsla,
    pub terminal_ansi_black: Hsla,
    pub terminal_ansi_bright_black: Hsla,
    pub terminal_ansi_dim_black: Hsla,
    pub terminal_ansi_red: Hsla,
    pub terminal_ansi_bright_red: Hsla,
    pub terminal_ansi_dim_red: Hsla,
    pub terminal_ansi_green: Hsla,
    pub terminal_ansi_bright_green: Hsla,
    pub terminal_ansi_dim_green: Hsla,
    pub terminal_ansi_yellow: Hsla,
    pub terminal_ansi_bright_yellow: Hsla,
    pub terminal_ansi_dim_yellow: Hsla,
    pub terminal_ansi_blue: Hsla,
    pub terminal_ansi_bright_blue: Hsla,
    pub terminal_ansi_dim_blue: Hsla,
    pub terminal_ansi_magenta: Hsla,
    pub terminal_ansi_bright_magenta: Hsla,
    pub terminal_ansi_dim_magenta: Hsla,
    pub terminal_ansi_cyan: Hsla,
    pub terminal_ansi_bright_cyan: Hsla,
    pub terminal_ansi_dim_cyan: Hsla,
    pub terminal_ansi_white: Hsla,
    pub terminal_ansi_bright_white: Hsla,
    pub terminal_ansi_dim_white: Hsla,
    pub link_text_hover: Hsla,

    /// Represents an added entry or hunk in vcs, like git.
    pub version_control_added: Hsla,
    /// Represents a deleted entry in version control systems.
    pub version_control_deleted: Hsla,
    /// Represents a modified entry in version control systems.
    pub version_control_modified: Hsla,
    /// Represents a renamed entry in version control systems.
    pub version_control_renamed: Hsla,
    /// Represents a conflicting entry in version control systems.
    pub version_control_conflict: Hsla,
    /// Represents an ignored entry in version control systems.
    pub version_control_ignored: Hsla,
    /// Represents an added word in a word diff.
    pub version_control_word_added: Hsla,
    /// Represents a deleted word in a word diff.
    pub version_control_word_deleted: Hsla,
    /// Represents the "ours" region of a merge conflict.
    pub version_control_conflict_marker_ours: Hsla,
    /// Represents the "theirs" region of a merge conflict.
    pub version_control_conflict_marker_theirs: Hsla,
}
