use super::track::Track;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};

// "...a work is a distinct intellectual or artistic creation, which can be expressed in the form
// of one or more audio recordings." [1]

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Work {
    pub identifiers: Vec<Identifier>,
    pub name: String,
    pub artists: Vec<Identifier>,
    pub tracks: Vec<Track>,
}

// [1] https://wiki.musicbrainz.org/Work
