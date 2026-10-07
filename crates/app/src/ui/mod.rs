//! Operator window panels (PLAN §4). Every panel takes `&AppState` and
//! pushes [`Action`]s; none of them mutates state.

pub mod dialogs;
mod library_panel;
mod playlist_panel;
mod preview_panel;
mod slide_grid;
mod status_bar;
mod style_panel;
pub mod theme;
mod toolbar;

use crate::actions::Action;
use crate::state::AppState;

/// Minimum width of the left (library / playlists) column.
const LEFT_MIN_WIDTH: f32 = 240.0;
/// Minimum width of the right (preview / style) column.
const RIGHT_MIN_WIDTH: f32 = 340.0;
/// Minimum height of each half of a split column.
const HALF_MIN_HEIGHT: f32 = 160.0;

/// Draws the whole operator window for one frame.
pub fn draw(root: &mut egui::Ui, state: &AppState, actions: &mut Vec<Action>) {
    egui::Panel::top("toolbar").show(root, |ui| toolbar::show(ui, state, actions));
    if state.live.is_blackout() {
        blackout_banner(root);
    }
    egui::Panel::bottom("status_bar").show(root, |ui| status_bar::show(ui, state, actions));

    egui::Panel::left("left")
        .resizable(true)
        .min_size(LEFT_MIN_WIDTH)
        .default_size(280.0)
        .show(root, |ui| {
            let halves = (
                library_panel::show as PanelFn,
                playlist_panel::show as PanelFn,
            );
            split_column(ui, "left_split", halves, state, actions);
        });

    egui::Panel::right("right")
        .resizable(true)
        .min_size(RIGHT_MIN_WIDTH)
        .default_size(400.0)
        .show(root, |ui| {
            let halves = (preview_panel::show as PanelFn, style_panel::show as PanelFn);
            split_column(ui, "right_split", halves, state, actions);
        });

    egui::CentralPanel::default().show(root, |ui| slide_grid::show(ui, state, actions));
}

/// Signature shared by every panel's `show` function.
type PanelFn = fn(&mut egui::Ui, &AppState, &mut Vec<Action>);

/// Splits a column into a resizable upper half and a lower half.
fn split_column(
    ui: &mut egui::Ui,
    id: &str,
    (upper, lower): (PanelFn, PanelFn),
    state: &AppState,
    actions: &mut Vec<Action>,
) {
    let half = (ui.available_height() / 2.0).max(HALF_MIN_HEIGHT);
    egui::Panel::top(egui::Id::new(id))
        .resizable(true)
        .min_size(HALF_MIN_HEIGHT)
        .default_size(half)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt((id, "upper"))
                .auto_shrink(false)
                .show(ui, |ui| upper(ui, state, actions));
        });
    egui::ScrollArea::vertical()
        .id_salt((id, "lower"))
        .auto_shrink(false)
        .show(ui, |ui| lower(ui, state, actions));
}

/// Large red banner shown in the operator window while blackout is on.
fn blackout_banner(root: &mut egui::Ui) {
    egui::Panel::top("blackout_banner")
        .frame(
            egui::Frame::new()
                .fill(theme::LIVE_RED)
                .inner_margin(egui::Margin::symmetric(12, 8)),
        )
        .show(root, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("BLACKOUT — projector is black")
                        .size(22.0)
                        .strong()
                        .color(theme::ON_ACCENT),
                );
            });
        });
}

/// Placeholder text used by panels that are not built yet.
fn placeholder(ui: &mut egui::Ui, title: &str, task: &str) {
    ui.heading(title);
    ui.weak(format!("Placeholder — built in {task}"));
}

/// A button for an action whose task is not built yet: shown so the layout
/// matches PLAN §4, disabled, and labeled with the task that enables it.
fn pending_button(ui: &mut egui::Ui, label: &str, task: &str) {
    ui.add_enabled(false, egui::Button::new(label))
        .on_disabled_hover_text(format!("Built in {task}"));
}
