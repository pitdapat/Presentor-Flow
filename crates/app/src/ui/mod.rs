//! Operator window panels (PLAN §4). Every panel takes `&AppState` and
//! pushes [`Action`]s; none of them mutates state.

pub mod dialogs;
mod library_panel;
mod playlist_panel;
mod preview_panel;
mod slide_grid;
mod status_bar;
mod style_panel;
mod toolbar;

use crate::actions::Action;
use crate::state::AppState;

/// Draws the whole operator window for one frame.
pub fn draw(root: &mut egui::Ui, state: &AppState, actions: &mut Vec<Action>) {
    egui::Panel::top("toolbar").show(root, |ui| toolbar::show(ui, state, actions));
    egui::Panel::bottom("status_bar").show(root, |ui| status_bar::show(ui, state, actions));

    egui::Panel::left("left")
        .resizable(true)
        .min_size(220.0)
        .show(root, |ui| {
            library_panel::show(ui, state, actions);
            ui.separator();
            playlist_panel::show(ui, state, actions);
        });

    egui::Panel::right("right")
        .resizable(true)
        .min_size(320.0)
        .show(root, |ui| {
            preview_panel::show(ui, state, actions);
            ui.separator();
            style_panel::show(ui, state, actions);
        });

    egui::CentralPanel::default().show(root, |ui| slide_grid::show(ui, state, actions));
}

/// Placeholder text used by panels that are not built yet.
fn placeholder(ui: &mut egui::Ui, title: &str, task: &str) {
    ui.heading(title);
    ui.weak(format!("Placeholder — built in {task}"));
}
