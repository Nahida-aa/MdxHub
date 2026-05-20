pub fn try_parse_color(color: &str) -> anyhow::Result<gpui::Hsla> {
    let rgba = gpui::Rgba::try_from(color)
        .map_err(|e| anyhow::anyhow!("invalid RGBA hex color: '{color}'. {e}"))?;
    Ok(rgba.into())
}
