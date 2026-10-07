//! JSON files in `%APPDATA%\PresenterFlow\` with atomic save, `.bak`
//! backup and corrupt-file recovery (PLAN §3.8).

use std::path::{Path, PathBuf};

use super::{LoadNotice, Repository};
use crate::library::Library;
use crate::settings::Settings;
use crate::CoreError;

/// Library file name.
pub const LIBRARY_FILE: &str = "library.json";
/// Backup of the previous good save.
pub const LIBRARY_BACKUP: &str = "library.json.bak";
/// Temporary file written before the atomic rename.
pub const LIBRARY_TEMP: &str = "library.json.tmp";
/// Settings file name.
pub const SETTINGS_FILE: &str = "settings.json";

/// File-based repository rooted at a data directory.
#[derive(Debug, Clone)]
pub struct JsonRepository {
    dir: PathBuf,
}

impl JsonRepository {
    /// Uses an explicit directory (tests use a temp dir).
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// Uses `%APPDATA%\PresenterFlow\`. Returns `None` if the OS reports no
    /// home directory.
    pub fn in_app_data() -> Option<Self> {
        directories::ProjectDirs::from("", "", "PresenterFlow").map(|d| Self::new(d.data_dir()))
    }

    /// The data directory.
    pub fn dir(&self) -> &Path {
        &self.dir
    }
}

impl Repository for JsonRepository {
    fn load_library(&self) -> Result<(Library, Vec<LoadNotice>), CoreError> {
        todo!("T1.3: load + migrate + validate + recover from .bak")
    }

    fn save_library(&self, library: &Library) -> Result<(), CoreError> {
        let _ = library;
        todo!("T1.3: write .tmp, sync_all, rotate .bak, rename")
    }

    fn load_settings(&self) -> Settings {
        todo!("T1.3: load settings.json, default on error")
    }

    fn save_settings(&self, settings: &Settings) -> Result<(), CoreError> {
        let _ = settings;
        todo!("T1.3: save settings.json")
    }
}
