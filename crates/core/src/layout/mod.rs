//! Slide layout: line breaking and positioning at canvas scale (PLAN §3.6).
//!
//! Layout is pure geometry. Font metrics come from a [`TextMeasurer`]
//! supplied by the app, so this module never loads fonts or touches egui.

pub mod measure;

pub use measure::TextMeasurer;

use crate::domain::{HAlign, Slide, TextStyle, VAlign};

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
///
/// Explicit line breaks are kept. Each line wraps greedily by word; a word
/// wider than the safe area breaks by character, and CJK characters can
/// break anywhere (they have no spaces between words).
pub fn layout_slide(slide: &Slide, style: &TextStyle, measurer: &dyn TextMeasurer) -> SlideLayout {
    let max_width = CANVAS.w - 2.0 * SAFE_MARGIN;
    let safe_height = CANVAS.h - 2.0 * SAFE_MARGIN;

    let rows: Vec<String> = slide
        .text
        .split('\n')
        .flat_map(|line| wrap_line(line, style, measurer, max_width))
        .collect();

    let advance = measurer.line_advance(style);
    let total_height = advance * rows.len() as f32;
    let top = SAFE_MARGIN
        + match style.v_align {
            VAlign::Top => 0.0,
            VAlign::Middle => (safe_height - total_height) / 2.0,
            VAlign::Bottom => safe_height - total_height,
        }
        // Overflowing text starts at the top of the safe area and is clipped
        // at the bottom, so the first lines stay readable.
        .max(0.0);

    let lines = rows
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            let width = measurer.line_width(&text, style);
            let x = SAFE_MARGIN
                + match style.h_align {
                    HAlign::Left => 0.0,
                    HAlign::Center => (max_width - width) / 2.0,
                    HAlign::Right => max_width - width,
                };
            PositionedLine {
                text,
                origin: Point {
                    x,
                    y: top + advance * i as f32,
                },
            }
        })
        .collect();

    SlideLayout {
        lines,
        // A small tolerance so rounding never flags text that fits exactly.
        overflow: total_height > safe_height + 0.5,
    }
}

/// A piece of a line that is never split unless it alone is too wide.
struct Atom<'a> {
    text: &'a str,
    /// Whether whitespace came before this atom in the source line.
    space_before: bool,
}

/// True for characters that may wrap without spaces (CJK ideographs, kana,
/// Hangul and full-width punctuation).
fn breaks_anywhere(c: char) -> bool {
    matches!(c,
        '\u{1100}'..='\u{11FF}'
        | '\u{2E80}'..='\u{9FFF}'
        | '\u{AC00}'..='\u{D7AF}'
        | '\u{F900}'..='\u{FAFF}'
        | '\u{FE30}'..='\u{FE4F}'
        | '\u{FF00}'..='\u{FFEF}'
        | '\u{20000}'..='\u{3FFFF}')
}

/// Splits a line into words and single CJK characters.
fn atoms(line: &str) -> Vec<Atom<'_>> {
    let mut out = Vec::new();
    // Start byte of the word being read, and whether a space preceded it.
    let mut word: Option<(usize, bool)> = None;
    let mut pending_space = false;

    for (i, c) in line.char_indices() {
        let is_space = c.is_whitespace();
        let is_cjk = !is_space && breaks_anywhere(c);
        if is_space || is_cjk {
            if let Some((start, space_before)) = word.take() {
                out.push(Atom {
                    text: &line[start..i],
                    space_before,
                });
                pending_space = false;
            }
        }
        if is_space {
            pending_space = true;
        } else if is_cjk {
            out.push(Atom {
                text: &line[i..i + c.len_utf8()],
                space_before: pending_space,
            });
            pending_space = false;
        } else if word.is_none() {
            word = Some((i, pending_space));
            pending_space = false;
        }
    }
    if let Some((start, space_before)) = word {
        out.push(Atom {
            text: &line[start..],
            space_before,
        });
    }
    out
}

/// Wraps one explicit line into rows no wider than `max_width`.
fn wrap_line(
    line: &str,
    style: &TextStyle,
    measurer: &dyn TextMeasurer,
    max_width: f32,
) -> Vec<String> {
    // Tolerance so float rounding never wraps text that fits exactly.
    let fits = |s: &str| measurer.line_width(s, style) <= max_width + 0.01;
    let mut rows = Vec::new();
    let mut current = String::new();

    for atom in atoms(line) {
        if !current.is_empty() {
            let mut candidate = current.clone();
            if atom.space_before {
                candidate.push(' ');
            }
            candidate.push_str(atom.text);
            if fits(&candidate) {
                current = candidate;
                continue;
            }
            rows.push(std::mem::take(&mut current));
        }
        if fits(atom.text) {
            current.push_str(atom.text);
        } else {
            // A single word wider than the line: break it by character.
            for c in atom.text.chars() {
                let mut candidate = current.clone();
                candidate.push(c);
                if !current.is_empty() && !fits(&candidate) {
                    rows.push(std::mem::take(&mut current));
                    current.push(c);
                } else {
                    current = candidate;
                }
            }
        }
    }
    if !current.is_empty() || rows.is_empty() {
        rows.push(current);
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every character is `size / 2` wide plus letter spacing; lines advance
    /// by `size × line_height`.
    struct FakeMeasurer;

    impl TextMeasurer for FakeMeasurer {
        fn line_width(&self, text: &str, style: &TextStyle) -> f32 {
            text.chars().count() as f32 * (style.size / 2.0 + style.letter_spacing)
        }
        fn line_advance(&self, style: &TextStyle) -> f32 {
            style.size * style.line_height
        }
    }

    /// Style where exactly 10 characters fit on a line (1728 / 172.8).
    fn ten_chars() -> TextStyle {
        TextStyle {
            size: 345.6,
            line_height: 1.0,
            ..TextStyle::default()
        }
    }

    fn texts(slide_text: &str, style: &TextStyle) -> Vec<String> {
        let slide = Slide {
            section: None,
            text: slide_text.into(),
        };
        layout_slide(&slide, style, &FakeMeasurer)
            .lines
            .into_iter()
            .map(|l| l.text)
            .collect()
    }

    #[test]
    fn wrapping_rules() {
        let style = ten_chars();
        let cases: &[(&str, &str, &[&str])] = &[
            ("fits on one line", "hello", &["hello"]),
            ("explicit breaks kept", "one\ntwo", &["one", "two"]),
            ("greedy by word", "aaaa bbbb cccc", &["aaaa bbbb", "cccc"]),
            ("exact fit is not wrapped", "aaaa bbbbb", &["aaaa bbbbb"]),
            (
                "long word breaks by character",
                "abcdefghijklm",
                &["abcdefghij", "klm"],
            ),
            (
                "CJK wraps without spaces",
                "一二三四五六七八九十十一",
                &["一二三四五六七八九十", "十一"],
            ),
            (
                "mixed CJK and Latin",
                "主 Jesus 爱我们",
                &["主 Jesus 爱我", "们"],
            ),
            ("repeated spaces collapse", "a    b", &["a b"]),
        ];
        for (name, input, expected) in cases {
            assert_eq!(texts(input, &style), *expected, "case: {name}");
        }
    }

    #[test]
    fn alignment_and_vertical_position() {
        let slide = Slide {
            section: None,
            text: "ab".into(),
        };
        let mut style = ten_chars();
        let width = 2.0 * 172.8;
        let max = CANVAS.w - 2.0 * SAFE_MARGIN;

        style.h_align = HAlign::Left;
        style.v_align = VAlign::Top;
        let l = layout_slide(&slide, &style, &FakeMeasurer);
        assert_eq!(
            l.lines[0].origin,
            Point {
                x: SAFE_MARGIN,
                y: SAFE_MARGIN
            }
        );

        style.h_align = HAlign::Right;
        style.v_align = VAlign::Bottom;
        let l = layout_slide(&slide, &style, &FakeMeasurer);
        let o = l.lines[0].origin;
        assert!((o.x - (SAFE_MARGIN + max - width)).abs() < 0.01);
        assert!((o.y - (CANVAS.h - SAFE_MARGIN - 345.6)).abs() < 0.01);

        style.h_align = HAlign::Center;
        style.v_align = VAlign::Middle;
        let l = layout_slide(&slide, &style, &FakeMeasurer);
        let o = l.lines[0].origin;
        assert!((o.x - (SAFE_MARGIN + (max - width) / 2.0)).abs() < 0.01);
        assert!((o.y - (CANVAS.h - 345.6) / 2.0).abs() < 0.01);
        assert!(!l.overflow);
    }

    #[test]
    fn overflow_is_flagged_and_font_is_not_shrunk() {
        let style = ten_chars(); // 345.6 per line; safe area is 888 tall
        let slide = Slide {
            section: None,
            text: "a\nb\nc".into(),
        };
        let l = layout_slide(&slide, &style, &FakeMeasurer);
        assert!(l.overflow);
        assert_eq!(l.lines.len(), 3);
        assert_eq!(l.lines[0].origin.y, SAFE_MARGIN);

        let two = Slide {
            section: None,
            text: "a\nb".into(),
        };
        assert!(!layout_slide(&two, &style, &FakeMeasurer).overflow);
    }
}
