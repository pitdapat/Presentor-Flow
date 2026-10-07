//! `TextMeasurer` implemented with egui's font system (T0.5).

use presenter_core::domain::TextStyle;
use presenter_core::layout::TextMeasurer;

/// Measures text with the egui fonts installed by [`super::fonts::install`].
pub struct EguiMeasurer {
    ctx: egui::Context,
}

impl EguiMeasurer {
    /// Creates a measurer bound to the egui context.
    pub fn new(ctx: egui::Context) -> Self {
        Self { ctx }
    }
}

impl TextMeasurer for EguiMeasurer {
    fn line_width(&self, text: &str, style: &TextStyle) -> f32 {
        let _ = (&self.ctx, text, style);
        todo!("T0.5: measure with ctx.fonts + extra_letter_spacing")
    }

    fn line_advance(&self, style: &TextStyle) -> f32 {
        let _ = style;
        todo!("T0.5: row height × line_height")
    }
}
