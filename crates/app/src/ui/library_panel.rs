//! Library search and song list.

use crate::actions::Action;
use crate::state::AppState;

use super::{theme, View};

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, view: &mut View<'_>) {
    ui.heading("Library");

    let mut search = state.ui.search.clone();
    let response = ui.add(
        egui::TextEdit::singleline(&mut search)
            .hint_text("🔍 Search titles…")
            .desired_width(f32::INFINITY),
    );
    if response.changed() {
        view.push(Action::SetSearch(search));
    }
    ui.add_space(4.0);

    if state.library.songs().is_empty() {
        ui.weak("No songs yet. Click ➕ Add Song.");
        return;
    }

    let mut any = false;
    for song in state.filtered_songs() {
        any = true;
        let selected = state.ui.selected_song == Some(song.id);
        let is_live = matches!(
            state.live.content(),
            presenter_core::presentation::LiveContent::Slide(s) if s.song == song.id
        );
        let mut text = egui::RichText::new(&song.title);
        if is_live {
            text = text.color(theme::LIVE_RED);
        }
        let response = ui
            .add_sized(
                [ui.available_width(), 24.0],
                egui::Button::selectable(selected, text),
            )
            .on_hover_text("Click to select · double-click to edit lyrics");
        if response.double_clicked() {
            view.push(Action::OpenEditor(Some(song.id)));
        } else if response.clicked() {
            view.push(Action::SelectSong(song.id));
        }
    }
    if !any {
        ui.weak("No song title matches the search.");
    }
}
