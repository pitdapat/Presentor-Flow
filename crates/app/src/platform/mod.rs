//! OS-specific code. The only place `unsafe` is allowed (PLAN §3.2).

#[cfg(windows)]
#[allow(unsafe_code, reason = "Win32 FFI boundary (PLAN §3.2)")]
pub mod windows;

use crate::error::AppError;

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
    /// Position in the OS enumeration order. winit enumerates monitors with
    /// the same `EnumDisplayMonitors` call, so this is the index egui's
    /// `ViewportBuilder::with_monitor` expects.
    pub os_index: usize,
}

impl DisplayInfo {
    /// Width and height in physical pixels.
    pub fn size_px(&self) -> (i32, i32) {
        (self.rect[2] - self.rect[0], self.rect[3] - self.rect[1])
    }

    /// The stored reference used to find this display again next launch.
    pub fn to_ref(&self) -> presenter_core::settings::DisplayRef {
        presenter_core::settings::DisplayRef {
            device_name: self.device_name.clone(),
            rect: self.rect,
        }
    }

    /// Whether this display is the one a saved reference points at. Both the
    /// device name and the rectangle must match (PLAN §3.7).
    pub fn matches(&self, r: &presenter_core::settings::DisplayRef) -> bool {
        self.device_name == r.device_name && self.rect == r.rect
    }
}

/// Label for the selector, for example `Display 2 — 1920×1080`. `number` is
/// the display's position in the sorted list (1 = primary), because Windows
/// device numbers (such as `DISPLAY129`) are not meaningful to people.
pub fn friendly_name(number: usize, rect: [i32; 4], is_primary: bool) -> String {
    let size = format!("{}×{}", rect[2] - rect[0], rect[3] - rect[1]);
    if is_primary {
        format!("Display {number} — {size} (this screen)")
    } else {
        format!("Display {number} — {size}")
    }
}

/// Lists attached monitors. Returns an empty list on other platforms.
pub fn list_displays() -> Result<Vec<DisplayInfo>, AppError> {
    #[cfg(windows)]
    {
        windows::list_displays()
    }
    #[cfg(not(windows))]
    {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_names() {
        assert_eq!(
            friendly_name(2, [1920, 0, 3840, 1080], false),
            "Display 2 — 1920×1080"
        );
        assert_eq!(
            friendly_name(1, [0, 0, 2560, 1600], true),
            "Display 1 — 2560×1600 (this screen)"
        );
    }

    #[test]
    fn list_displays_finds_at_least_one_monitor() -> Result<(), AppError> {
        // A Windows machine (including CI) always has a primary display.
        if cfg!(windows) {
            let displays = list_displays()?;
            assert!(!displays.is_empty());
            assert!(displays[0].is_primary);
            assert!(displays
                .iter()
                .all(|d| d.size_px().0 > 0 && d.size_px().1 > 0));
        }
        Ok(())
    }
}
