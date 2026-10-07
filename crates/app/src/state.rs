//! All application state. UI code reads it; only `actions::apply` writes it.

use presenter_core::ids::{PlaylistId, SongId};
use presenter_core::library::Library;
use presenter_core::presentation::LiveState;
use presenter_core::settings::Settings;

/// Everything the app knows.
#[derive(Debug, Default)]
pub struct AppState {
    /// Songs, playlists and templates.
    pub library: Library,
    /// What is on the projector.
    pub live: LiveState,
    /// Persisted user settings.
    pub settings: Settings,
    /// Transient UI-only state (never saved to the library).
    pub ui: UiState,
}

/// UI selection and transient messages.
#[derive(Debug, Default)]
pub struct UiState {
    /// Song selected in the library or playlist (not necessarily live).
    pub selected_song: Option<SongId>,
    /// Slide selected for preview (not necessarily live).
    pub selected_slide: Option<usize>,
    /// Playlist shown in the playlist panel.
    pub selected_playlist: Option<PlaylistId>,
    /// Library search text.
    pub search: String,
    /// Last error or info message for the status bar.
    pub status_message: Option<String>,
}
