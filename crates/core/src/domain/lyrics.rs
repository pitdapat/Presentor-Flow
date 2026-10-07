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
    let _ = source;
    todo!("T1.1: implement parse_lyrics with table-driven tests")
}
