//! Playlists (services). Entries reference songs; they never copy them.

use crate::ids::{EntryId, PlaylistId, SongId};

/// An ordered list of songs for a service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Playlist {
    /// Unique identifier.
    pub id: PlaylistId,
    /// Display name, non-empty after trimming.
    pub name: String,
    /// Entries in service order.
    pub entries: Vec<PlaylistEntry>,
}

/// One position in a playlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaylistEntry {
    /// Identifies this position, so the same song can appear twice.
    pub id: EntryId,
    /// The referenced song. Must exist in the library.
    pub song: SongId,
}

/// Direction for moving a playlist entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDir {
    /// Towards the start of the playlist.
    Up,
    /// Towards the end of the playlist.
    Down,
}
