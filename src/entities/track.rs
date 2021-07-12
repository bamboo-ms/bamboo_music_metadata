use super::artist::Artist;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};

// Representation of a single audio recording. The identifier is used to define associated audio
// files. For example, multiple files may exist in the library which are all different encodings of
// the same song, but the Track will only contain a single UUID that is shared with all of those
// files.

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub identifiers: Vec<Identifier>,
    pub name: String,
    pub artists: Vec<Artist>,
}
