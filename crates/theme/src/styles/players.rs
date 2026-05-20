use gpui::Hsla;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub background: Hsla,
    pub selection: Hsla,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerColors(pub Vec<PlayerColor>);

impl PlayerColors {
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn local(&self) -> PlayerColor {
        self.0.first().copied().unwrap_or(PlayerColor {
            cursor: Hsla::transparent_black(),
            background: Hsla::transparent_black(),
            selection: Hsla::transparent_black(),
        })
    }
}
