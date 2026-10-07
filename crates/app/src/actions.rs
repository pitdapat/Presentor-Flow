//! Every state change the UI can request, and the single function that
//! applies them (PLAN §3.3).

use presenter_core::domain::{MoveDir, TextStyle};
use presenter_core::ids::{EntryId, PlaylistId, SongId, TemplateId};
use presenter_core::settings::DisplayRef;

use crate::error::AppError;
use crate::state::AppState;

/// A requested state change. UI panels push these; they never mutate state.
#[derive(Debug, Clone)]
pub enum Action {
    // Library
    CreateSong {
        title: String,
        lyrics: String,
    },
    UpdateSong {
        id: SongId,
        title: String,
        lyrics: String,
    },
    DeleteSong(SongId),
    SetSongStyle {
        id: SongId,
        style: TextStyle,
    },
    ApplyTemplate {
        song: SongId,
        template: TemplateId,
    },
    SaveTemplate {
        name: String,
        style: TextStyle,
    },
    CreatePlaylist {
        name: String,
    },
    RenamePlaylist {
        id: PlaylistId,
        name: String,
    },
    DeletePlaylist(PlaylistId),
    AddToPlaylist {
        playlist: PlaylistId,
        song: SongId,
    },
    MoveEntry {
        playlist: PlaylistId,
        entry: EntryId,
        dir: MoveDir,
    },
    RemoveEntry {
        playlist: PlaylistId,
        entry: EntryId,
    },
    // Selection (never touches live output)
    SelectSong(SongId),
    SelectSlide(usize),
    // Presentation
    GoLive {
        song: SongId,
        slide: usize,
    },
    NextSlide,
    PreviousSlide,
    ClearLyrics,
    ToggleBlackout,
    // Output
    ChooseDisplay(DisplayRef),
    CloseOutput,
}

/// Applies one action. Errors are shown in the status bar; never panics.
pub fn apply(state: &mut AppState, action: Action) -> Result<(), AppError> {
    match action {
        Action::SelectSong(id) => {
            state.ui.selected_song = Some(id);
            state.ui.selected_slide = None;
        }
        Action::SelectSlide(index) => state.ui.selected_slide = Some(index),
        Action::ToggleBlackout => state.live.toggle_blackout(),
        other => return Err(AppError::NotImplemented(format!("{other:?}"))),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_song_does_not_touch_live_output() {
        let mut state = AppState::default();
        let id = SongId::new();
        assert!(apply(&mut state, Action::SelectSong(id)).is_ok());
        assert_eq!(state.ui.selected_song, Some(id));
        assert!(matches!(
            state.live.content(),
            presenter_core::presentation::LiveContent::Empty
        ));
    }

    #[test]
    fn unimplemented_actions_return_an_error_not_a_panic() {
        let mut state = AppState::default();
        assert!(apply(&mut state, Action::NextSlide).is_err());
    }
}
