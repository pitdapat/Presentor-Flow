//! App-level errors. Each has a user-readable message for the status bar.

use presenter_core::CoreError;

/// Errors raised while applying actions or driving the output window.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// An error from the core crate.
    #[error(transparent)]
    Core(#[from] CoreError),

    /// A platform call (for example monitor enumeration) failed.
    #[error("Display error: {0}")]
    Platform(String),

    /// Skeleton placeholder for actions not implemented yet. Removed by M4.
    #[error("Not implemented yet: {0}")]
    NotImplemented(String),
}
