//! Versioned on-disk format. These DTOs are the ONLY serialized types;
//! domain types are converted to and from them here (PLAN §3.2).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Schema version written by this build.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Root of `library.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryFileV1 {
    /// Always [`CURRENT_SCHEMA_VERSION`] when written.
    pub schema_version: u32,
    /// Songs.
    pub songs: Vec<SongV1>,
    /// Playlists.
    pub playlists: Vec<PlaylistV1>,
    /// Templates.
    pub templates: Vec<TemplateV1>,
}

/// A stored song.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongV1 {
    /// Song ID.
    pub id: Uuid,
    /// Title.
    pub title: String,
    /// Raw lyrics source.
    pub lyrics_source: String,
    /// Style.
    pub style: TextStyleV1,
    /// Creation time (Unix seconds).
    pub created_at: u64,
    /// Last change (Unix seconds).
    pub updated_at: u64,
}

/// A stored playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistV1 {
    /// Playlist ID.
    pub id: Uuid,
    /// Name.
    pub name: String,
    /// Entries in order.
    pub entries: Vec<PlaylistEntryV1>,
}

/// A stored playlist entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistEntryV1 {
    /// Entry ID.
    pub id: Uuid,
    /// Referenced song ID.
    pub song: Uuid,
}

/// A stored template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateV1 {
    /// Template ID.
    pub id: Uuid,
    /// Name.
    pub name: String,
    /// Style.
    pub style: TextStyleV1,
}

/// A stored text style. Enums are stored as strings for readability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyleV1 {
    /// Font family name, for example `"NotoSans"`.
    pub font: String,
    /// Font size.
    pub size: f32,
    /// Line-height multiplier.
    pub line_height: f32,
    /// Extra letter spacing.
    pub letter_spacing: f32,
    /// `"Left"`, `"Center"` or `"Right"`.
    pub h_align: String,
    /// `"Top"`, `"Middle"` or `"Bottom"`.
    pub v_align: String,
    /// Text color as `[r, g, b, a]`.
    pub text_color: [u8; 4],
    /// Background color as `[r, g, b, a]`.
    pub background: [u8; 4],
}

// T1.3: `impl From<&Library> for LibraryFileV1` and
// `impl TryFrom<LibraryFileV1> for Library` go here.
