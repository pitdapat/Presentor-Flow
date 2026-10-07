//! Top toolbar: Add Song, New Playlist, Templates, display selector, output toggle.

use crate::actions::Action;
use crate::state::AppState;

use super::{theme, View};

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, view: &mut View<'_>) {
    ui.horizontal(|ui| {
        ui.strong("Presenter Flow");
        ui.separator();
        if ui.button("➕ Add Song").clicked() {
            view.push(Action::OpenEditor(None));
        }
        super::pending_button(ui, "New Playlist", "T3.2");
        super::pending_button(ui, "Templates", "T4.2");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            output_toggle(ui, state, view);
            display_selector(ui, state, view);
            ui.label("Output display:");
        });
    });
}

fn output_toggle(ui: &mut egui::Ui, state: &AppState, view: &mut View<'_>) {
    if state.output_open {
        let button = egui::Button::new(egui::RichText::new("Output: ON").color(theme::ON_ACCENT))
            .fill(theme::LIVE_RED);
        if ui
            .add(button)
            .on_hover_text("Close the output window")
            .clicked()
        {
            view.push(Action::CloseOutput);
        }
    } else {
        let enabled = state.output_display().is_some();
        let response = ui
            .add_enabled(enabled, egui::Button::new("Output: Off"))
            .on_hover_text("Open the output window on the chosen display")
            .on_disabled_hover_text("Choose an output display first");
        if response.clicked() {
            view.push(Action::OpenOutput);
        }
    }
}

fn display_selector(ui: &mut egui::Ui, state: &AppState, view: &mut View<'_>) {
    let selected = match (state.output_display(), &state.settings.output_display) {
        (Some(d), _) => d.friendly_name.clone(),
        (None, Some(_)) => "Saved display not found — choose".into(),
        (None, None) => "Choose a display…".into(),
    };
    egui::ComboBox::from_id_salt("output_display")
        .selected_text(selected)
        .width(260.0)
        .show_ui(ui, |ui| {
            if state.displays.is_empty() {
                ui.weak("No displays found");
            }
            for display in &state.displays {
                let is_chosen = state
                    .settings
                    .output_display
                    .as_ref()
                    .is_some_and(|r| display.matches(r));
                let mut label = display.friendly_name.clone();
                if display.is_primary {
                    label.push_str(" — test window");
                }
                if ui.selectable_label(is_chosen, label).clicked() && !is_chosen {
                    view.push(Action::ChooseDisplay(display.to_ref()));
                }
            }
        })
        .response
        .on_hover_text(
            "The projector or TV for the audience. Choosing this screen opens a \
             test window instead of full screen.",
        );
}
