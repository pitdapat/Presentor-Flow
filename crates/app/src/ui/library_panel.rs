//! Library search and song list.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    super::placeholder(ui, "Library", "T1.4");
}
