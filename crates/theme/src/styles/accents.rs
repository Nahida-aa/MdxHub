use gpui::Hsla;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct AccentColors(pub Arc<[Hsla]>);

impl AccentColors {
    pub fn empty() -> Self {
        Self(Arc::new([]))
    }

    pub fn color_for_index(&self, index: u32) -> Hsla {
        if self.0.is_empty() {
            return Hsla::transparent_black();
        }
        self.0[index as usize % self.0.len()]
    }
}
