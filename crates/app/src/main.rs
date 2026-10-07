//! Presenter Flow entry point: logging and window setup only (PLAN §3.1).

// Hide the console window in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Skeleton phase: many items are declared ahead of the tasks that use them.
// `expect` (not `allow`) makes the compiler flag this line once everything is
// wired up, so it cannot linger. Remove it no later than T5.2.
#![expect(dead_code, reason = "skeleton: items are wired up during M0-M4")]

mod actions;
mod app;
mod autosave;
mod error;
mod output;
mod platform;
mod render;
mod samples;
mod state;
mod ui;

fn main() -> eframe::Result {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Presenter Flow")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1024.0, 640.0])
            // The operator console uses the whole screen; this also keeps
            // the bottom bar visible on screens smaller than the default size.
            .with_maximized(true),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Presenter Flow",
        options,
        Box::new(|cc| Ok(Box::new(app::PresenterApp::new(cc)))),
    )
}
