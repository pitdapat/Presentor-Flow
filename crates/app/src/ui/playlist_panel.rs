//! Playlists and the ordered entries of the selected playlist.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    super::placeholder(ui, "Playlists", "T3.2");
    ui.horizontal(|ui| {
        super::pending_button(ui, "Move Up", "T3.2");
        super::pending_button(ui, "Move Down", "T3.2");
        super::pending_button(ui, "Remove", "T3.2");
    });
}
