//! Add / edit song dialog: title plus raw lyrics text (T1.4).

use presenter_core::domain::parse_lyrics;

use crate::actions::Action;
use crate::state::{AppState, EditorState};
use crate::ui::{theme, View};

/// Help shown under the lyrics box.
const FORMAT_HELP: &str = "A blank line starts a new slide. A line like [Chorus] names the \
     section that follows (it is not shown on screen).";

/// Shows the editor as a modal dialog.
pub fn show(ctx: &egui::Context, state: &AppState, editor: &EditorState, view: &mut View<'_>) {
    let mut draft = editor.clone();
    let is_new = editor.song.is_none();

    let modal = egui::Modal::new(egui::Id::new("song_editor")).show(ctx, |ui| {
        ui.set_width(640.0);
        ui.heading(if is_new { "Add Song" } else { "Edit Song" });
        ui.add_space(6.0);

        ui.label("Title");
        ui.add(
            egui::TextEdit::singleline(&mut draft.title)
                .hint_text("Song title")
                .desired_width(f32::INFINITY),
        );
        ui.add_space(6.0);

        let slide_count = parse_lyrics(&draft.lyrics).slides.len();
        ui.horizontal(|ui| {
            ui.label("Lyrics");
            ui.weak(match slide_count {
                1 => "1 slide".to_owned(),
                n => format!("{n} slides"),
            });
        });
        egui::ScrollArea::vertical()
            .max_height(420.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut draft.lyrics)
                        .hint_text("[Verse 1]\nFirst line\nSecond line\n\n[Chorus]\n…")
                        .desired_rows(16)
                        .desired_width(f32::INFINITY),
                );
            });
        ui.weak(FORMAT_HELP);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let can_save = !draft.title.trim().is_empty() && slide_count > 0;
            let save = ui
                .add_enabled(can_save, egui::Button::new("Save"))
                .on_disabled_hover_text("A song needs a title and at least one slide");
            if save.clicked() {
                view.push(match draft.song {
                    Some(id) => Action::UpdateSong {
                        id,
                        title: draft.title.clone(),
                        lyrics: draft.lyrics.clone(),
                    },
                    None => Action::CreateSong {
                        title: draft.title.clone(),
                        lyrics: draft.lyrics.clone(),
                    },
                });
            }
            if ui.button("Cancel").on_hover_text("Esc").clicked() {
                view.push(Action::CloseEditor);
            }

            if let Some(id) = draft.song {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    delete_controls(ui, state, editor, id, view);
                });
            }
        });
    });

    if draft.title != editor.title || draft.lyrics != editor.lyrics {
        view.push(Action::EditDraft {
            title: draft.title,
            lyrics: draft.lyrics,
        });
    }
    // Esc or a click outside closes the dialog only when nothing would be
    // lost; with unsaved edits the operator must press Save or Cancel.
    if modal.should_close() && !has_unsaved_edits(state, editor) {
        view.push(Action::CloseEditor);
    }
}

/// Whether the draft differs from the saved song (or is non-empty for a new song).
fn has_unsaved_edits(state: &AppState, editor: &EditorState) -> bool {
    match editor.song.and_then(|id| state.library.song(id)) {
        Some(song) => editor.title != song.title || editor.lyrics != song.lyrics_source,
        None => !editor.title.trim().is_empty() || !editor.lyrics.trim().is_empty(),
    }
}

/// Delete needs two clicks: the first asks for confirmation.
fn delete_controls(
    ui: &mut egui::Ui,
    state: &AppState,
    editor: &EditorState,
    id: presenter_core::ids::SongId,
    view: &mut View<'_>,
) {
    if editor.confirm_delete {
        let delete = egui::Button::new(egui::RichText::new("Yes, delete").color(theme::ON_ACCENT))
            .fill(theme::LIVE_RED);
        if ui.add(delete).clicked() {
            view.push(Action::DeleteSong(id));
        }
        if ui.button("Keep").clicked() {
            view.push(Action::ConfirmDeleteInEditor(false));
        }
        let live = matches!(
            state.live.content(),
            presenter_core::presentation::LiveContent::Slide(s) if s.song == id
        );
        ui.colored_label(
            theme::LIVE_RED,
            if live {
                "Delete this song? (The projector keeps showing it until you change slide.)"
            } else {
                "Delete this song?"
            },
        );
    } else if ui.button("🗑 Delete Song").clicked() {
        view.push(Action::ConfirmDeleteInEditor(true));
    }
}
