//! The measuring interface layout depends on. The app implements it with
//! egui's font system; tests use a fixed-width fake.

use crate::domain::TextStyle;

/// Measures text in canvas units.
pub trait TextMeasurer {
    /// Width of `text` on one line, including letter spacing.
    fn line_width(&self, text: &str, style: &TextStyle) -> f32;

    /// Vertical advance per line (font metrics × `style.line_height`).
    fn line_advance(&self, style: &TextStyle) -> f32;
}
