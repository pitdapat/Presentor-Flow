//! Template selector, style controls, Save as Template.

use crate::state::AppState;

use super::View;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _view: &mut View<'_>) {
    super::placeholder(ui, "Style", "T4.1 / T4.2");
    ui.weak("Songs use the built-in Centered Lyrics style for now.");
    super::pending_button(ui, "Save as Template", "T4.2");
}
