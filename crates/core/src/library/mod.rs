//! The library aggregate: every validated operation on songs, playlists and
//! templates goes through here, so invariants are enforced in one place
//! (PLAN §3.4).

use crate::domain::{parse_lyrics, song::now, MoveDir, Playlist, Song, Template, TextStyle};
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
    revision: u64,
}

/// Checks a song's title and lyrics; returns the trimmed title.
fn validate_song(title: &str, lyrics: &str) -> Result<String, CoreError> {
    let title = title.trim();
    if title.is_empty() {
        return Err(CoreError::EmptyTitle);
    }
    if parse_lyrics(lyrics).slides.is_empty() {
        return Err(CoreError::NoSlides);
    }
    Ok(title.to_owned())
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
            revision: 0,
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

    /// Increases on every successful change. Views use it to drop cached
    /// layouts; it is not saved.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    fn song_mut(&mut self, id: SongId) -> Result<&mut Song, CoreError> {
        self.songs
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or(CoreError::SongNotFound(id))
    }

    // --- Songs (M1: T1.2) ---------------------------------------------------

    /// Creates a song with the default style. Fails on an empty title or
    /// lyrics with no slides. Duplicate titles are allowed.
    pub fn create_song(&mut self, title: &str, lyrics: &str) -> Result<SongId, CoreError> {
        let title = validate_song(title, lyrics)?;
        let now = now();
        let id = SongId::new();
        self.songs.push(Song {
            id,
            title,
            lyrics_source: lyrics.to_owned(),
            style: TextStyle::default(),
            created_at: now,
            updated_at: now,
        });
        self.revision += 1;
        Ok(id)
    }

    /// Replaces a song's title and lyrics. The style is kept.
    pub fn update_song(&mut self, id: SongId, title: &str, lyrics: &str) -> Result<(), CoreError> {
        let title = validate_song(title, lyrics)?;
        let song = self.song_mut(id)?;
        song.title = title;
        song.lyrics_source = lyrics.to_owned();
        song.updated_at = now();
        self.revision += 1;
        Ok(())
    }

    /// Deletes a song and every playlist entry referring to it. Returns the
    /// playlists that were affected.
    pub fn delete_song(&mut self, id: SongId) -> Result<Vec<PlaylistId>, CoreError> {
        let index = self
            .songs
            .iter()
            .position(|s| s.id == id)
            .ok_or(CoreError::SongNotFound(id))?;
        self.songs.remove(index);

        let mut affected = Vec::new();
        for playlist in &mut self.playlists {
            let before = playlist.entries.len();
            playlist.entries.retain(|e| e.song != id);
            if playlist.entries.len() != before {
                affected.push(playlist.id);
            }
        }
        self.revision += 1;
        Ok(affected)
    }

    /// Sets a song's style after validating it.
    pub fn set_song_style(&mut self, id: SongId, style: TextStyle) -> Result<(), CoreError> {
        style.validate()?;
        let song = self.song_mut(id)?;
        song.style = style;
        song.updated_at = now();
        self.revision += 1;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::PlaylistEntry;

    #[test]
    fn create_song_validates_title_and_lyrics() {
        let mut lib = Library::default();
        assert!(matches!(
            lib.create_song("  ", "a"),
            Err(CoreError::EmptyTitle)
        ));
        assert!(matches!(
            lib.create_song("T", "\n[Chorus]\n"),
            Err(CoreError::NoSlides)
        ));
        assert!(lib.songs().is_empty());
        assert_eq!(lib.revision(), 0);
    }

    #[test]
    fn create_trims_title_and_allows_duplicates() -> Result<(), CoreError> {
        let mut lib = Library::default();
        let a = lib.create_song("  Song  ", "x")?;
        let b = lib.create_song("Song", "y")?;
        assert_ne!(a, b);
        assert_eq!(lib.song(a).map(|s| s.title.as_str()), Some("Song"));
        assert_eq!(lib.songs().len(), 2);
        Ok(())
    }

    #[test]
    fn update_song_keeps_style_and_bumps_revision() -> Result<(), CoreError> {
        let mut lib = Library::default();
        let id = lib.create_song("A", "x")?;
        let style = TextStyle {
            size: 50.0,
            ..TextStyle::default()
        };
        lib.set_song_style(id, style)?;
        let rev = lib.revision();
        lib.update_song(id, "B", "y\n\nz")?;
        let song = lib.song(id).ok_or(CoreError::SongNotFound(id))?;
        assert_eq!(song.title, "B");
        assert_eq!(song.slides().len(), 2);
        assert_eq!(song.style, style);
        assert!(lib.revision() > rev);
        Ok(())
    }

    #[test]
    fn invalid_update_changes_nothing() -> Result<(), CoreError> {
        let mut lib = Library::default();
        let id = lib.create_song("A", "x")?;
        assert!(lib.update_song(id, "", "y").is_err());
        assert_eq!(lib.song(id).map(|s| s.title.as_str()), Some("A"));
        assert!(matches!(
            lib.update_song(SongId::new(), "A", "x"),
            Err(CoreError::SongNotFound(_))
        ));
        Ok(())
    }

    #[test]
    fn delete_song_removes_its_playlist_entries() -> Result<(), CoreError> {
        let mut lib = Library::default();
        let keep = lib.create_song("Keep", "x")?;
        let gone = lib.create_song("Gone", "x")?;
        let entry = |song| PlaylistEntry {
            id: EntryId::new(),
            song,
        };
        let p1 = Playlist {
            id: PlaylistId::new(),
            name: "Sunday".into(),
            entries: vec![entry(gone), entry(keep), entry(gone)],
        };
        let p2 = Playlist {
            id: PlaylistId::new(),
            name: "Other".into(),
            entries: vec![entry(keep)],
        };
        let p1_id = p1.id;
        lib.playlists = vec![p1, p2];

        assert_eq!(lib.delete_song(gone)?, vec![p1_id]);
        assert!(lib.song(gone).is_none());
        assert_eq!(lib.playlists()[0].entries.len(), 1);
        assert_eq!(lib.playlists()[1].entries.len(), 1);
        assert!(lib.delete_song(gone).is_err());
        Ok(())
    }

    #[test]
    fn set_song_style_rejects_invalid_style() -> Result<(), CoreError> {
        let mut lib = Library::default();
        let id = lib.create_song("A", "x")?;
        let bad = TextStyle {
            size: 1000.0,
            ..TextStyle::default()
        };
        assert!(lib.set_song_style(id, bad).is_err());
        assert_eq!(lib.song(id).map(|s| s.style), Some(TextStyle::default()));
        Ok(())
    }
}
