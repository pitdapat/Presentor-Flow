//! Output viewport lifecycle (PLAN §3.7): borderless fullscreen on the
//! chosen monitor, created with `show_viewport_immediate` (T0.4, T2.4).
//!
//! If the chosen display is the operator's own screen, a normal resizable
//! test window opens instead, so the app can be tried with one monitor and
//! fullscreen output never covers the operator window.

use crate::actions::Action;
use crate::render::SlideRenderer;
use crate::state::AppState;

/// Size of the test window used when the output display is this screen.
const TEST_WINDOW_SIZE: [f32; 2] = [960.0, 540.0];

/// Shows the output viewport while output is open.
pub fn show(
    ctx: &egui::Context,
    state: &AppState,
    render: &mut SlideRenderer,
    actions: &mut Vec<Action>,
) {
    if !state.output_open {
        return;
    }
    let Some(display) = state.output_display() else {
        return;
    };

    // A new id per display, so choosing another display builds a new window.
    let id = egui::ViewportId::from_hash_of(("output", &display.device_name, display.rect));
    let builder = if display.is_primary {
        egui::ViewportBuilder::default()
            .with_title("Presenter Flow — Output (test window)")
            .with_inner_size(TEST_WINDOW_SIZE)
            .with_resizable(true)
    } else {
        egui::ViewportBuilder::default()
            .with_title("Presenter Flow — Output")
            .with_decorations(false)
            .with_taskbar(false)
            // Borderless fullscreen on that monitor (index in OS order).
            .with_monitor(display.os_index)
    };

    let frame = state.live.frame();
    ctx.show_viewport_immediate(id, builder, |ui, _class| {
        let rect = ui.max_rect();
        ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK);
        render.paint(ui.painter(), rect, &frame);

        let close = ui.input(|i| {
            if state.ui.editor.is_none() {
                crate::app::shortcut_actions(i, true, actions);
            }
            i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)
        });
        if close {
            actions.push(Action::CloseOutput);
        }
    });
}
