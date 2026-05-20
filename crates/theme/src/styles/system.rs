use gpui::Hsla;

#[derive(Debug, Clone, PartialEq)]
pub struct SystemColors {
    pub transparent: Hsla,
}

impl Default for SystemColors {
    fn default() -> Self {
        Self {
            transparent: Hsla::transparent_black(),
        }
    }
}
