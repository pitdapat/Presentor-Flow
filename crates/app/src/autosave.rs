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
    /// Records a change. An immediate save is never postponed by a later
    /// debounced one.
    pub fn mark_dirty(&mut self, kind: DirtyKind, now: Instant) {
        let due = match kind {
            DirtyKind::Immediate => now,
            DirtyKind::Debounced => now + DEBOUNCE,
        };
        self.due = Some(match self.due {
            Some(existing) if kind == DirtyKind::Debounced && existing <= now => existing,
            _ => due,
        });
    }

    /// True when a save should happen now.
    pub fn is_due(&self, now: Instant) -> bool {
        self.due.is_some_and(|due| now >= due)
    }

    /// True while a change is waiting to be saved.
    pub fn is_dirty(&self) -> bool {
        self.due.is_some()
    }

    /// Call after a save attempt. A failed save stays dirty and is retried
    /// on the next change (PLAN §3.8), not in a tight loop.
    pub fn saved(&mut self, ok: bool) {
        if ok {
            self.due = None;
        } else {
            self.due = Some(Instant::now() + Duration::from_secs(3600));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_is_due_now_and_debounced_waits() {
        let t = Instant::now();
        let mut a = Autosave::default();
        assert!(!a.is_dirty());
        a.mark_dirty(DirtyKind::Debounced, t);
        assert!(!a.is_due(t));
        assert!(a.is_due(t + DEBOUNCE));
        a.mark_dirty(DirtyKind::Immediate, t);
        assert!(a.is_due(t));
        a.saved(true);
        assert!(!a.is_dirty());
    }

    #[test]
    fn debounced_does_not_postpone_a_due_immediate_save() {
        let t = Instant::now();
        let mut a = Autosave::default();
        a.mark_dirty(DirtyKind::Immediate, t);
        a.mark_dirty(DirtyKind::Debounced, t);
        assert!(a.is_due(t));
    }

    #[test]
    fn failed_save_stays_dirty_until_the_next_change() {
        let t = Instant::now();
        let mut a = Autosave::default();
        a.mark_dirty(DirtyKind::Immediate, t);
        a.saved(false);
        assert!(a.is_dirty());
        assert!(!a.is_due(Instant::now()));
        a.mark_dirty(DirtyKind::Immediate, Instant::now());
        assert!(a.is_due(Instant::now()));
    }
}
