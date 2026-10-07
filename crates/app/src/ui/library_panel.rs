//! Library search and song list.

use crate::actions::Action;
use crate::state::AppState;

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, _actions: &mut Vec<Action>) {
    ui.heading("Library");
    // Read-only copy until T1.4 adds a `SetSearch` action.
    let mut search = state.ui.search.clone();
    ui.add_enabled(
        false,
        egui::TextEdit::singleline(&mut search).hint_text("Search titles…"),
    )
    .on_disabled_hover_text("Built in T1.4");
    ui.weak("Song list — built in T1.4");
}
