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
        /// The slide that was cleared, so Next/Previous continue from it.
        last: Option<LiveCursor>,
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
        self.content = LiveContent::Slide(snapshot(library, song, slide)?);
        Ok(())
    }

    /// Re-snapshots the next slide of the live song (clamped).
    pub fn next(&mut self, library: &Library) -> Result<(), CoreError> {
        self.step(library, Step::Next)
    }

    /// Re-snapshots the previous slide of the live song (clamped).
    pub fn previous(&mut self, library: &Library) -> Result<(), CoreError> {
        self.step(library, Step::Previous)
    }

    /// Removes the lyrics but keeps the background.
    pub fn clear_lyrics(&mut self) {
        if let LiveContent::Slide(snap) = &self.content {
            self.content = LiveContent::LyricsCleared {
                background: snap.style.background,
                last: Some(LiveCursor {
                    song: snap.song,
                    slide_index: snap.slide_index,
                }),
            };
        }
    }

    /// Toggles the blackout overlay without touching the content.
    pub fn toggle_blackout(&mut self) {
        self.blackout = !self.blackout;
    }

    /// What the projector draws.
    pub fn frame(&self) -> OutputFrame {
        if self.blackout {
            return OutputFrame::Black;
        }
        match &self.content {
            LiveContent::Empty => OutputFrame::Black,
            LiveContent::LyricsCleared { background, .. } => OutputFrame::Background(*background),
            LiveContent::Slide(snap) => OutputFrame::Slide {
                slide: snap.slide.clone(),
                style: snap.style,
            },
        }
    }

    /// The song and slide that Next/Previous move from, if any.
    fn cursor(&self) -> Option<LiveCursor> {
        match &self.content {
            LiveContent::Empty => None,
            LiveContent::Slide(snap) => Some(LiveCursor {
                song: snap.song,
                slide_index: snap.slide_index,
            }),
            LiveContent::LyricsCleared { last, .. } => *last,
        }
    }

    fn step(&mut self, library: &Library, step: Step) -> Result<(), CoreError> {
        let Some(cursor) = self.cursor() else {
            return Err(CoreError::NothingLive);
        };
        let song = library
            .song(cursor.song)
            .ok_or(CoreError::SongNotFound(cursor.song))?;
        let count = song.slides().len();
        if count == 0 {
            return Err(CoreError::NoSlides);
        }
        let index = match step {
            // After Clear, Next presents the following slide (PLAN §3.5).
            Step::Next => cursor.slide_index.saturating_add(1),
            Step::Previous => cursor.slide_index.saturating_sub(1),
        }
        // Clamped to the song, which may have shrunk since it went live.
        .min(count - 1);
        self.go_live(library, cursor.song, index)
    }
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Next,
    Previous,
}

/// Where the live output last was, kept through Clear Lyrics so Next and
/// Previous still know the song.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiveCursor {
    /// The live song.
    pub song: SongId,
    /// Index of the slide that was live.
    pub slide_index: usize,
}

/// Builds a snapshot of `song`'s slide `index` from the library.
fn snapshot(library: &Library, song: SongId, index: usize) -> Result<LiveSnapshot, CoreError> {
    let song = library.song(song).ok_or(CoreError::SongNotFound(song))?;
    let slides = song.slides();
    let slide_count = slides.len();
    let slide = slides
        .into_iter()
        .nth(index)
        .ok_or(CoreError::SlideNotFound(index))?;
    Ok(LiveSnapshot {
        song: song.id,
        song_title: song.title.clone(),
        slide_index: index,
        slide_count,
        slide,
        style: song.style,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn library() -> Result<(Library, SongId), CoreError> {
        let mut lib = Library::default();
        let id = lib.create_song("Song", "one\n\ntwo\n\nthree")?;
        Ok((lib, id))
    }

    fn live_index(live: &LiveState) -> Option<usize> {
        match live.content() {
            LiveContent::Slide(s) => Some(s.slide_index),
            _ => None,
        }
    }

    fn live_text(live: &LiveState) -> Option<String> {
        match live.frame() {
            OutputFrame::Slide { slide, .. } => Some(slide.text),
            _ => None,
        }
    }

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

    #[test]
    fn go_live_snapshots_the_slide() -> Result<(), CoreError> {
        let (lib, id) = library()?;
        let mut live = LiveState::default();
        live.go_live(&lib, id, 1)?;
        assert_eq!(live_text(&live).as_deref(), Some("two"));
        let LiveContent::Slide(snap) = live.content() else {
            return Err(CoreError::NothingLive);
        };
        assert_eq!((snap.slide_index, snap.slide_count), (1, 3));
        assert_eq!(snap.song_title, "Song");
        Ok(())
    }

    #[test]
    fn go_live_with_bad_ids_changes_nothing() -> Result<(), CoreError> {
        let (lib, id) = library()?;
        let mut live = LiveState::default();
        live.go_live(&lib, id, 0)?;
        assert!(live.go_live(&lib, id, 9).is_err());
        assert!(live.go_live(&lib, SongId::new(), 0).is_err());
        assert_eq!(live_index(&live), Some(0));
        Ok(())
    }

    #[test]
    fn next_and_previous_clamp_within_the_song() -> Result<(), CoreError> {
        let (lib, id) = library()?;
        let mut live = LiveState::default();
        assert!(matches!(live.next(&lib), Err(CoreError::NothingLive)));
        live.go_live(&lib, id, 0)?;
        live.previous(&lib)?;
        assert_eq!(live_index(&live), Some(0));
        live.next(&lib)?;
        live.next(&lib)?;
        live.next(&lib)?;
        assert_eq!(live_index(&live), Some(2));
        live.previous(&lib)?;
        assert_eq!(live_index(&live), Some(1));
        Ok(())
    }

    #[test]
    fn editing_a_live_song_does_not_change_output_until_next() -> Result<(), CoreError> {
        let (mut lib, id) = library()?;
        let mut live = LiveState::default();
        live.go_live(&lib, id, 0)?;
        lib.update_song(id, "Song", "ONE\n\nTWO\n\nTHREE")?;
        assert_eq!(live_text(&live).as_deref(), Some("one"));
        live.next(&lib)?;
        assert_eq!(live_text(&live).as_deref(), Some("TWO"));
        Ok(())
    }

    #[test]
    fn next_clamps_when_the_song_got_shorter() -> Result<(), CoreError> {
        let (mut lib, id) = library()?;
        let mut live = LiveState::default();
        live.go_live(&lib, id, 2)?;
        lib.update_song(id, "Song", "only")?;
        live.next(&lib)?;
        assert_eq!(live_text(&live).as_deref(), Some("only"));
        Ok(())
    }

    #[test]
    fn deleting_the_live_song_keeps_output_and_next_reports_it() -> Result<(), CoreError> {
        let (mut lib, id) = library()?;
        let mut live = LiveState::default();
        live.go_live(&lib, id, 1)?;
        lib.delete_song(id)?;
        assert_eq!(live_text(&live).as_deref(), Some("two"));
        assert!(matches!(live.next(&lib), Err(CoreError::SongNotFound(_))));
        assert_eq!(live_text(&live).as_deref(), Some("two"));
        Ok(())
    }

    #[test]
    fn clear_keeps_background_and_next_continues() -> Result<(), CoreError> {
        let (lib, id) = library()?;
        let mut live = LiveState::default();
        live.clear_lyrics();
        assert!(matches!(live.content(), LiveContent::Empty));

        live.go_live(&lib, id, 0)?;
        live.clear_lyrics();
        assert!(matches!(live.frame(), OutputFrame::Background(Rgba::BLACK)));
        live.next(&lib)?;
        assert_eq!(live_text(&live).as_deref(), Some("two"));
        Ok(())
    }

    #[test]
    fn blackout_overrides_frame_and_restores_content() -> Result<(), CoreError> {
        let (lib, id) = library()?;
        let mut live = LiveState::default();
        assert!(matches!(live.frame(), OutputFrame::Black));
        live.go_live(&lib, id, 0)?;
        live.toggle_blackout();
        assert!(matches!(live.frame(), OutputFrame::Black));
        live.next(&lib)?;
        assert!(matches!(live.frame(), OutputFrame::Black));
        live.toggle_blackout();
        assert_eq!(live_text(&live).as_deref(), Some("two"));
        Ok(())
    }
}
