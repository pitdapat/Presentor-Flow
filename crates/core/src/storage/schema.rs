//! Versioned on-disk format. These DTOs are the ONLY serialized types;
//! domain types are converted to and from them here (PLAN §3.2).

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    FontFamily, HAlign, Playlist, PlaylistEntry, Rgba, Song, Template, TextStyle, VAlign,
};
use crate::ids::{EntryId, PlaylistId, SongId, TemplateId};
use crate::library::Library;
use crate::settings::{DisplayRef, Settings};

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

// --- Settings ---------------------------------------------------------------

/// Root of `settings.json`. Every field is optional so a partial or older
/// file still loads.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsFileV1 {
    /// Chosen output display.
    #[serde(default)]
    pub output_display: Option<DisplayRefV1>,
    /// Playlist that was open at exit.
    #[serde(default)]
    pub last_playlist: Option<Uuid>,
}

/// A stored display reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayRefV1 {
    /// OS device name.
    pub device_name: String,
    /// Physical rectangle: left, top, right, bottom.
    pub rect: [i32; 4],
}

impl From<&Settings> for SettingsFileV1 {
    fn from(s: &Settings) -> Self {
        Self {
            output_display: s.output_display.as_ref().map(|d| DisplayRefV1 {
                device_name: d.device_name.clone(),
                rect: d.rect,
            }),
            last_playlist: s.last_playlist.map(|p| p.as_uuid()),
        }
    }
}

impl From<SettingsFileV1> for Settings {
    fn from(f: SettingsFileV1) -> Self {
        Self {
            output_display: f.output_display.map(|d| DisplayRef {
                device_name: d.device_name,
                rect: d.rect,
            }),
            last_playlist: f.last_playlist.map(PlaylistId::from_uuid),
        }
    }
}

// --- Library: domain -> file ------------------------------------------------

impl From<&TextStyle> for TextStyleV1 {
    fn from(s: &TextStyle) -> Self {
        let rgba = |c: Rgba| [c.r, c.g, c.b, c.a];
        Self {
            font: match s.font {
                FontFamily::NotoSans => "NotoSans".into(),
            },
            size: s.size,
            line_height: s.line_height,
            letter_spacing: s.letter_spacing,
            h_align: match s.h_align {
                HAlign::Left => "Left",
                HAlign::Center => "Center",
                HAlign::Right => "Right",
            }
            .into(),
            v_align: match s.v_align {
                VAlign::Top => "Top",
                VAlign::Middle => "Middle",
                VAlign::Bottom => "Bottom",
            }
            .into(),
            text_color: rgba(s.text_color),
            background: rgba(s.background),
        }
    }
}

impl From<&Library> for LibraryFileV1 {
    fn from(lib: &Library) -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            songs: lib
                .songs()
                .iter()
                .map(|s| SongV1 {
                    id: s.id.as_uuid(),
                    title: s.title.clone(),
                    lyrics_source: s.lyrics_source.clone(),
                    style: (&s.style).into(),
                    created_at: s.created_at,
                    updated_at: s.updated_at,
                })
                .collect(),
            playlists: lib
                .playlists()
                .iter()
                .map(|p| PlaylistV1 {
                    id: p.id.as_uuid(),
                    name: p.name.clone(),
                    entries: p
                        .entries
                        .iter()
                        .map(|e| PlaylistEntryV1 {
                            id: e.id.as_uuid(),
                            song: e.song.as_uuid(),
                        })
                        .collect(),
                })
                .collect(),
            templates: lib
                .templates()
                .iter()
                .map(|t| TemplateV1 {
                    id: t.id.as_uuid(),
                    name: t.name.clone(),
                    style: (&t.style).into(),
                })
                .collect(),
        }
    }
}

// --- Library: file -> domain (validating) -----------------------------------

/// Why a stored value was rejected; becomes [`CoreError::Corrupt`].
fn invalid(what: &str, value: &str) -> String {
    format!("invalid {what} {value:?}")
}

impl TryFrom<TextStyleV1> for TextStyle {
    type Error = String;

    fn try_from(s: TextStyleV1) -> Result<Self, String> {
        let rgba = |c: [u8; 4]| Rgba {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        };
        let style = TextStyle {
            font: match s.font.as_str() {
                "NotoSans" => FontFamily::NotoSans,
                other => return Err(invalid("font", other)),
            },
            size: s.size,
            line_height: s.line_height,
            letter_spacing: s.letter_spacing,
            h_align: match s.h_align.as_str() {
                "Left" => HAlign::Left,
                "Center" => HAlign::Center,
                "Right" => HAlign::Right,
                other => return Err(invalid("horizontal alignment", other)),
            },
            v_align: match s.v_align.as_str() {
                "Top" => VAlign::Top,
                "Middle" => VAlign::Middle,
                "Bottom" => VAlign::Bottom,
                other => return Err(invalid("vertical alignment", other)),
            },
            text_color: rgba(s.text_color),
            background: rgba(s.background),
        };
        style.validate().map_err(|e| e.to_string())?;
        Ok(style)
    }
}

/// A library converted from a file, plus how many dangling playlist entries
/// were dropped.
pub struct Converted {
    /// The validated library.
    pub library: Library,
    /// Entries that pointed at missing songs and were dropped.
    pub dropped_entries: usize,
}

impl TryFrom<LibraryFileV1> for Converted {
    type Error = String;

    fn try_from(file: LibraryFileV1) -> Result<Self, String> {
        let mut song_ids = HashSet::new();
        let mut songs = Vec::with_capacity(file.songs.len());
        for s in file.songs {
            if !song_ids.insert(s.id) {
                return Err(format!("duplicate song id {}", s.id));
            }
            if s.title.trim().is_empty() {
                return Err(format!("song {} has an empty title", s.id));
            }
            songs.push(Song {
                id: SongId::from_uuid(s.id),
                title: s.title,
                lyrics_source: s.lyrics_source,
                style: s.style.try_into()?,
                created_at: s.created_at,
                updated_at: s.updated_at,
            });
        }

        let mut dropped_entries = 0;
        let playlists = file
            .playlists
            .into_iter()
            .map(|p| {
                let before = p.entries.len();
                let entries: Vec<_> = p
                    .entries
                    .into_iter()
                    .filter(|e| song_ids.contains(&e.song))
                    .map(|e| PlaylistEntry {
                        id: EntryId::from_uuid(e.id),
                        song: SongId::from_uuid(e.song),
                    })
                    .collect();
                dropped_entries += before - entries.len();
                Playlist {
                    id: PlaylistId::from_uuid(p.id),
                    name: p.name,
                    entries,
                }
            })
            .collect();

        let templates = file
            .templates
            .into_iter()
            .map(|t| {
                Ok(Template {
                    id: TemplateId::from_uuid(t.id),
                    name: t.name,
                    style: t.style.try_into()?,
                })
            })
            .collect::<Result<_, String>>()?;

        Ok(Converted {
            library: Library::from_parts(songs, playlists, templates),
            dropped_entries,
        })
    }
}
