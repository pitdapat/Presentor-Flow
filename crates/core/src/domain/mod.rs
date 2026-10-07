//! Domain types and pure domain functions. Nothing here touches files,
//! fonts or the UI.

pub mod lyrics;
pub mod playlist;
pub mod song;
pub mod style;
pub mod template;

pub use lyrics::{parse_lyrics, ParsedLyrics, Slide};
pub use playlist::{MoveDir, Playlist, PlaylistEntry};
pub use song::{Song, Timestamp};
pub use style::{FontFamily, HAlign, Rgba, TextStyle, VAlign};
pub use template::Template;
