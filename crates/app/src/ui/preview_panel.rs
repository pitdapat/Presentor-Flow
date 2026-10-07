//! Selected-slide preview (teal) and live-output preview (red).

use presenter_core::presentation::{LiveContent, LiveState};

use crate::actions::Action;
use crate::state::AppState;

use super::theme;

/// Projector aspect ratio used for the preview boxes until a display is
/// chosen (T0.3).
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

pub(super) fn show(ui: &mut egui::Ui, state: &AppState, _actions: &mut Vec<Action>) {
    // Both boxes must fit the space we are given: size them from whichever
    // of width or height runs out first.
    let box_height = ((ui.available_height() - HEADINGS_HEIGHT) / 2.0).max(MIN_BOX_HEIGHT);
    let box_size = {
        let width = ui.available_width().min(box_height * ASPECT);
        egui::vec2(width, width / ASPECT)
    };

    ui.strong("Preview");
    let preview_text = match state.ui.selected_slide {
        Some(_) => "Slide preview — drawn in T2.2",
        None => "No slide selected",
    };
    framed_box(ui, box_size, theme::TEAL, preview_text);

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.strong("Live output");
        if let Some(badge) = LiveBadge::for_state(&state.live) {
            badge_label(ui, badge);
        }
    });
    let live_text = match state.live.content() {
        LiveContent::Empty => "Nothing live",
        LiveContent::Slide(_) => "Live slide — drawn in T2.2",
        LiveContent::LyricsCleared { .. } => "Lyrics cleared",
    };
    framed_box(ui, box_size, theme::LIVE_RED, live_text);
}

/// A 16:9 box with a colored frame and centered placeholder text.
fn framed_box(ui: &mut egui::Ui, size: egui::Vec2, stroke: egui::Color32, text: &str) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 2.0, theme::INSET);
    painter.rect_stroke(
        rect,
        2.0,
        egui::Stroke::new(theme::FRAME_STROKE, stroke),
        egui::StrokeKind::Inside,
    );
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(14.0),
        theme::TEXT_WEAK,
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
