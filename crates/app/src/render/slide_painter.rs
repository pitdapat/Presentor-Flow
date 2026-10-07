//! The ONE function that draws a slide into a rectangle. Thumbnails, both
//! previews and the projector all call it (PLAN §3.6).

use presenter_core::layout::SlideLayout;
use presenter_core::presentation::OutputFrame;

/// Paints `frame` scaled from canvas units into `rect`.
pub fn paint_slide(
    painter: &egui::Painter,
    rect: egui::Rect,
    frame: &OutputFrame,
    layout: &SlideLayout,
) {
    let _ = (painter, rect, frame, layout);
    todo!("T0.5: background fill + positioned lines, no wrapping")
}
