//! Song title, Edit Lyrics, and the slide thumbnail grid.
//!
//! Click a thumbnail to send it live. Right-click or Alt+click only selects
//! it for the preview (PLAN §4).

use presenter_core::domain::Song;
use presenter_core::presentation::{LiveContent, OutputFrame};

use crate::actions::Action;
use crate::state::AppState;

use super::{theme, View};

/// Thumbnail width in points.
const THUMB_WIDTH: f32 = 220.0;

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, view: &mut View<'_>) {
    let Some(song) = state.ui.selected_song.and_then(|id| state.library.song(id)) else {
        ui.heading("No song selected");
        ui.separator();
        ui.weak("Select a song in the library to see its slides.");
        return;
    };

    ui.horizontal(|ui| {
        ui.heading(&song.title);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("✏ Edit Lyrics").clicked() {
                view.push(Action::OpenEditor(Some(song.id)));
            }
        });
    });
    ui.weak("Click a slide to present it · right-click or Alt+click to preview only");
    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| grid(ui, state, song, view));
}

fn grid(ui: &mut egui::Ui, state: &AppState, song: &Song, view: &mut View<'_>) {
    let live_index = match state.live.content() {
        LiveContent::Slide(s) if s.song == song.id => Some(s.slide_index),
        _ => None,
    };

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
        for (index, slide) in song.slides().into_iter().enumerate() {
            let label = match &slide.section {
                Some(section) => format!("{} · {section}", index + 1),
                None => format!("{}", index + 1),
            };
            let frame = OutputFrame::Slide {
                slide,
                style: song.style,
            };
            let border = if live_index == Some(index) {
                Some(theme::LIVE_RED)
            } else if state.ui.selected_slide == Some(index) {
                Some(theme::TEAL)
            } else {
                None
            };

            ui.vertical(|ui| {
                ui.set_width(THUMB_WIDTH);
                let size = egui::vec2(THUMB_WIDTH, THUMB_WIDTH * 9.0 / 16.0);
                let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
                let a11y_label = format!("Slide {label}: {}", first_line(&frame));
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &a11y_label)
                });
                let overflow = view.render.paint(ui.painter(), rect, &frame);

                let stroke = match border {
                    Some(c) => egui::Stroke::new(theme::FRAME_STROKE + 1.0, c),
                    None if response.hovered() => egui::Stroke::new(1.0, theme::TEAL),
                    None => egui::Stroke::new(1.0, theme::WIDGET_HOVER),
                };
                ui.painter()
                    .rect_stroke(rect, 2.0, stroke, egui::StrokeKind::Outside);

                ui.horizontal(|ui| {
                    ui.label(label);
                    if live_index == Some(index) {
                        ui.colored_label(theme::LIVE_RED, "LIVE");
                    }
                    if overflow {
                        ui.colored_label(theme::LIVE_RED, "⚠ too long")
                            .on_hover_text(
                                "This slide's text does not fit. Split it with a blank line.",
                            );
                    }
                });

                let preview_only = response.secondary_clicked()
                    || (response.clicked() && ui.input(|i| i.modifiers.alt));
                if response.clicked() || response.secondary_clicked() {
                    // Mouse targets: give focus back so Space and the arrow
                    // keys keep meaning Next/Previous (PLAN §3.9).
                    response.surrender_focus();
                }
                if preview_only {
                    view.push(Action::SelectSlide(index));
                } else if response.clicked() {
                    view.push(Action::GoLive {
                        song: song.id,
                        slide: index,
                    });
                }
            });
        }
    });
}

/// First line of a slide, for its accessible name.
fn first_line(frame: &OutputFrame) -> &str {
    match frame {
        OutputFrame::Slide { slide, .. } => slide.text.lines().next().unwrap_or_default(),
        _ => "",
    }
}
