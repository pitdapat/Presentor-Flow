//! OS-specific code. The only place `unsafe` is allowed (PLAN §3.2).

#[cfg(windows)]
pub mod windows;

/// One attached monitor.
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayInfo {
    /// OS device name, for example `\\.\DISPLAY2`.
    pub device_name: String,
    /// Human-readable name for the selector.
    pub friendly_name: String,
    /// Monitor rectangle in physical pixels: left, top, right, bottom.
    pub rect: [i32; 4],
    /// Whether this is the primary (operator) display.
    pub is_primary: bool,
    /// DPI scale factor (1.0 = 100 %).
    pub scale_factor: f32,
}
