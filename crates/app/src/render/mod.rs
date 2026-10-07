//! Drawing slides. `slide_painter::paint_slide` is the only slide drawing
//! code in the app (PLAN §3.6).

pub mod fonts;
pub mod measurer;
pub mod slide_painter;

use std::collections::HashMap;

use presenter_core::domain::{Slide, TextStyle};
use presenter_core::layout::{layout_slide, SlideLayout};
use presenter_core::presentation::OutputFrame;

use measurer::EguiMeasurer;

/// Cache entries kept before the cache is cleared (a library has a few
/// hundred slides; this only bounds pathological growth).
const CACHE_LIMIT: usize = 4096;

/// Lays out and paints slides, caching layouts by slide text and style so
/// thumbnails are not re-measured every frame.
pub struct SlideRenderer {
    measurer: EguiMeasurer,
    cache: HashMap<(String, String), SlideLayout>,
}

impl SlideRenderer {
    /// Creates a renderer bound to the egui context (fonts must be installed).
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            measurer: EguiMeasurer::new(ctx),
            cache: HashMap::new(),
        }
    }

    /// The layout of `slide` in `style`, from the cache when possible.
    pub fn layout(&mut self, slide: &Slide, style: &TextStyle) -> SlideLayout {
        // `TextStyle` holds floats, so it is keyed by its debug text, which
        // is exact for every field.
        let key = (slide.text.clone(), format!("{style:?}"));
        if let Some(layout) = self.cache.get(&key) {
            return layout.clone();
        }
        if self.cache.len() >= CACHE_LIMIT {
            self.cache.clear();
        }
        let layout = layout_slide(slide, style, &self.measurer);
        self.cache.insert(key, layout.clone());
        layout
    }

    /// Paints `frame` into `rect` (letterboxed to 16:9). Returns whether the
    /// slide's text overflows the safe area.
    pub fn paint(
        &mut self,
        painter: &egui::Painter,
        rect: egui::Rect,
        frame: &OutputFrame,
    ) -> bool {
        let canvas = slide_painter::fit_canvas(rect);
        let (layout, row_height) = match frame {
            OutputFrame::Slide { slide, style } => {
                (self.layout(slide, style), self.measurer.row_height(style))
            }
            _ => (SlideLayout::default(), 0.0),
        };
        slide_painter::paint_slide(painter, canvas, frame, &layout, row_height);
        layout.overflow
    }
}
