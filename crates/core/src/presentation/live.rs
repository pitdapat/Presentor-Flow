//! Live output state and its rules (PLAN §3.5).
//!
//! The live output holds a *snapshot* of the slide that was sent live, so
//! editing or deleting a song never changes the projector by itself.

use crate::domain::{Rgba, Slide, TextStyle};
use crate::ids::SongId;
use crate::library::Library;
use crate::CoreError;

/// Live output: the current content plus an independent blackout overlay.
#[derive(Debug, Clone, Default)]
pub struct LiveState {
    content: LiveContent,
    blackout: bool,
}

/// What the live output contains, ignoring blackout.
#[derive(Debug, Clone, Default)]
pub enum LiveContent {
    /// Nothing has been sent live. Always the state at app start.
    #[default]
    Empty,
    /// A slide is live.
    Slide(LiveSnapshot),
    /// Lyrics were cleared; the background stays.
    LyricsCleared {
        /// Background of the slide that was cleared.
        background: Rgba,
    },
}

/// A copy of the slide that was sent live.
#[derive(Debug, Clone)]
pub struct LiveSnapshot {
    /// The song the slide came from.
    pub song: SongId,
    /// Song title at the time it went live.
    pub song_title: String,
    /// Index of the slide within the song.
    pub slide_index: usize,
    /// Number of slides in the song at the time it went live.
    pub slide_count: usize,
    /// The slide content.
    pub slide: Slide,
    /// The style at the time it went live.
    pub style: TextStyle,
}

/// What the projector should draw this frame.
#[derive(Debug, Clone)]
pub enum OutputFrame {
    /// Solid black (empty or blackout).
    Black,
    /// Background color only (lyrics cleared).
    Background(Rgba),
    /// A slide with its style.
    Slide {
        /// Slide to draw.
        slide: Slide,
        /// Style to draw it with.
        style: TextStyle,
    },
}

impl LiveState {
    /// Current content, ignoring blackout.
    pub fn content(&self) -> &LiveContent {
        &self.content
    }

    /// Whether blackout is on.
    pub fn is_blackout(&self) -> bool {
        self.blackout
    }

    /// Sends a slide live, snapshotting the current library version.
    pub fn go_live(
        &mut self,
        library: &Library,
        song: SongId,
        slide: usize,
    ) -> Result<(), CoreError> {
        let _ = (library, song, slide);
        todo!("T2.1: go_live")
    }

    /// Re-snapshots the next slide of the live song (clamped).
    pub fn next(&mut self, library: &Library) -> Result<(), CoreError> {
        let _ = library;
        todo!("T2.1: next")
    }

    /// Re-snapshots the previous slide of the live song (clamped).
    pub fn previous(&mut self, library: &Library) -> Result<(), CoreError> {
        let _ = library;
        todo!("T2.1: previous")
    }

    /// Removes the lyrics but keeps the background.
    pub fn clear_lyrics(&mut self) {
        todo!("T2.1: clear_lyrics")
    }

    /// Toggles the blackout overlay without touching the content.
    pub fn toggle_blackout(&mut self) {
        self.blackout = !self.blackout;
    }

    /// What the projector draws.
    pub fn frame(&self) -> OutputFrame {
        todo!("T2.1: frame()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_empty_and_not_blacked_out() {
        let live = LiveState::default();
        assert!(matches!(live.content(), LiveContent::Empty));
        assert!(!live.is_blackout());
    }

    #[test]
    fn blackout_toggles() {
        let mut live = LiveState::default();
        live.toggle_blackout();
        assert!(live.is_blackout());
        live.toggle_blackout();
        assert!(!live.is_blackout());
    }
}
