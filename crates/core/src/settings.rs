//! User settings, stored separately from the library (PLAN §3.8).

use crate::ids::PlaylistId;

/// Identifies the chosen output display so it can be matched next launch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayRef {
    /// OS device name, for example `\\.\DISPLAY2`.
    pub device_name: String,
    /// Monitor rectangle in physical pixels: left, top, right, bottom.
    pub rect: [i32; 4],
}

/// Application settings.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Settings {
    /// Last chosen output display, if any.
    pub output_display: Option<DisplayRef>,
    /// Playlist that was open when the app last closed.
    pub last_playlist: Option<PlaylistId>,
}
