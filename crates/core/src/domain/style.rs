//! Text style for slides. All sizes are in canvas units (1920 × 1080).

use std::ops::RangeInclusive;

/// Allowed font size range.
pub const SIZE_RANGE: RangeInclusive<f32> = 8.0..=400.0;
/// Allowed line-height multiplier range.
pub const LINE_HEIGHT_RANGE: RangeInclusive<f32> = 0.8..=3.0;
/// Allowed extra letter spacing range.
pub const LETTER_SPACING_RANGE: RangeInclusive<f32> = -10.0..=50.0;

/// Bundled fonts available in v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontFamily {
    /// Noto Sans, falling back to Noto Sans SC for Chinese characters.
    #[default]
    NotoSans,
}

/// Horizontal text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAlign {
    /// Align to the left margin.
    Left,
    /// Center between the margins.
    #[default]
    Center,
    /// Align to the right margin.
    Right,
}

/// Vertical position of the text block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VAlign {
    /// Top of the safe area.
    Top,
    /// Middle of the safe area.
    #[default]
    Middle,
    /// Bottom of the safe area.
    Bottom,
}

/// An sRGB color with alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    /// Red.
    pub r: u8,
    /// Green.
    pub g: u8,
    /// Blue.
    pub b: u8,
    /// Alpha (255 = opaque).
    pub a: u8,
}

impl Rgba {
    /// Opaque black.
    pub const BLACK: Self = Self::opaque(0, 0, 0);
    /// Opaque white.
    pub const WHITE: Self = Self::opaque(255, 255, 255);

    /// Creates an opaque color.
    pub const fn opaque(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

/// Complete text style of a song or template.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    /// Font family.
    pub font: FontFamily,
    /// Font size, within [`SIZE_RANGE`].
    pub size: f32,
    /// Line-height multiplier, within [`LINE_HEIGHT_RANGE`].
    pub line_height: f32,
    /// Extra letter spacing, within [`LETTER_SPACING_RANGE`].
    pub letter_spacing: f32,
    /// Horizontal alignment.
    pub h_align: HAlign,
    /// Vertical position.
    pub v_align: VAlign,
    /// Text color.
    pub text_color: Rgba,
    /// Solid background color.
    pub background: Rgba,
}

impl Default for TextStyle {
    /// The built-in "Centered Lyrics" style.
    fn default() -> Self {
        Self {
            font: FontFamily::NotoSans,
            size: 72.0,
            line_height: 1.2,
            letter_spacing: 0.0,
            h_align: HAlign::Center,
            v_align: VAlign::Middle,
            text_color: Rgba::WHITE,
            background: Rgba::BLACK,
        }
    }
}

impl TextStyle {
    /// Checks every numeric field against its allowed range.
    pub fn validate(&self) -> Result<(), crate::CoreError> {
        todo!("T4.1: validate style ranges")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_style_is_within_ranges() {
        let s = TextStyle::default();
        assert!(SIZE_RANGE.contains(&s.size));
        assert!(LINE_HEIGHT_RANGE.contains(&s.line_height));
        assert!(LETTER_SPACING_RANGE.contains(&s.letter_spacing));
    }
}
