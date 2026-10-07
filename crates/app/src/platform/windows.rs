//! Windows monitor enumeration via `EnumDisplayMonitors` + `GetMonitorInfoW`
//! (T0.3). Every `unsafe` block here must carry a `// SAFETY:` comment.

use windows::core::BOOL;
use windows::Win32::Foundation::{LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW,
};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};

use super::DisplayInfo;
use crate::error::AppError;

/// `MONITORINFOF_PRIMARY` from WinUser.h.
const MONITORINFOF_PRIMARY: u32 = 1;
/// The DPI that Windows treats as 100 % scaling.
const BASE_DPI: f32 = 96.0;

/// Lists all attached monitors, primary first, then left to right.
pub fn list_displays() -> Result<Vec<DisplayInfo>, AppError> {
    let mut handles: Vec<HMONITOR> = Vec::new();

    // SAFETY: `collect` only runs during this call, and `dwdata` points at
    // `handles`, which outlives the call and is not otherwise borrowed.
    let ok = unsafe {
        EnumDisplayMonitors(
            None,
            None,
            Some(collect),
            LPARAM(&mut handles as *mut Vec<HMONITOR> as isize),
        )
    };
    if !ok.as_bool() {
        return Err(AppError::Platform("could not list the displays".into()));
    }

    let mut displays: Vec<DisplayInfo> = handles
        .into_iter()
        .enumerate()
        .filter_map(|(os_index, monitor)| describe(monitor, os_index))
        .collect();
    displays.sort_by_key(|d| (!d.is_primary, d.rect[0], d.rect[1]));
    for (i, d) in displays.iter_mut().enumerate() {
        d.friendly_name = super::friendly_name(i + 1, d.rect, d.is_primary);
    }
    Ok(displays)
}

/// `EnumDisplayMonitors` callback: appends each monitor handle.
unsafe extern "system" fn collect(
    monitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    data: LPARAM,
) -> BOOL {
    // SAFETY: `data` is the `&mut Vec<HMONITOR>` passed by `list_displays`,
    // valid and exclusively ours for the duration of the enumeration.
    let handles = unsafe { &mut *(data.0 as *mut Vec<HMONITOR>) };
    handles.push(monitor);
    BOOL::from(true)
}

/// Reads one monitor's name, rectangle, primary flag and scale.
fn describe(monitor: HMONITOR, os_index: usize) -> Option<DisplayInfo> {
    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = u32::try_from(std::mem::size_of::<MONITORINFOEXW>()).ok()?;

    // SAFETY: `info` is a properly sized MONITORINFOEXW with `cbSize` set,
    // which `GetMonitorInfoW` accepts through a `MONITORINFO` pointer.
    let ok = unsafe { GetMonitorInfoW(monitor, &mut info as *mut _ as *mut MONITORINFO) };
    if !ok.as_bool() {
        return None;
    }

    let (mut dpi_x, mut dpi_y) = (0u32, 0u32);
    // SAFETY: both out-pointers refer to live local variables.
    let scale_factor =
        match unsafe { GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) } {
            Ok(()) if dpi_x > 0 => dpi_x as f32 / BASE_DPI,
            _ => 1.0,
        };

    let device = &info.szDevice;
    let len = device.iter().position(|&c| c == 0).unwrap_or(device.len());
    let device_name = String::from_utf16_lossy(&device[..len]);

    let r = info.monitorInfo.rcMonitor;
    let rect = [r.left, r.top, r.right, r.bottom];
    let is_primary = info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY != 0;

    Some(DisplayInfo {
        // Numbered after sorting, in `list_displays`.
        friendly_name: String::new(),
        device_name,
        rect,
        is_primary,
        scale_factor,
        os_index,
    })
}
