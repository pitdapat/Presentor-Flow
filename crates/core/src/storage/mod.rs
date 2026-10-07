//! Persistence. The [`Repository`] trait is what the app depends on; the
//! JSON implementation and the on-disk schema live in submodules.

pub mod json;
pub mod migration;
pub mod schema;

pub use json::JsonRepository;

use crate::library::Library;
use crate::settings::Settings;
use crate::CoreError;

/// What happened while loading, so the UI can tell the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadNotice {
    /// Loaded normally.
    Clean,
    /// The main file was damaged and set aside; the backup was used.
    RecoveredFromBackup {
        /// Where the damaged file was moved.
        corrupt_copy: std::path::PathBuf,
    },
    /// Neither file could be loaded; started with an empty library.
    StartedEmpty {
        /// Damaged files that were set aside.
        corrupt_copies: Vec<std::path::PathBuf>,
    },
    /// Playlist entries pointing at missing songs were dropped.
    DroppedDanglingEntries(usize),
}

/// Loads and saves the library and settings.
pub trait Repository {
    /// Loads the library, recovering from damage where possible.
    fn load_library(&self) -> Result<(Library, Vec<LoadNotice>), CoreError>;

    /// Saves the library atomically, keeping a backup of the previous save.
    fn save_library(&self, library: &Library) -> Result<(), CoreError>;

    /// Loads settings, falling back to defaults if missing or damaged.
    fn load_settings(&self) -> Settings;

    /// Saves settings.
    fn save_settings(&self, settings: &Settings) -> Result<(), CoreError>;
}
