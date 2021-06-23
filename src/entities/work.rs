use super::{artist::Artist, track::Track};
use crate::identifier::Identifier;

// "...a work is a distinct intellectual or artistic creation, which can be expressed in the form
// of one or more audio recordings." [1]

pub struct Work {
    identifiers: Vec<Identifier>,
    name: String,
    artists: Vec<Artist>,
    tracks: Vec<Track>,
}

// [1] https://wiki.musicbrainz.org/Work