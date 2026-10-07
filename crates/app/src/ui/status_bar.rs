//! Bottom bar: Previous, Next, Clear Lyrics, Blackout; save and output status.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, actions: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        if ui.button("Blackout").clicked() {
            actions.push(Action::ToggleBlackout);
        }
        if state.live.is_blackout() {
            ui.colored_label(egui::Color32::RED, "BLACKOUT");
        }
        ui.separator();
        match &state.ui.status_message {
            Some(msg) => ui.label(msg),
            None => ui.weak("Ready"),
        };
    });
}
