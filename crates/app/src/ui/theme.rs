//! Operator window theme (PLAN §4): dark charcoal, teal for selection, red
//! for anything live. All UI colors come from here; panels never use
//! literal colors.

use egui::{Color32, CornerRadius, Stroke, Theme};

/// Main window background.
pub const CHARCOAL: Color32 = Color32::from_rgb(0x1E, 0x1F, 0x22);
/// Panel background, slightly lighter than the window.
pub const PANEL: Color32 = Color32::from_rgb(0x26, 0x28, 0x2C);
/// Inset areas (text fields, slide placeholders).
pub const INSET: Color32 = Color32::from_rgb(0x15, 0x16, 0x18);
/// Inactive widget fill.
pub const WIDGET: Color32 = Color32::from_rgb(0x33, 0x36, 0x3B);
/// Hovered widget fill.
pub const WIDGET_HOVER: Color32 = Color32::from_rgb(0x3E, 0x42, 0x48);
/// Body text.
pub const TEXT: Color32 = Color32::from_rgb(0xE6, 0xE6, 0xE6);
/// Secondary text.
pub const TEXT_WEAK: Color32 = Color32::from_rgb(0x9A, 0x9D, 0xA3);
/// Selection, preview frame and keyboard focus.
pub const TEAL: Color32 = Color32::from_rgb(0x14, 0xB8, 0xA6);
/// Teal at low opacity, for selected-item backgrounds.
pub const TEAL_DIM: Color32 = Color32::from_rgb(0x12, 0x4D, 0x48);
/// Live frame, live badge and blackout banner.
pub const LIVE_RED: Color32 = Color32::from_rgb(0xE5, 0x3E, 0x3E);
/// Text drawn on top of [`LIVE_RED`] or [`TEAL`].
pub const ON_ACCENT: Color32 = Color32::WHITE;

/// Width of the preview / live frame strokes.
pub const FRAME_STROKE: f32 = 2.0;

/// Padding inside the side and central panels, so content never touches a
/// panel or window edge. Every panel uses the same value so content lines up.
pub const PANEL_MARGIN: egui::Margin = egui::Margin::symmetric(16, 12);
/// Padding inside the top toolbar and bottom status bar.
pub const BAR_MARGIN: egui::Margin = egui::Margin::symmetric(16, 8);
/// Vertical gap between the two halves of a split column.
pub const SECTION_GAP: f32 = 12.0;

/// Frame for side panels and bars: panel background plus `margin`.
pub fn panel_frame(margin: egui::Margin) -> egui::Frame {
    egui::Frame::new().fill(PANEL).inner_margin(margin)
}

/// Frame for the central slide area: slightly darker so the panels around
/// it read as separate columns.
pub fn central_frame() -> egui::Frame {
    egui::Frame::new().fill(CHARCOAL).inner_margin(PANEL_MARGIN)
}

/// Installs the theme on the context. Called once at startup.
pub fn install(ctx: &egui::Context) {
    ctx.set_theme(Theme::Dark);
    ctx.style_mut_of(Theme::Dark, |style| {
        let v = &mut style.visuals;
        v.dark_mode = true;
        v.override_text_color = Some(TEXT);
        v.panel_fill = PANEL;
        v.window_fill = PANEL;
        v.extreme_bg_color = INSET;
        v.faint_bg_color = CHARCOAL;
        v.hyperlink_color = TEAL;
        v.selection.bg_fill = TEAL_DIM;
        v.selection.stroke = Stroke::new(1.0, TEAL);

        let radius = CornerRadius::same(4);
        let w = &mut v.widgets;
        w.noninteractive.bg_fill = PANEL;
        w.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_WEAK);
        w.inactive.bg_fill = WIDGET;
        w.inactive.weak_bg_fill = WIDGET;
        w.inactive.corner_radius = radius;
        w.hovered.bg_fill = WIDGET_HOVER;
        w.hovered.weak_bg_fill = WIDGET_HOVER;
        w.hovered.bg_stroke = Stroke::new(1.0, TEAL);
        w.hovered.corner_radius = radius;
        // Focus must always be visible (PLAN §4): pressed/focused widgets
        // get a teal outline.
        w.active.bg_fill = TEAL_DIM;
        w.active.weak_bg_fill = TEAL_DIM;
        w.active.bg_stroke = Stroke::new(FRAME_STROKE, TEAL);
        w.active.corner_radius = radius;
        w.open.bg_fill = WIDGET_HOVER;
        w.open.corner_radius = radius;

        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    });
}
