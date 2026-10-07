//! Output viewport lifecycle (PLAN §3.7): borderless, fullscreen on the
//! chosen monitor, created with `show_viewport_immediate` (T0.4, T2.4).

use crate::actions::Action;
use crate::state::AppState;

/// Shows the output viewport if a display is chosen.
pub fn show(ctx: &egui::Context, state: &AppState, actions: &mut Vec<Action>) {
    let _ = (ctx, state, actions);
    todo!("T0.4: show_viewport_immediate on the chosen monitor")
}
