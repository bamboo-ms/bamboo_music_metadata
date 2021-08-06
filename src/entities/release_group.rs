use super::release::Release;
use crate::identifier::Identifier;
use serde::{Deserialize, Serialize};

// "A release group... is used to group several different releases into a single logical entity.
// Every release belongs to one, and only one, release group." [1]

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Album,
    Single,
    EP,
    Broadcast,
    // For all media that does not fit into the above categories
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseGroup {
    pub identifiers: Vec<Identifier>,
    // "The title of a release group is usually very similar, if not the same, as the titles of the
    // releases contained within it." [1]
    pub title: String,

    pub artists: Vec<Identifier>,
    // "The type of a release group describes what *kind* of release group it is. It is divided
    // in two: a release group can have a "main" type and an unspecified number of extra types."
    // [2]
    // Bamboo currently only recognizes the main release group type.
    pub r#type: Option<Type>,
    pub releases: Vec<Release>,
}

// [1] https://wiki.musicbrainz.org/Release_Group
// [2] https://wiki.musicbrainz.org/Release_Group/Type
