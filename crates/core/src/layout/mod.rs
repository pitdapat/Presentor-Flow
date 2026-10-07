//! Slide layout: line breaking and positioning at canvas scale (PLAN §3.6).
//!
//! Layout is pure geometry. Font metrics come from a [`TextMeasurer`]
//! supplied by the app, so this module never loads fonts or touches egui.

pub mod measure;

pub use measure::TextMeasurer;

use crate::domain::{Slide, TextStyle};

/// Width and height in canvas units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

/// A point in canvas units, origin top-left.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// Horizontal position.
    pub x: f32,
    /// Vertical position.
    pub y: f32,
}

/// The logical canvas every slide is laid out on.
pub const CANVAS: Size = Size {
    w: 1920.0,
    h: 1080.0,
};
/// Margin on every side of the canvas that text stays inside.
pub const SAFE_MARGIN: f32 = 96.0;

/// One line of text with its position.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionedLine {
    /// Line text.
    pub text: String,
    /// Top-left of the line in canvas units.
    pub origin: Point,
}

/// The result of laying out one slide.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SlideLayout {
    /// Lines in drawing order.
    pub lines: Vec<PositionedLine>,
    /// True when the text is taller than the safe area.
    pub overflow: bool,
}

/// Lays out a slide on the canvas. Never shrinks the font.
pub fn layout_slide(slide: &Slide, style: &TextStyle, measurer: &dyn TextMeasurer) -> SlideLayout {
    let _ = (slide, style, measurer);
    todo!("T0.5: greedy word wrap, CJK char break, alignment, overflow flag")
}
