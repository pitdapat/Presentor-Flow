//! Playlists and the ordered entries of the selected playlist.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    super::placeholder(ui, "Playlists", "T3.2");
}
