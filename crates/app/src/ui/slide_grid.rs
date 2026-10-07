//! Song title, Edit Lyrics, and the slide thumbnail grid.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        ui.heading("No song selected");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            super::pending_button(ui, "Edit Lyrics", "T1.4");
        });
    });
    ui.separator();
    ui.weak("Slide thumbnails — built in T1.4 / T2.2");
}
