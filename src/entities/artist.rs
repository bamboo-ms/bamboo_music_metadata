use super::release_group::ReleaseGroup;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};

// "An artist is generally a musician (or musician persona), group of musicians, or other music
// professional (like a producer or engineer)." [1]

#[derive(Serialize, Deserialize)]
pub struct Artist {
    identifiers: Vec<Identifier>,
    // "The official name of an artist, be it a person or a band" [1]
    name: String,
    // "Aliases are used to store alternate names or misspellings." [1]
    aliases: Vec<String>,
    release_groups: Vec<ReleaseGroup>,
}

// [1] https://wiki.musicbrainz.org/Artist
