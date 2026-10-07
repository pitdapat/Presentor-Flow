//! The ONE function that draws a slide into a rectangle. Thumbnails, both
//! previews and the projector all call it (PLAN §3.6).

use egui::{Color32, Pos2, Rect};
use presenter_core::domain::{Rgba, TextStyle};
use presenter_core::layout::{SlideLayout, CANVAS, SAFE_MARGIN};
use presenter_core::presentation::OutputFrame;

use super::measurer::text_job;

/// Converts a core color to an egui color.
pub fn color(c: Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

/// The largest 16:9 rectangle centered in `outer` (letterboxed).
pub fn fit_canvas(outer: Rect) -> Rect {
    let aspect = CANVAS.w / CANVAS.h;
    let mut size = outer.size();
    if size.x / size.y > aspect {
        size.x = size.y * aspect;
    } else {
        size.y = size.x / aspect;
    }
    Rect::from_center_size(outer.center(), size)
}

/// Paints `frame` scaled from canvas units into `rect`. `row_height` is the
/// glyph row height in canvas units (from the measurer); lines are centered
/// vertically in their line-height box.
///
/// Lines are drawn exactly as `layout` positioned them; nothing is wrapped
/// here, so every view breaks lines identically.
pub fn paint_slide(
    painter: &egui::Painter,
    rect: Rect,
    frame: &OutputFrame,
    layout: &SlideLayout,
    row_height: f32,
) {
    let (background, slide) = match frame {
        OutputFrame::Black => (Color32::BLACK, None),
        OutputFrame::Background(c) => (color(*c), None),
        OutputFrame::Slide { style, .. } => (color(style.background), Some(style)),
    };
    painter.rect_filled(rect, 0.0, background);
    let Some(style) = slide else {
        return;
    };

    let scale = rect.width() / CANVAS.w;
    let to_screen = |x: f32, y: f32| Pos2::new(rect.min.x + x * scale, rect.min.y + y * scale);
    // Text is clipped to the safe area; overflow is flagged in the UI.
    let safe = Rect::from_min_max(
        to_screen(SAFE_MARGIN, SAFE_MARGIN),
        to_screen(CANVAS.w - SAFE_MARGIN, CANVAS.h - SAFE_MARGIN),
    );
    let clipped = painter.with_clip_rect(safe.intersect(painter.clip_rect()));
    paint_lines(&clipped, layout, style, scale, &to_screen, row_height);
}

fn paint_lines(
    painter: &egui::Painter,
    layout: &SlideLayout,
    style: &TextStyle,
    scale: f32,
    to_screen: &dyn Fn(f32, f32) -> Pos2,
    row_height: f32,
) {
    let size = style.size * scale;
    // Below ~1 px text is invisible; skip it rather than asking for a
    // zero-size font.
    if size < 1.0 {
        return;
    }
    let text_color = color(style.text_color);
    let advance = row_height * style.line_height;
    let offset = (advance - row_height) / 2.0;
    for line in &layout.lines {
        let galley = painter.layout_job(text_job(&line.text, style, size, text_color));
        painter.galley(
            to_screen(line.origin.x, line.origin.y + offset),
            galley,
            text_color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_canvas_letterboxes_to_16_by_9() {
        let wide = fit_canvas(Rect::from_min_size(Pos2::ZERO, egui::vec2(400.0, 100.0)));
        assert!((wide.width() - 177.78).abs() < 0.01);
        assert_eq!(wide.height(), 100.0);
        assert_eq!(wide.center(), Pos2::new(200.0, 50.0));

        let tall = fit_canvas(Rect::from_min_size(Pos2::ZERO, egui::vec2(160.0, 400.0)));
        assert_eq!(tall.width(), 160.0);
        assert_eq!(tall.height(), 90.0);
    }
}
