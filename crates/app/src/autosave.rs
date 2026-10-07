//! Save scheduling (PLAN §3.8): immediate for finished operations,
//! debounced (1 s) for typing and style edits.

use std::time::{Duration, Instant};

/// Debounce delay for continuous edits.
pub const DEBOUNCE: Duration = Duration::from_secs(1);

/// How urgently a change must be saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyKind {
    /// Finished operation (create, delete, reorder): save now.
    Immediate,
    /// Continuous edit (typing, sliders): save after [`DEBOUNCE`].
    Debounced,
}

/// Tracks whether the library needs saving and when.
#[derive(Debug, Default)]
pub struct Autosave {
    due: Option<Instant>,
}

impl Autosave {
    /// Records a change.
    pub fn mark_dirty(&mut self, kind: DirtyKind, now: Instant) {
        let _ = (kind, now);
        todo!("T1.4: schedule save")
    }

    /// True when a save should happen now.
    pub fn is_due(&self, now: Instant) -> bool {
        self.due.is_some_and(|due| now >= due)
    }
}
