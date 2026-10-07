//! Windows monitor enumeration via `EnumDisplayMonitors` + `GetMonitorInfoW`
//! (T0.3). Every `unsafe` block here must carry a `// SAFETY:` comment.

use super::DisplayInfo;
use crate::error::AppError;

/// Lists all attached monitors.
pub fn list_displays() -> Result<Vec<DisplayInfo>, AppError> {
    todo!("T0.3: EnumDisplayMonitors")
}
