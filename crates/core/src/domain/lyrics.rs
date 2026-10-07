//! Lyrics parsing: raw editor text in, slides out (PLAN §3.4).

/// One slide of a song.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slide {
    /// Section label (from a `[Chorus]`-style line), if any. Never shown on screen.
    pub section: Option<String>,
    /// Slide text with its internal line breaks kept.
    pub text: String,
}

/// Result of parsing a song's lyrics source.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedLyrics {
    /// Slides in display order. Empty slides are already dropped.
    pub slides: Vec<Slide>,
}

/// Parses raw lyrics text into slides.
///
/// Rules (PLAN §3.4): `\r\n` becomes `\n`; trailing whitespace is trimmed;
/// blank lines separate slides; a `[Label]` line starts a new section and
/// ends the current slide; empty slides are dropped.
pub fn parse_lyrics(source: &str) -> ParsedLyrics {
    let mut slides = Vec::new();
    let mut section: Option<String> = None;
    let mut current: Vec<&str> = Vec::new();

    let mut flush = |current: &mut Vec<&str>, section: &Option<String>| {
        if !current.is_empty() {
            slides.push(Slide {
                section: section.clone(),
                text: current.join("\n"),
            });
            current.clear();
        }
    };

    // `str::lines` already treats "\r\n" as a line break.
    for raw in source.lines() {
        let line = raw.trim_end();
        if line.trim().is_empty() {
            flush(&mut current, &section);
        } else if let Some(label) = section_label(line) {
            flush(&mut current, &section);
            section = Some(label.to_owned());
        } else {
            current.push(line);
        }
    }
    flush(&mut current, &section);

    ParsedLyrics { slides }
}

/// Returns the label of a `[Label]` line (after trimming), if it is one.
fn section_label(line: &str) -> Option<&str> {
    let label = line.trim().strip_prefix('[')?.strip_suffix(']')?.trim();
    (!label.is_empty()).then_some(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slide(section: Option<&str>, text: &str) -> Slide {
        Slide {
            section: section.map(str::to_owned),
            text: text.to_owned(),
        }
    }

    #[test]
    fn parsing_rules() {
        let cases: &[(&str, &str, Vec<Slide>)] = &[
            ("empty input", "", vec![]),
            ("only blank lines", "\n  \n\t\n", vec![]),
            ("one line", "Hello", vec![slide(None, "Hello")]),
            (
                "line breaks inside a slide are kept",
                "a\nb",
                vec![slide(None, "a\nb")],
            ),
            (
                "blank line separates slides",
                "a\n\nb",
                vec![slide(None, "a"), slide(None, "b")],
            ),
            (
                "several blank lines are one separator",
                "a\n\n\n\nb",
                vec![slide(None, "a"), slide(None, "b")],
            ),
            (
                "CRLF is normalized",
                "a\r\nb\r\n\r\nc",
                vec![slide(None, "a\nb"), slide(None, "c")],
            ),
            (
                "trailing whitespace is trimmed",
                "a  \t\nb ",
                vec![slide(None, "a\nb")],
            ),
            (
                "leading whitespace is kept",
                "  a",
                vec![slide(None, "  a")],
            ),
            (
                "label starts a section and is not shown",
                "[Verse 1]\na\n\nb",
                vec![slide(Some("Verse 1"), "a"), slide(Some("Verse 1"), "b")],
            ),
            (
                "label ends the current slide",
                "a\n[Chorus]\nb",
                vec![slide(None, "a"), slide(Some("Chorus"), "b")],
            ),
            (
                "label with surrounding spaces",
                "  [ Bridge ]  \nx",
                vec![slide(Some("Bridge"), "x")],
            ),
            (
                "empty brackets are text, not a label",
                "[]\nx",
                vec![slide(None, "[]\nx")],
            ),
            (
                "text around brackets is not a label",
                "[a] b",
                vec![slide(None, "[a] b")],
            ),
            (
                "repeated sections and lines are kept",
                "[C]\nx\nx\n[C]\nx",
                vec![slide(Some("C"), "x\nx"), slide(Some("C"), "x")],
            ),
            (
                "label with no lines produces no slide",
                "[Intro]\n\n[Verse]\na",
                vec![slide(Some("Verse"), "a")],
            ),
            (
                "Chinese text",
                "奇妙恩典\n何等甘甜",
                vec![slide(None, "奇妙恩典\n何等甘甜")],
            ),
        ];

        for (name, input, expected) in cases {
            assert_eq!(&parse_lyrics(input).slides, expected, "case: {name}");
        }
    }
}
