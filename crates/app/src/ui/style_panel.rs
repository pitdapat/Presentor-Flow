//! Template selector, style controls, Save as Template.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    super::placeholder(ui, "Style", "T4.1 / T4.2");
    super::pending_button(ui, "Save as Template", "T4.2");
}
