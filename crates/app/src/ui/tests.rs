//! UI tests that drive the real operator window headlessly (no GPU, no
//! window) through its accessibility tree, the same way a screen reader would.
//! They cover the T1.4 add / edit flow end to end: button -> dialog -> typed
//! text -> Save -> `apply` -> library.

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;

use crate::actions::{apply, Action};
use crate::render::SlideRenderer;
use crate::state::AppState;

/// Everything one frame of the operator window needs.
struct Fixture {
    app: AppState,
    render: Option<SlideRenderer>,
}

/// Builds a harness that runs one operator-window frame per step, then
/// applies the queued actions, exactly like `app.rs` does.
fn harness() -> Harness<'static, Fixture> {
    let fixture = Fixture {
        app: AppState::default(),
        render: None,
    };
    Harness::builder()
        .with_size(egui::vec2(1600.0, 1000.0))
        .build_ui_state(
            |ui, f: &mut Fixture| {
                // First frame: install the bundled fonts like `app.rs` does at
                // startup. They take effect from the next frame.
                if f.render.is_none() {
                    let installed = crate::render::fonts::install(ui.ctx());
                    assert!(installed.is_ok(), "bundled fonts: {installed:?}");
                    f.render = Some(SlideRenderer::new(ui.ctx().clone()));
                    ui.ctx().request_repaint();
                    return;
                }
                let Some(render) = f.render.as_mut() else {
                    return;
                };
                let mut actions: Vec<Action> = Vec::new();
                let mut view = super::View {
                    actions: &mut actions,
                    render,
                };
                super::draw(ui, &f.app, &mut view);
                for action in actions {
                    // A failed action shows in the status bar in the app;
                    // here it would fail the assertions that follow.
                    let _ = apply(&mut f.app, action);
                }
            },
            fixture,
        )
}

/// Types `text` into the `index`-th text box of the given role.
fn type_into(h: &mut Harness<'_, Fixture>, role: Role, index: usize, text: &str) {
    let nodes: Vec<_> = h.get_all_by_role(role).collect();
    let node = &nodes[index];
    node.focus();
    node.type_text(text);
    h.run();
}

const LYRICS: &str =
    "[Verse 1]\nAmazing grace\nhow sweet the sound\n\n[Chorus]\n奇异恩典\n何等甘甜";

#[test]
fn add_song_through_the_dialog() {
    let mut h = harness();
    h.get_by_label("➕ Add Song").click();
    h.run();
    assert!(h.state().app.ui.editor.is_some(), "dialog opened");

    type_into(&mut h, Role::TextInput, 1, "My Song");
    type_into(&mut h, Role::MultilineTextInput, 0, LYRICS);
    h.get_by_label("Save").click();
    h.run();

    let app = &h.state().app;
    assert!(app.ui.editor.is_none(), "dialog closed after Save");
    let song = &app.library.songs()[0];
    assert_eq!(song.title, "My Song");
    // The raw text is stored exactly as typed (PLAN: lyrics source of truth).
    assert_eq!(song.lyrics_source, LYRICS);
    assert_eq!(song.slides().len(), 2);
    assert_eq!(app.ui.selected_song, Some(song.id), "new song is selected");
}

#[test]
fn edit_song_through_the_dialog_keeps_text_exactly() {
    let mut h = harness();
    let created = apply(
        &mut h.state_mut().app,
        Action::CreateSong {
            title: "Old".into(),
            lyrics: "one\n\ntwo".into(),
        },
    );
    assert!(created.is_ok());
    h.run();

    h.get_by_label("✏ Edit Lyrics").click();
    h.run();
    type_into(&mut h, Role::MultilineTextInput, 0, "\n\nthree");
    h.get_by_label("Save").click();
    h.run();

    let song = &h.state().app.library.songs()[0];
    assert_eq!(song.lyrics_source, "one\n\ntwo\n\nthree");
    assert_eq!(song.slides().len(), 3);
}

#[test]
fn save_is_disabled_without_a_title() {
    let mut h = harness();
    h.get_by_label("➕ Add Song").click();
    h.run();
    type_into(&mut h, Role::MultilineTextInput, 0, "lyrics only");
    h.get_by_label("Save").click();
    h.run();
    assert!(h.state().app.library.songs().is_empty());
    assert!(h.state().app.ui.editor.is_some(), "dialog stays open");
}
