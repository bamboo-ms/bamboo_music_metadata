use super::{artist::Artist, release::Release};
use crate::identifier::Identifier;

// "A release group... is used to group several different releases into a single logical entity.
// Every release belongs to one, and only one, release group." [1]

enum Type {
    Album,
    Single,
    EP,
    Broadcast,
    // For all media that does not fit into the above categories
    Other,
}

pub struct ReleaseGroup {
    identifiers: Vec<Identifier>,
    // "The title of a release group is usually very similar, if not the same, as the titles of the
    // releases contained within it." [1]
    title: String,
    artist: Option<Artist>,

    // "The type of a release group describes what *kind* of release group it is. It is divided
    // in two: a release group can have a "main" type and an unspecified number of extra types."
    // [2]
    // Bamboo currently only recognizes the main release group type.
    r#type: Option<Type>,
    releases: Vec<Release>,
}

// [1] https://wiki.musicbrainz.org/Release_Group
// [2] https://wiki.musicbrainz.org/Release_Group/Type
