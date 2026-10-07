//! Every state change the UI can request, and the single function that
//! applies them (PLAN §3.3).

use presenter_core::domain::{MoveDir, TextStyle};
use presenter_core::ids::{EntryId, PlaylistId, SongId, TemplateId};
use presenter_core::settings::DisplayRef;

use crate::autosave::DirtyKind;
use crate::error::AppError;
use crate::platform::DisplayInfo;
use crate::state::{AppState, EditorState};

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
    // Song editor dialog
    /// Opens the editor for a song, or for a new song with `None`.
    OpenEditor(Option<SongId>),
    /// The draft title or lyrics changed.
    EditDraft {
        title: String,
        lyrics: String,
    },
    /// First press of Delete in the editor; the second press deletes.
    ConfirmDeleteInEditor(bool),
    CloseEditor,
    // Selection (never touches live output)
    SetSearch(String),
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
    OpenOutput,
    CloseOutput,
    /// The periodic monitor scan finished.
    DisplaysRefreshed(Vec<DisplayInfo>),
}

/// Applies one action. Errors are shown in the status bar; never panics.
pub fn apply(state: &mut AppState, action: Action) -> Result<(), AppError> {
    match action {
        Action::CreateSong { title, lyrics } => {
            let id = state.library.create_song(&title, &lyrics)?;
            library_changed(state);
            state.ui.editor = None;
            select_song(state, id);
            state.ui.info(format!("Added \"{}\".", title.trim()));
        }
        Action::UpdateSong { id, title, lyrics } => {
            state.library.update_song(id, &title, &lyrics)?;
            library_changed(state);
            state.ui.editor = None;
            // Keep the preview on a slide that still exists.
            let count = state.library.song(id).map_or(0, |s| s.slides().len());
            if state.ui.selected_slide.is_some_and(|i| i >= count) {
                state.ui.selected_slide = None;
            }
            state
                .ui
                .info("Song saved. The projector changes when you present a slide.");
        }
        Action::DeleteSong(id) => {
            let title = state.library.song(id).map(|s| s.title.clone());
            state.library.delete_song(id)?;
            library_changed(state);
            state.ui.editor = None;
            if state.ui.selected_song == Some(id) {
                state.ui.selected_song = None;
                state.ui.selected_slide = None;
            }
            state
                .ui
                .info(format!("Deleted \"{}\".", title.unwrap_or_default()));
        }
        Action::OpenEditor(song) => {
            let draft = match song {
                Some(id) => {
                    let s = state
                        .library
                        .song(id)
                        .ok_or(presenter_core::CoreError::SongNotFound(id))?;
                    EditorState {
                        song: Some(id),
                        title: s.title.clone(),
                        lyrics: s.lyrics_source.clone(),
                        confirm_delete: false,
                    }
                }
                None => EditorState::default(),
            };
            state.ui.editor = Some(draft);
        }
        Action::EditDraft { title, lyrics } => {
            if let Some(editor) = &mut state.ui.editor {
                editor.title = title;
                editor.lyrics = lyrics;
            }
        }
        Action::ConfirmDeleteInEditor(on) => {
            if let Some(editor) = &mut state.ui.editor {
                editor.confirm_delete = on;
            }
        }
        Action::CloseEditor => state.ui.editor = None,
        Action::SetSearch(text) => state.ui.search = text,
        Action::SelectSong(id) => select_song(state, id),
        Action::SelectSlide(index) => state.ui.selected_slide = Some(index),
        Action::GoLive { song, slide } => {
            state.live.go_live(&state.library, song, slide)?;
            state.ui.selected_song = Some(song);
            state.ui.selected_slide = Some(slide);
        }
        Action::NextSlide => {
            state.live.next(&state.library)?;
            follow_live(state);
        }
        Action::PreviousSlide => {
            state.live.previous(&state.library)?;
            follow_live(state);
        }
        Action::ClearLyrics => state.live.clear_lyrics(),
        Action::ToggleBlackout => state.live.toggle_blackout(),
        Action::ChooseDisplay(display) => {
            state.settings.output_display = Some(display);
            state.pending.settings = true;
        }
        Action::OpenOutput => {
            if state.output_display().is_none() {
                return Err(AppError::Platform(
                    "Choose an output display in the toolbar first.".into(),
                ));
            }
            state.output_open = true;
        }
        Action::CloseOutput => state.output_open = false,
        Action::DisplaysRefreshed(displays) => {
            state.displays = displays;
            if state.output_open && state.output_display().is_none() {
                // Never move output onto another screen (PLAN §3.7).
                state.output_open = false;
                state
                    .ui
                    .error("The output display was disconnected. Output is closed.");
            }
        }
        other => return Err(AppError::NotImplemented(format!("{other:?}"))),
    }
    Ok(())
}

/// Marks the library for an immediate save.
fn library_changed(state: &mut AppState) {
    state.pending.library = Some(DirtyKind::Immediate);
}

fn select_song(state: &mut AppState, id: SongId) {
    if state.ui.selected_song != Some(id) {
        state.ui.selected_slide = None;
    }
    state.ui.selected_song = Some(id);
}

/// After Next/Previous, the selection follows the live slide so the operator
/// sees where they are.
fn follow_live(state: &mut AppState) {
    if let presenter_core::presentation::LiveContent::Slide(snap) = state.live.content() {
        state.ui.selected_song = Some(snap.song);
        state.ui.selected_slide = Some(snap.slide_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use presenter_core::presentation::LiveContent;

    type TestResult = Result<(), AppError>;

    fn with_song() -> Result<(AppState, SongId), AppError> {
        let mut state = AppState::default();
        apply(
            &mut state,
            Action::CreateSong {
                title: "Song".into(),
                lyrics: "one\n\ntwo".into(),
            },
        )?;
        let id = state.library.songs()[0].id;
        Ok((state, id))
    }

    #[test]
    fn select_song_does_not_touch_live_output() {
        let mut state = AppState::default();
        let id = SongId::new();
        assert!(apply(&mut state, Action::SelectSong(id)).is_ok());
        assert_eq!(state.ui.selected_song, Some(id));
        assert!(matches!(state.live.content(), LiveContent::Empty));
    }

    #[test]
    fn create_song_saves_selects_and_closes_editor() -> TestResult {
        let mut state = AppState::default();
        apply(&mut state, Action::OpenEditor(None))?;
        apply(
            &mut state,
            Action::CreateSong {
                title: "Song".into(),
                lyrics: "x".into(),
            },
        )?;
        let id = state.library.songs()[0].id;
        assert_eq!(state.ui.selected_song, Some(id));
        assert!(state.ui.editor.is_none());
        assert_eq!(state.pending.library, Some(DirtyKind::Immediate));
        Ok(())
    }

    #[test]
    fn invalid_song_keeps_the_editor_open() -> TestResult {
        let mut state = AppState::default();
        apply(&mut state, Action::OpenEditor(None))?;
        let result = apply(
            &mut state,
            Action::CreateSong {
                title: String::new(),
                lyrics: "x".into(),
            },
        );
        assert!(result.is_err());
        assert!(state.ui.editor.is_some());
        assert!(state.pending.library.is_none());
        Ok(())
    }

    #[test]
    fn preview_selection_never_changes_live_output() -> TestResult {
        let (mut state, id) = with_song()?;
        apply(&mut state, Action::GoLive { song: id, slide: 0 })?;
        apply(&mut state, Action::SelectSlide(1))?;
        let LiveContent::Slide(snap) = state.live.content() else {
            return Err(AppError::Platform("expected a live slide".into()));
        };
        assert_eq!(snap.slide_index, 0);
        assert_eq!(state.ui.selected_slide, Some(1));
        Ok(())
    }

    #[test]
    fn next_moves_live_and_selection_together() -> TestResult {
        let (mut state, id) = with_song()?;
        apply(&mut state, Action::GoLive { song: id, slide: 0 })?;
        apply(&mut state, Action::NextSlide)?;
        assert_eq!(state.ui.selected_slide, Some(1));
        Ok(())
    }

    #[test]
    fn open_output_needs_a_chosen_display() {
        let mut state = AppState::default();
        assert!(apply(&mut state, Action::OpenOutput).is_err());
        assert!(!state.output_open);
    }

    #[test]
    fn output_closes_when_its_display_disappears() -> TestResult {
        let mut state = AppState::default();
        let display = DisplayInfo {
            device_name: r"\\.\DISPLAY2".into(),
            friendly_name: "Display 2".into(),
            rect: [1920, 0, 3840, 1080],
            is_primary: false,
            scale_factor: 1.0,
            os_index: 1,
        };
        apply(&mut state, Action::DisplaysRefreshed(vec![display.clone()]))?;
        apply(&mut state, Action::ChooseDisplay(display.to_ref()))?;
        apply(&mut state, Action::OpenOutput)?;
        assert!(state.output_open);
        apply(&mut state, Action::DisplaysRefreshed(Vec::new()))?;
        assert!(!state.output_open);
        assert!(state.ui.status_message.as_ref().is_some_and(|m| m.is_error));
        Ok(())
    }

    #[test]
    fn deleting_the_selected_song_clears_selection_not_live() -> TestResult {
        let (mut state, id) = with_song()?;
        apply(&mut state, Action::GoLive { song: id, slide: 1 })?;
        apply(&mut state, Action::DeleteSong(id))?;
        assert!(state.ui.selected_song.is_none());
        assert!(matches!(state.live.content(), LiveContent::Slide(_)));
        Ok(())
    }
}
