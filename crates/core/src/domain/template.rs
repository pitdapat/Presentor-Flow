//! Named style templates.

use super::style::TextStyle;
use crate::ids::TemplateId;

/// A named text style. Applying a template copies its style into a song.
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    /// Unique identifier.
    pub id: TemplateId,
    /// Display name, non-empty after trimming.
    pub name: String,
    /// The style copied into songs.
    pub style: TextStyle,
}
