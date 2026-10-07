//! Bottom bar: Previous, Next, Clear Lyrics, Blackout; save and output status.

use crate::actions::Action;
use crate::state::AppState;

use super::theme;

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, actions: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        // Navigation actions are wired now; `apply` reports them as not
        // implemented until T2.1 builds the live-state rules.
        if ui.button("◀ Previous").clicked() {
            actions.push(Action::PreviousSlide);
        }
        if ui.button("Next ▶").clicked() {
            actions.push(Action::NextSlide);
        }
        if ui.button("Clear Lyrics").clicked() {
            actions.push(Action::ClearLyrics);
        }

        let blackout = state.live.is_blackout();
        let label = if blackout { "Blackout: ON" } else { "Blackout" };
        let mut button = egui::Button::new(label);
        if blackout {
            button = button.fill(theme::LIVE_RED);
        }
        if ui.add(button).clicked() {
            actions.push(Action::ToggleBlackout);
        }

        ui.separator();
        match &state.ui.status_message {
            Some(msg) => ui.label(msg),
            None => ui.weak("Ready"),
        };

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Real values arrive with storage (T1.3) and the output window (T0.4).
            ui.weak("Output: off");
            ui.separator();
            ui.weak("Not saved yet");
        });
    });
}
