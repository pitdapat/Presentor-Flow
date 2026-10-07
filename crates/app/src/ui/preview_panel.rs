//! Selected-slide preview (teal) and live-output preview (red).

use presenter_core::presentation::{LiveContent, LiveState, OutputFrame};

use crate::state::AppState;

use super::{theme, View};

/// Projector aspect ratio of the preview boxes.
const ASPECT: f32 = 16.0 / 9.0;
/// Vertical space taken by the two box headings and the gap between them.
const HEADINGS_HEIGHT: f32 = 60.0;
/// Smallest box height, so the boxes stay readable in a short panel.
const MIN_BOX_HEIGHT: f32 = 60.0;

/// Badge on the live preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LiveBadge {
    Live,
    Blackout,
    Cleared,
}

impl LiveBadge {
    /// Badge for the current live state. Blackout wins over the content
    /// because it is what the audience sees.
    fn for_state(live: &LiveState) -> Option<Self> {
        if live.is_blackout() {
            return Some(Self::Blackout);
        }
        match live.content() {
            LiveContent::Empty => None,
            LiveContent::Slide(_) => Some(Self::Live),
            LiveContent::LyricsCleared { .. } => Some(Self::Cleared),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Live => "LIVE",
            Self::Blackout => "BLACKOUT",
            Self::Cleared => "CLEARED",
        }
    }
}

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, view: &mut View<'_>) {
    // Both boxes must fit the space we are given: size them from whichever
    // of width or height runs out first.
    let box_height = ((ui.available_height() - HEADINGS_HEIGHT) / 2.0).max(MIN_BOX_HEIGHT);
    let box_size = {
        let width = ui.available_width().min(box_height * ASPECT);
        egui::vec2(width, width / ASPECT)
    };

    ui.strong("Preview");
    let selected = state
        .ui
        .selected_song
        .and_then(|id| state.library.song(id))
        .zip(state.ui.selected_slide)
        .and_then(|(song, index)| {
            let slide = song.slides().into_iter().nth(index)?;
            Some(OutputFrame::Slide {
                slide,
                style: song.style,
            })
        });
    slide_box(
        ui,
        view,
        box_size,
        theme::TEAL,
        selected.as_ref(),
        "No slide selected",
    );

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.strong("Live output");
        if let Some(badge) = LiveBadge::for_state(&state.live) {
            badge_label(ui, badge);
        }
        if let LiveContent::Slide(snap) = state.live.content() {
            ui.weak(format!(
                "{} — slide {} of {}",
                snap.song_title,
                snap.slide_index + 1,
                snap.slide_count
            ));
        }
    });
    let live = match state.live.content() {
        LiveContent::Empty if !state.live.is_blackout() => None,
        _ => Some(state.live.frame()),
    };
    slide_box(
        ui,
        view,
        box_size,
        theme::LIVE_RED,
        live.as_ref(),
        "Nothing live",
    );
}

/// A 16:9 box with a colored frame showing `frame`, or `empty_text`.
fn slide_box(
    ui: &mut egui::Ui,
    view: &mut View<'_>,
    size: egui::Vec2,
    stroke: egui::Color32,
    frame: Option<&OutputFrame>,
    empty_text: &str,
) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    match frame {
        Some(frame) => {
            view.render.paint(&painter, rect, frame);
        }
        None => {
            painter.rect_filled(rect, 2.0, theme::INSET);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                empty_text,
                egui::FontId::proportional(14.0),
                theme::TEXT_WEAK,
            );
        }
    }
    painter.rect_stroke(
        rect,
        2.0,
        egui::Stroke::new(theme::FRAME_STROKE, stroke),
        egui::StrokeKind::Inside,
    );
}

fn badge_label(ui: &mut egui::Ui, badge: LiveBadge) {
    egui::Frame::new()
        .fill(theme::LIVE_RED)
        .corner_radius(3)
        .inner_margin(egui::Margin::symmetric(6, 1))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(badge.label())
                    .strong()
                    .small()
                    .color(theme::ON_ACCENT),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_badge_when_nothing_is_live() {
        assert_eq!(LiveBadge::for_state(&LiveState::default()), None);
    }

    #[test]
    fn blackout_badge_wins() {
        let mut live = LiveState::default();
        live.toggle_blackout();
        assert_eq!(LiveBadge::for_state(&live), Some(LiveBadge::Blackout));
    }
}
