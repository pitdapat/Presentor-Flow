//! Top toolbar: Add Song, New Playlist, Templates, display selector, output toggle.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        ui.strong("Presenter Flow");
        ui.separator();
        ui.weak("Toolbar — built in T0.2 / T0.3");
    });
}
