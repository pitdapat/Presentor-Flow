//! `eframe::App` implementation: draws the UI, collects actions, applies
//! them after the frame, and runs the save and display timers (PLAN §3.3).

use std::time::{Duration, Instant};

use presenter_core::library::Library;
use presenter_core::storage::{JsonRepository, LoadNotice, Repository};

use crate::actions::{apply, Action};
use crate::autosave::Autosave;
use crate::render::{fonts, SlideRenderer};
use crate::state::{AppState, SaveStatus};
use crate::{output, platform, samples, ui};

/// How often the monitor list is refreshed (PLAN §3.7).
const DISPLAY_REFRESH: Duration = Duration::from_secs(2);

/// The operator application.
pub struct PresenterApp {
    state: AppState,
    /// `None` when no data folder could be found; nothing is saved then.
    repo: Option<Box<dyn Repository>>,
    autosave: Autosave,
    render: SlideRenderer,
    last_display_scan: Instant,
}

impl PresenterApp {
    /// Creates the app: theme, fonts, library, settings and displays.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        ui::theme::install(&cc.egui_ctx);
        let mut state = AppState::default();
        if let Err(err) = fonts::install(&cc.egui_ctx) {
            tracing::warn!(%err, "fonts");
            state.ui.error(err);
        }

        let repo: Option<Box<dyn Repository>> =
            JsonRepository::in_app_data().map(|r| Box::new(r) as Box<dyn Repository>);
        let mut autosave = Autosave::default();
        match &repo {
            Some(repo) => load(repo.as_ref(), &mut state, &mut autosave),
            None => state
                .ui
                .error("No data folder found: changes will not be saved."),
        }

        match platform::list_displays() {
            Ok(displays) => state.displays = displays,
            Err(err) => state.ui.error(err.to_string()),
        }

        Self {
            state,
            repo,
            autosave,
            render: SlideRenderer::new(cc.egui_ctx.clone()),
            last_display_scan: Instant::now(),
        }
    }

    /// Saves the library and settings if they are due.
    fn save_if_due(&mut self, force: bool) {
        let now = Instant::now();
        if let Some(kind) = self.state.pending.library.take() {
            self.autosave.mark_dirty(kind, now);
            self.state.ui.save_status = SaveStatus::Unsaved;
        }
        let Some(repo) = &self.repo else {
            return;
        };
        if self.autosave.is_due(now) || (force && self.autosave.is_dirty()) {
            match repo.save_library(&self.state.library) {
                Ok(()) => {
                    self.autosave.saved(true);
                    self.state.ui.save_status = SaveStatus::Saved;
                }
                Err(err) => {
                    tracing::error!(%err, "library save failed");
                    self.autosave.saved(false);
                    self.state.ui.save_status = SaveStatus::Failed(err.to_string());
                }
            }
        }
        if std::mem::take(&mut self.state.pending.settings) {
            if let Err(err) = repo.save_settings(&self.state.settings) {
                self.state
                    .ui
                    .error(format!("Settings were not saved: {err}"));
            }
        }
    }
}

/// Loads library and settings, reporting recovery to the operator and
/// adding sample songs on first launch.
fn load(repo: &dyn Repository, state: &mut AppState, autosave: &mut Autosave) {
    state.settings = repo.load_settings();
    match repo.load_library() {
        Ok((library, notices)) => {
            state.library = library;
            for notice in notices {
                match notice {
                    LoadNotice::Clean => {}
                    LoadNotice::FirstRun => {
                        add_samples(&mut state.library);
                        autosave.mark_dirty(crate::autosave::DirtyKind::Immediate, Instant::now());
                        state
                            .ui
                            .info("Welcome! Three sample songs were added to try things out.");
                    }
                    LoadNotice::RecoveredFromBackup { corrupt_copy } => state.ui.error(format!(
                        "The library file was damaged; the previous save was restored. \
                         The damaged file was kept as {}.",
                        corrupt_copy.display()
                    )),
                    LoadNotice::StartedEmpty { corrupt_copies } => state.ui.error(format!(
                        "The library could not be loaded, so it starts empty. \
                         The damaged files were kept: {}.",
                        corrupt_copies
                            .iter()
                            .map(|p| p.display().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                    LoadNotice::DroppedDanglingEntries(n) => state.ui.info(format!(
                        "{n} playlist entries pointed at missing songs and were removed."
                    )),
                }
            }
        }
        Err(err) => state.ui.error(err.to_string()),
    }
}

fn add_samples(library: &mut Library) {
    for (title, lyrics) in samples::SAMPLE_SONGS {
        if let Err(err) = library.create_song(title, lyrics) {
            tracing::error!(%err, title, "sample song rejected");
        }
    }
}

/// Keyboard shortcuts (PLAN §3.9). Ignored while a text field has focus or
/// a dialog is open.
fn shortcuts(ctx: &egui::Context, state: &AppState, actions: &mut Vec<Action>) {
    if state.ui.editor.is_some() || ctx.egui_wants_keyboard_input() {
        return;
    }
    ctx.input(|i| shortcut_actions(i, ctx.memory(|m| m.focused().is_none()), actions));
}

/// Maps pressed keys to presentation actions. Shared with the output window,
/// which receives the keys when it has focus (e.g. a clicker after the
/// operator clicked the projector screen).
///
/// `space_is_next` is false while a button has keyboard focus: Space then
/// presses that button, and also advancing would do two things at once.
pub fn shortcut_actions(input: &egui::InputState, space_is_next: bool, actions: &mut Vec<Action>) {
    use egui::Key;
    let pressed = |keys: &[Key]| keys.iter().any(|k| input.key_pressed(*k));
    if pressed(&[Key::ArrowRight, Key::ArrowDown, Key::PageDown])
        || (space_is_next && pressed(&[Key::Space]))
    {
        actions.push(Action::NextSlide);
    }
    if pressed(&[Key::ArrowLeft, Key::ArrowUp, Key::PageUp]) {
        actions.push(Action::PreviousSlide);
    }
    if pressed(&[Key::C]) {
        actions.push(Action::ClearLyrics);
    }
    if pressed(&[Key::B]) {
        actions.push(Action::ToggleBlackout);
    }
}

impl eframe::App for PresenterApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = root.ctx().clone();
        let mut actions: Vec<Action> = Vec::new();

        if self.last_display_scan.elapsed() >= DISPLAY_REFRESH {
            self.last_display_scan = Instant::now();
            match platform::list_displays() {
                Ok(displays) if displays != self.state.displays => {
                    actions.push(Action::DisplaysRefreshed(displays));
                }
                Ok(_) => {}
                Err(err) => tracing::warn!(%err, "display scan failed"),
            }
        }

        shortcuts(&ctx, &self.state, &mut actions);
        {
            let mut view = ui::View {
                actions: &mut actions,
                render: &mut self.render,
            };
            ui::draw(root, &self.state, &mut view);
        }
        output::output_viewport::show(&ctx, &self.state, &mut self.render, &mut actions);

        for action in actions {
            if let Err(err) = apply(&mut self.state, action) {
                tracing::warn!(%err, "action failed");
                self.state.ui.error(err.to_string());
            }
        }
        self.save_if_due(false);

        // Keep the timers ticking while idle.
        ctx.request_repaint_after(Duration::from_millis(500));
    }
}

impl Drop for PresenterApp {
    /// Saves anything still pending when the app closes normally.
    fn drop(&mut self) {
        self.save_if_due(true);
    }
}
