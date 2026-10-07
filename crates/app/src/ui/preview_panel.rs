//! Selected-slide preview (teal) and live-output preview (red).

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    super::placeholder(ui, "Preview / Live", "T2.2");
}
