//! `eframe::App` implementation: draws the UI, collects actions, applies
//! them after the frame (PLAN §3.3).

use crate::actions::{apply, Action};
use crate::state::AppState;
use crate::ui;

/// The operator application.
pub struct PresenterApp {
    state: AppState,
}

impl PresenterApp {
    /// Creates the app. Fonts, theme and storage loading are added in
    /// T0.2, T0.5 and T1.3.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for PresenterApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut actions: Vec<Action> = Vec::new();

        ui::draw(root, &self.state, &mut actions);

        for action in actions {
            if let Err(err) = apply(&mut self.state, action) {
                tracing::warn!(%err, "action failed");
                self.state.ui.status_message = Some(err.to_string());
            }
        }
    }
}
