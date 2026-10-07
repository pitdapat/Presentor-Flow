//! Song title, Edit Lyrics, and the slide thumbnail grid.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, _state: &AppState, _actions: &mut Vec<Action>) {
    super::placeholder(ui, "Slides", "T1.4 / T2.2");
}
