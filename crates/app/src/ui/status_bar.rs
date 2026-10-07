//! Bottom bar: Previous, Next, Clear Lyrics, Blackout; save and output status.

use crate::actions::Action;
use crate::state::{AppState, SaveStatus};

use super::{theme, View};

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, view: &mut View<'_>) {
    ui.horizontal(|ui| {
        if ui
            .button("◀ Previous")
            .on_hover_text("← ↑ PageUp")
            .clicked()
        {
            view.push(Action::PreviousSlide);
        }
        if ui
            .button("Next ▶")
            .on_hover_text("→ ↓ Space PageDown")
            .clicked()
        {
            view.push(Action::NextSlide);
        }
        if ui.button("Clear Lyrics").on_hover_text("C").clicked() {
            view.push(Action::ClearLyrics);
        }

        let blackout = state.live.is_blackout();
        let button = if blackout {
            egui::Button::new(egui::RichText::new("Blackout: ON").color(theme::ON_ACCENT))
                .fill(theme::LIVE_RED)
        } else {
            egui::Button::new("Blackout")
        };
        if ui.add(button).on_hover_text("B").clicked() {
            view.push(Action::ToggleBlackout);
        }

        ui.separator();
        match &state.ui.status_message {
            Some(msg) if msg.is_error => ui.colored_label(theme::LIVE_RED, &msg.text),
            Some(msg) => ui.label(&msg.text),
            None => ui.weak("Ready"),
        };

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            match state.output_display() {
                Some(d) if state.output_open => {
                    ui.colored_label(theme::LIVE_RED, format!("Output: {}", d.friendly_name))
                }
                Some(_) => ui.weak("Output: closed"),
                None => ui.weak("Output: no display chosen"),
            };
            ui.separator();
            match &state.ui.save_status {
                SaveStatus::Idle => ui.weak("No changes"),
                SaveStatus::Unsaved => ui.weak("Saving…"),
                SaveStatus::Saved => ui.weak("All changes saved"),
                SaveStatus::Failed(err) => ui
                    .colored_label(theme::LIVE_RED, "⚠ Save failed")
                    .on_hover_text(err),
            };
        });
    });
}
