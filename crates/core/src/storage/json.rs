//! JSON files in `%APPDATA%\PresenterFlow\` with atomic save, `.bak`
//! backup and corrupt-file recovery (PLAN §3.8).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::migration::migrate;
use super::schema::{self, LibraryFileV1, SettingsFileV1};
use super::{LoadNotice, Repository};
use crate::domain::song::now;
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
/// Temporary settings file written before the atomic rename.
pub const SETTINGS_TEMP: &str = "settings.json.tmp";

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

impl JsonRepository {
    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    /// Reads, migrates and validates one library file.
    fn read_library(path: &Path) -> Result<schema::Converted, CoreError> {
        let corrupt = |message: String| CoreError::Corrupt {
            path: path.to_owned(),
            message,
        };
        let text = fs::read_to_string(path).map_err(|source| CoreError::Io {
            path: path.to_owned(),
            source,
        })?;
        let value: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| corrupt(e.to_string()))?;
        let value = migrate(value)?;
        let file: LibraryFileV1 =
            serde_json::from_value(value).map_err(|e| corrupt(e.to_string()))?;
        schema::Converted::try_from(file).map_err(corrupt)
    }

    /// Moves a file that failed to load to `library.corrupt-<unix>.json`
    /// (with a counter if that name is taken). Never deletes anything.
    fn set_aside(&self, path: &Path) -> Option<PathBuf> {
        let stamp = now();
        let target = (0..100)
            .map(|n| match n {
                0 => self.path(&format!("library.corrupt-{stamp}.json")),
                n => self.path(&format!("library.corrupt-{stamp}-{n}.json")),
            })
            .find(|p| !p.exists())?;
        match fs::rename(path, &target) {
            Ok(()) => Some(target),
            Err(err) => {
                tracing::error!(%err, ?path, "could not set aside a damaged file");
                None
            }
        }
    }

    /// Writes `bytes` to the temp file `temp` and syncs it to disk.
    fn write_synced(&self, temp: &str, bytes: &[u8]) -> Result<PathBuf, CoreError> {
        let io = |path: PathBuf| move |source| CoreError::Io { path, source };
        fs::create_dir_all(&self.dir).map_err(io(self.dir.clone()))?;
        let tmp = self.path(temp);
        let mut file = fs::File::create(&tmp).map_err(io(tmp.clone()))?;
        file.write_all(bytes).map_err(io(tmp.clone()))?;
        file.sync_all().map_err(io(tmp.clone()))?;
        Ok(tmp)
    }
}

/// Renames `from` to `to`, replacing `to` (atomic on Windows and Unix).
fn replace(from: &Path, to: &Path) -> Result<(), CoreError> {
    fs::rename(from, to).map_err(|source| CoreError::Io {
        path: to.to_owned(),
        source,
    })
}

impl Repository for JsonRepository {
    fn load_library(&self) -> Result<(Library, Vec<LoadNotice>), CoreError> {
        let main = self.path(LIBRARY_FILE);
        let backup = self.path(LIBRARY_BACKUP);
        if !main.exists() && !backup.exists() {
            return Ok((Library::default(), vec![LoadNotice::FirstRun]));
        }

        let mut notices = Vec::new();
        let mut corrupt_copies = Vec::new();

        for (path, is_backup) in [(&main, false), (&backup, true)] {
            if !path.exists() {
                continue;
            }
            match Self::read_library(path) {
                Ok(converted) => {
                    if is_backup {
                        if let Some(copy) = corrupt_copies.first() {
                            notices.push(LoadNotice::RecoveredFromBackup {
                                corrupt_copy: PathBuf::clone(copy),
                            });
                        }
                    }
                    if converted.dropped_entries > 0 {
                        notices.push(LoadNotice::DroppedDanglingEntries(
                            converted.dropped_entries,
                        ));
                    }
                    if notices.is_empty() {
                        notices.push(LoadNotice::Clean);
                    }
                    return Ok((converted.library, notices));
                }
                // A newer schema is not damage: refuse to touch the files.
                Err(err @ CoreError::UnsupportedSchema(_)) => return Err(err),
                Err(err) => {
                    tracing::warn!(%err, ?path, "library file failed to load");
                    // Set it aside so no later save can overwrite it.
                    match self.set_aside(path) {
                        Some(copy) => corrupt_copies.push(copy),
                        None => corrupt_copies.push(path.clone()),
                    }
                }
            }
        }

        Ok((
            Library::default(),
            vec![LoadNotice::StartedEmpty { corrupt_copies }],
        ))
    }

    fn save_library(&self, library: &Library) -> Result<(), CoreError> {
        let file = LibraryFileV1::from(library);
        let bytes = serde_json::to_vec_pretty(&file).map_err(|e| CoreError::Corrupt {
            path: self.path(LIBRARY_FILE),
            message: e.to_string(),
        })?;

        let tmp = self.write_synced(LIBRARY_TEMP, &bytes)?;
        let main = self.path(LIBRARY_FILE);
        if main.exists() {
            replace(&main, &self.path(LIBRARY_BACKUP))?;
        }
        replace(&tmp, &main)
    }

    fn load_settings(&self) -> Settings {
        let path = self.path(SETTINGS_FILE);
        let Ok(text) = fs::read_to_string(&path) else {
            return Settings::default();
        };
        match serde_json::from_str::<SettingsFileV1>(&text) {
            Ok(file) => file.into(),
            Err(err) => {
                tracing::warn!(%err, "settings.json is damaged; using defaults");
                Settings::default()
            }
        }
    }

    fn save_settings(&self, settings: &Settings) -> Result<(), CoreError> {
        let bytes = serde_json::to_vec_pretty(&SettingsFileV1::from(settings)).map_err(|e| {
            CoreError::Corrupt {
                path: self.path(SETTINGS_FILE),
                message: e.to_string(),
            }
        })?;
        let tmp = self.write_synced(SETTINGS_TEMP, &bytes)?;
        replace(&tmp, &self.path(SETTINGS_FILE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::TextStyle;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn sample() -> Result<Library, CoreError> {
        let mut lib = Library::default();
        let a = lib.create_song("Amazing", "[Verse]\na\nb\n\nc")?;
        lib.create_song("中文", "一二三")?;
        lib.set_song_style(
            a,
            TextStyle {
                size: 60.0,
                ..TextStyle::default()
            },
        )?;
        Ok(lib)
    }

    fn same(a: &Library, b: &Library) -> bool {
        a.songs() == b.songs() && a.playlists() == b.playlists() && a.templates() == b.templates()
    }

    #[test]
    fn first_run_is_empty_and_reported() -> TestResult {
        let dir = tempfile::tempdir()?;
        let (lib, notices) = JsonRepository::new(dir.path()).load_library()?;
        assert!(lib.songs().is_empty());
        assert_eq!(notices, vec![LoadNotice::FirstRun]);
        Ok(())
    }

    #[test]
    fn round_trip_keeps_everything() -> TestResult {
        let dir = tempfile::tempdir()?;
        let repo = JsonRepository::new(dir.path());
        let lib = sample()?;
        repo.save_library(&lib)?;
        let (loaded, notices) = repo.load_library()?;
        assert!(same(&lib, &loaded));
        assert_eq!(notices, vec![LoadNotice::Clean]);
        assert!(!dir.path().join(LIBRARY_TEMP).exists());
        Ok(())
    }

    #[test]
    fn second_save_keeps_a_backup_of_the_first() -> TestResult {
        let dir = tempfile::tempdir()?;
        let repo = JsonRepository::new(dir.path());
        let mut lib = sample()?;
        repo.save_library(&lib)?;
        let first = fs::read_to_string(dir.path().join(LIBRARY_FILE))?;
        lib.create_song("Third", "x")?;
        repo.save_library(&lib)?;
        assert_eq!(fs::read_to_string(dir.path().join(LIBRARY_BACKUP))?, first);
        Ok(())
    }

    #[test]
    fn corrupt_main_file_recovers_from_backup_and_is_kept() -> TestResult {
        let dir = tempfile::tempdir()?;
        let repo = JsonRepository::new(dir.path());
        let lib = sample()?;
        repo.save_library(&lib)?;
        repo.save_library(&lib)?; // now library.json.bak exists too
        fs::write(dir.path().join(LIBRARY_FILE), "{ not json")?;

        let (loaded, notices) = repo.load_library()?;
        assert!(same(&lib, &loaded));
        let [LoadNotice::RecoveredFromBackup { corrupt_copy }] = notices.as_slice() else {
            return Err(format!("unexpected notices {notices:?}").into());
        };
        assert_eq!(fs::read_to_string(corrupt_copy)?, "{ not json");
        assert!(!dir.path().join(LIBRARY_FILE).exists());
        Ok(())
    }

    #[test]
    fn both_files_corrupt_starts_empty_and_deletes_nothing() -> TestResult {
        let dir = tempfile::tempdir()?;
        let repo = JsonRepository::new(dir.path());
        fs::write(dir.path().join(LIBRARY_FILE), "garbage")?;
        fs::write(dir.path().join(LIBRARY_BACKUP), "also garbage")?;

        let (loaded, notices) = repo.load_library()?;
        assert!(loaded.songs().is_empty());
        let [LoadNotice::StartedEmpty { corrupt_copies }] = notices.as_slice() else {
            return Err(format!("unexpected notices {notices:?}").into());
        };
        assert_eq!(corrupt_copies.len(), 2);
        for copy in corrupt_copies {
            assert!(copy.exists(), "{copy:?} was deleted");
        }
        Ok(())
    }

    #[test]
    fn newer_schema_is_refused_without_touching_files() -> TestResult {
        let dir = tempfile::tempdir()?;
        let text = r#"{"schema_version": 99, "songs": [], "playlists": [], "templates": []}"#;
        fs::write(dir.path().join(LIBRARY_FILE), text)?;
        let result = JsonRepository::new(dir.path()).load_library();
        assert!(matches!(result, Err(CoreError::UnsupportedSchema(99))));
        assert_eq!(fs::read_to_string(dir.path().join(LIBRARY_FILE))?, text);
        Ok(())
    }

    #[test]
    fn dangling_playlist_entries_are_dropped_and_reported() -> TestResult {
        let dir = tempfile::tempdir()?;
        let text = format!(
            r#"{{"schema_version":1,"songs":[],"templates":[],
               "playlists":[{{"id":"{}","name":"P","entries":[{{"id":"{}","song":"{}"}}]}}]}}"#,
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4()
        );
        fs::write(dir.path().join(LIBRARY_FILE), text)?;
        let (lib, notices) = JsonRepository::new(dir.path()).load_library()?;
        assert!(lib.playlists()[0].entries.is_empty());
        assert_eq!(notices, vec![LoadNotice::DroppedDanglingEntries(1)]);
        Ok(())
    }

    #[test]
    fn failed_save_leaves_the_previous_file_intact() -> TestResult {
        let dir = tempfile::tempdir()?;
        let repo = JsonRepository::new(dir.path());
        let lib = sample()?;
        repo.save_library(&lib)?;
        let before = fs::read_to_string(dir.path().join(LIBRARY_FILE))?;
        // A directory where the temp file should go makes the write fail.
        fs::create_dir(dir.path().join(LIBRARY_TEMP))?;
        assert!(repo.save_library(&lib).is_err());
        assert_eq!(fs::read_to_string(dir.path().join(LIBRARY_FILE))?, before);
        Ok(())
    }

    #[test]
    fn settings_round_trip_and_default_on_damage() -> TestResult {
        let dir = tempfile::tempdir()?;
        let repo = JsonRepository::new(dir.path());
        assert_eq!(repo.load_settings(), Settings::default());
        let settings = Settings {
            output_display: Some(crate::settings::DisplayRef {
                device_name: r"\\.\DISPLAY2".into(),
                rect: [1920, 0, 3840, 1080],
            }),
            last_playlist: None,
        };
        repo.save_settings(&settings)?;
        assert_eq!(repo.load_settings(), settings);
        fs::write(dir.path().join(SETTINGS_FILE), "{{{")?;
        assert_eq!(repo.load_settings(), Settings::default());
        Ok(())
    }
}
