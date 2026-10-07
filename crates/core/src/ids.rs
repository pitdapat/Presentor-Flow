//! Strongly typed identifiers. Each is a newtype over a v4 UUID so IDs of
//! different kinds cannot be mixed up.

use uuid::Uuid;

macro_rules! id_type {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Uuid);

        impl $name {
            /// Creates a new random identifier.
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            /// Wraps an existing UUID (used when loading from storage).
            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Returns the underlying UUID (used when saving to storage).
            pub fn as_uuid(&self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

id_type!(
    /// Identifies a song in the library.
    SongId
);
id_type!(
    /// Identifies a playlist (service).
    PlaylistId
);
id_type!(
    /// Identifies one entry in a playlist. The same song may appear in
    /// several entries.
    EntryId
);
id_type!(
    /// Identifies a style template.
    TemplateId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ids_are_unique() {
        assert_ne!(SongId::new(), SongId::new());
    }

    #[test]
    fn uuid_round_trip() {
        let id = PlaylistId::new();
        assert_eq!(PlaylistId::from_uuid(id.as_uuid()), id);
    }
}
