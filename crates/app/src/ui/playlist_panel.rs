//! Playlists and the ordered entries of the selected playlist.

use crate::state::AppState;

use super::View;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _view: &mut View<'_>) {
    super::placeholder(ui, "Playlists", "T3.2");
    ui.horizontal(|ui| {
        super::pending_button(ui, "Move Up", "T3.2");
        super::pending_button(ui, "Move Down", "T3.2");
        super::pending_button(ui, "Remove", "T3.2");
    });
}
