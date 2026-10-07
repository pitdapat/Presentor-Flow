//! All application state. UI code reads it; only `actions::apply` writes it.

use presenter_core::ids::{PlaylistId, SongId};
use presenter_core::library::Library;
use presenter_core::presentation::LiveState;
use presenter_core::settings::Settings;

use crate::autosave::DirtyKind;
use crate::platform::DisplayInfo;

/// Everything the app knows.
#[derive(Debug, Default)]
pub struct AppState {
    /// Songs, playlists and templates.
    pub library: Library,
    /// What is on the projector.
    pub live: LiveState,
    /// Persisted user settings.
    pub settings: Settings,
    /// Attached monitors, refreshed every 2 seconds.
    pub displays: Vec<DisplayInfo>,
    /// Whether the output window is open.
    pub output_open: bool,
    /// Pending saves, set by `apply` and consumed by the app.
    pub pending: Pending,
    /// Transient UI-only state (never saved to the library).
    pub ui: UiState,
}

/// Work `apply` asks the app to do after the frame.
#[derive(Debug, Default)]
pub struct Pending {
    /// The library changed and must be saved.
    pub library: Option<DirtyKind>,
    /// The settings changed and must be saved.
    pub settings: bool,
}

/// Result of the last save, shown in the status bar.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SaveStatus {
    /// Nothing has been saved this session and nothing is unsaved.
    #[default]
    Idle,
    /// Changes are waiting to be saved.
    Unsaved,
    /// The last save succeeded.
    Saved,
    /// The last save failed; retried on the next change.
    Failed(String),
}

/// A status-bar message and how serious it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusMessage {
    /// Text shown to the operator.
    pub text: String,
    /// Shown in red when true.
    pub is_error: bool,
}

/// Draft contents of the song editor dialog.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EditorState {
    /// The song being edited, or `None` for a new song.
    pub song: Option<SongId>,
    /// Draft title.
    pub title: String,
    /// Draft lyrics source.
    pub lyrics: String,
    /// The delete button was pressed once and asks for confirmation.
    pub confirm_delete: bool,
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
    /// The open song editor, if any.
    pub editor: Option<EditorState>,
    /// Last error or info message for the status bar.
    pub status_message: Option<StatusMessage>,
    /// Result of the last library save.
    pub save_status: SaveStatus,
}

impl UiState {
    /// Shows an informational message in the status bar.
    pub fn info(&mut self, text: impl Into<String>) {
        self.status_message = Some(StatusMessage {
            text: text.into(),
            is_error: false,
        });
    }

    /// Shows an error in the status bar (red).
    pub fn error(&mut self, text: impl Into<String>) {
        self.status_message = Some(StatusMessage {
            text: text.into(),
            is_error: true,
        });
    }
}

impl AppState {
    /// The display output goes to, if it is attached right now.
    pub fn output_display(&self) -> Option<&DisplayInfo> {
        let chosen = self.settings.output_display.as_ref()?;
        self.displays.iter().find(|d| d.matches(chosen))
    }

    /// Songs whose title contains the search text (case-insensitive), in
    /// library order.
    pub fn filtered_songs(&self) -> impl Iterator<Item = &presenter_core::domain::Song> {
        let needle = self.ui.search.trim().to_lowercase();
        self.library
            .songs()
            .iter()
            .filter(move |s| needle.is_empty() || s.title.to_lowercase().contains(&needle))
    }
}
