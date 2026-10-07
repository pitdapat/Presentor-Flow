//! `TextMeasurer` implemented with egui's font system (T0.5).
//!
//! Text is measured at a fixed reference size and scaled linearly to the
//! style's canvas size. Measuring at canvas size (up to 400) would make egui
//! rasterize huge glyphs into its texture atlas for every measurement.
//! [`text_job`] is shared with the painter, so measuring and drawing use
//! exactly the same text settings (risk R2).

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId};
use presenter_core::domain::TextStyle;
use presenter_core::layout::TextMeasurer;

use super::fonts::slide_family;

/// Font size text is measured at before scaling to the canvas.
const REFERENCE_SIZE: f32 = 32.0;

/// A single-line, non-wrapping layout job for `text` drawn at `size` points.
/// Letter spacing scales with the size, like everything else on a slide.
pub fn text_job(text: &str, style: &TextStyle, size: f32, color: Color32) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = f32::INFINITY;
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: FontId::new(size, slide_family()),
            extra_letter_spacing: style.letter_spacing * size / style.size,
            color,
            ..TextFormat::default()
        },
    );
    job
}

/// Measures text with the egui fonts installed by [`super::fonts::install`].
pub struct EguiMeasurer {
    ctx: egui::Context,
}

impl EguiMeasurer {
    /// Creates a measurer bound to the egui context.
    pub fn new(ctx: egui::Context) -> Self {
        Self { ctx }
    }

    /// Height of one row of glyphs (without line spacing) in canvas units.
    pub fn row_height(&self, style: &TextStyle) -> f32 {
        let font = FontId::new(REFERENCE_SIZE, slide_family());
        self.ctx.fonts_mut(|f| f.row_height(&font)) * style.size / REFERENCE_SIZE
    }
}

impl TextMeasurer for EguiMeasurer {
    fn line_width(&self, text: &str, style: &TextStyle) -> f32 {
        let job = text_job(text, style, REFERENCE_SIZE, Color32::WHITE);
        let width = self.ctx.fonts_mut(|f| f.layout_job(job)).size().x;
        width * style.size / REFERENCE_SIZE
    }

    fn line_advance(&self, style: &TextStyle) -> f32 {
        self.row_height(style) * style.line_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::fonts;

    /// Parity test (T0.5, risk R2): what the measurer reports at canvas size
    /// matches what the painter draws at three output scales, so lines laid
    /// out once fit identically in a thumbnail, a preview and the projector.
    #[test]
    fn measured_width_matches_drawn_width_at_three_scales() -> Result<(), String> {
        let dir = fonts::find_font_dir().ok_or("assets/fonts not found")?;
        let ctx = egui::Context::default();
        ctx.set_fonts(fonts::definitions(&dir).map_err(|e| e.to_string())?);

        let mut failures = Vec::new();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            let measurer = EguiMeasurer::new(ui.ctx().clone());
            let style = TextStyle {
                letter_spacing: 4.0,
                ..TextStyle::default()
            };
            for text in [
                "Amazing grace, how sweet the sound",
                "晨光照亮新的一天",
                "Mixed 中文 text",
            ] {
                let canvas_width = measurer.line_width(text, &style);
                // Thumbnail (220 px wide), preview (~400 px) and 1080p projector.
                for scale in [220.0 / 1920.0, 400.0 / 1920.0, 1.0] {
                    let job = text_job(text, &style, style.size * scale, Color32::WHITE);
                    let drawn = ui.ctx().fonts_mut(|f| f.layout_job(job)).size().x;
                    let expected = canvas_width * scale;
                    // Allow 2 % for glyph hinting / pixel rounding.
                    if (drawn - expected).abs() > expected * 0.02 + 0.5 {
                        failures.push(format!(
                            "{text:?} at {scale}: drawn {drawn}, expected {expected}"
                        ));
                    }
                }
            }
            assert!(measurer.line_advance(&style) > 0.0);
        });
        // No renderer in this test: discard the glyph texture uploads.
        output.textures_delta.clear();
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures.join("\n"))
        }
    }
}
