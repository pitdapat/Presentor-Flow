//! The library aggregate: every validated operation on songs, playlists and
//! templates goes through here, so invariants are enforced in one place
//! (PLAN §3.4).

use crate::domain::{MoveDir, Playlist, Song, Template, TextStyle};
use crate::ids::{EntryId, PlaylistId, SongId, TemplateId};
use crate::CoreError;

/// All songs, playlists and templates.
///
/// Invariant: every playlist entry refers to an existing song.
#[derive(Debug, Clone, Default)]
pub struct Library {
    songs: Vec<Song>,
    playlists: Vec<Playlist>,
    templates: Vec<Template>,
}

impl Library {
    /// Builds a library from already-validated parts (used by storage).
    pub fn from_parts(
        songs: Vec<Song>,
        playlists: Vec<Playlist>,
        templates: Vec<Template>,
    ) -> Self {
        Self {
            songs,
            playlists,
            templates,
        }
    }

    /// All songs, in insertion order.
    pub fn songs(&self) -> &[Song] {
        &self.songs
    }

    /// All playlists.
    pub fn playlists(&self) -> &[Playlist] {
        &self.playlists
    }

    /// All templates.
    pub fn templates(&self) -> &[Template] {
        &self.templates
    }

    /// Looks up a song by ID.
    pub fn song(&self, id: SongId) -> Option<&Song> {
        self.songs.iter().find(|s| s.id == id)
    }

    // --- Songs (M1: T1.2) ---------------------------------------------------

    /// Creates a song. Fails on an empty title or lyrics with no slides.
    pub fn create_song(&mut self, title: &str, lyrics: &str) -> Result<SongId, CoreError> {
        let _ = (title, lyrics);
        todo!("T1.2: create_song")
    }

    /// Replaces a song's title and lyrics.
    pub fn update_song(&mut self, id: SongId, title: &str, lyrics: &str) -> Result<(), CoreError> {
        let _ = (id, title, lyrics);
        todo!("T1.2: update_song")
    }

    /// Deletes a song and every playlist entry referring to it. Returns the
    /// playlists that were affected.
    pub fn delete_song(&mut self, id: SongId) -> Result<Vec<PlaylistId>, CoreError> {
        let _ = id;
        todo!("T1.2 + T3.1: delete_song with reference cleanup")
    }

    /// Sets a song's style.
    pub fn set_song_style(&mut self, id: SongId, style: TextStyle) -> Result<(), CoreError> {
        let _ = (id, style);
        todo!("T4.1: set_song_style")
    }

    // --- Playlists (M3: T3.1) ----------------------------------------------

    /// Creates an empty playlist.
    pub fn create_playlist(&mut self, name: &str) -> Result<PlaylistId, CoreError> {
        let _ = name;
        todo!("T3.1: create_playlist")
    }

    /// Renames a playlist.
    pub fn rename_playlist(&mut self, id: PlaylistId, name: &str) -> Result<(), CoreError> {
        let _ = (id, name);
        todo!("T3.1: rename_playlist")
    }

    /// Deletes a playlist. Songs are not affected.
    pub fn delete_playlist(&mut self, id: PlaylistId) -> Result<(), CoreError> {
        let _ = id;
        todo!("T3.1: delete_playlist")
    }

    /// Appends a song to a playlist.
    pub fn add_to_playlist(
        &mut self,
        playlist: PlaylistId,
        song: SongId,
    ) -> Result<EntryId, CoreError> {
        let _ = (playlist, song);
        todo!("T3.1: add_to_playlist")
    }

    /// Moves an entry one position up or down.
    pub fn move_entry(
        &mut self,
        playlist: PlaylistId,
        entry: EntryId,
        dir: MoveDir,
    ) -> Result<(), CoreError> {
        let _ = (playlist, entry, dir);
        todo!("T3.1: move_entry")
    }

    /// Removes an entry from a playlist.
    pub fn remove_entry(&mut self, playlist: PlaylistId, entry: EntryId) -> Result<(), CoreError> {
        let _ = (playlist, entry);
        todo!("T3.1: remove_entry")
    }

    // --- Templates (M4: T4.2) ----------------------------------------------

    /// Saves a new template.
    pub fn save_template(&mut self, name: &str, style: TextStyle) -> Result<TemplateId, CoreError> {
        let _ = (name, style);
        todo!("T4.2: save_template")
    }

    /// Copies a template's style into a song.
    pub fn apply_template(&mut self, song: SongId, template: TemplateId) -> Result<(), CoreError> {
        let _ = (song, template);
        todo!("T4.2: apply_template")
    }
}
