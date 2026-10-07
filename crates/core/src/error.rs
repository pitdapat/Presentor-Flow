//! Error type for all fallible core operations. Every variant has a
//! user-readable message, because errors end up in the status bar.

use crate::ids::{PlaylistId, SongId, TemplateId};

/// Errors returned by library, storage and validation code.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    /// A song title was empty after trimming.
    #[error("The song title cannot be empty.")]
    EmptyTitle,

    /// The lyrics produced no slides.
    #[error("The lyrics contain no slides.")]
    NoSlides,

    /// A playlist name was empty after trimming.
    #[error("The playlist name cannot be empty.")]
    EmptyPlaylistName,

    /// A template name was empty after trimming.
    #[error("The template name cannot be empty.")]
    EmptyTemplateName,

    /// The referenced song does not exist.
    #[error("That song no longer exists.")]
    SongNotFound(SongId),

    /// The referenced playlist does not exist.
    #[error("That playlist no longer exists.")]
    PlaylistNotFound(PlaylistId),

    /// The referenced template does not exist.
    #[error("That template no longer exists.")]
    TemplateNotFound(TemplateId),

    /// A style value was outside its allowed range.
    #[error("{field} must be between {min} and {max}.")]
    StyleOutOfRange {
        /// Name of the style field shown to the user.
        field: &'static str,
        /// Lowest allowed value.
        min: f32,
        /// Highest allowed value.
        max: f32,
    },

    /// Reading or writing a file failed.
    #[error("Could not access {path}: {source}")]
    Io {
        /// File that was being accessed.
        path: std::path::PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A saved file could not be parsed.
    #[error("The file {path} is damaged: {message}")]
    Corrupt {
        /// File that failed to load.
        path: std::path::PathBuf,
        /// Parser message.
        message: String,
    },

    /// A saved file uses a schema version this build does not know.
    #[error("The library was saved by a newer version (schema {0}).")]
    UnsupportedSchema(u32),
}
