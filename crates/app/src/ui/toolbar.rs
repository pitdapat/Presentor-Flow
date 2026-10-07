//! Top toolbar: Add Song, New Playlist, Templates, display selector, output toggle.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        ui.strong("Presenter Flow");
        ui.separator();
        super::pending_button(ui, "Add Song", "T1.4");
        super::pending_button(ui, "New Playlist", "T3.2");
        super::pending_button(ui, "Templates", "T4.2");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            super::pending_button(ui, "Output: Off", "T0.4");
            ui.add_enabled_ui(false, |ui| {
                egui::ComboBox::from_id_salt("output_display")
                    .selected_text("No display")
                    .show_ui(ui, |_ui| {});
            })
            .response
            .on_disabled_hover_text("Display list built in T0.3");
            ui.label("Output display:");
        });
    });
}
