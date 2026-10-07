//! The song aggregate.

use super::lyrics::{parse_lyrics, Slide};
use super::style::TextStyle;
use crate::ids::SongId;

/// Seconds since the Unix epoch.
pub type Timestamp = u64;

/// A song in the library. `lyrics_source` is the source of truth; slides are
/// always derived from it.
#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    /// Unique identifier.
    pub id: SongId,
    /// Display title, non-empty after trimming.
    pub title: String,
    /// Raw lyrics exactly as typed in the editor.
    pub lyrics_source: String,
    /// Text style, copied from a template and independent afterwards.
    pub style: TextStyle,
    /// When the song was created.
    pub created_at: Timestamp,
    /// When the song was last changed. Also part of the layout cache key.
    pub updated_at: Timestamp,
}

impl Song {
    /// Slides derived from the lyrics source.
    pub fn slides(&self) -> Vec<Slide> {
        parse_lyrics(&self.lyrics_source).slides
    }
}
