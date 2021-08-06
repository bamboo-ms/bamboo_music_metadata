// Information required to uniquely identify the media in one or more databases

use bamboo_metadata::id_provider::IdProvider;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Identifier {
    // local Bamboo database ID
    // Locally used to identify entities for music
    Bmbo(Uuid),
    // MusicBrainz ID
    Mbid(Uuid),
    // https://wiki.musicbrainz.org/ISRC
    // International Standards Recording Code
    // See musicbrainz wiki for constraints
    // TODO: Create an Isrc type that enforces ISRC constraints
    Isrc(String),
    // https://en.wikipedia.org/wiki/International_Standard_Musical_Work_Code
    // International Standard Musical Work Code
    // TODO: Create an Iswc type that enforced Iswc constraints
    Iswc(String),
    // https://wiki.musicbrainz.org/IPI
    // Interested Parties Information Code
    Ipi(String),
    // https://wiki.musicbrainz.org/ISNI
    // International Standard Name Identifier
    Isni(String),
    // https://wiki.musicbrainz.org/Label/LabelCode
    // The Label Code, per IFPI (International Federation of Phonogram and Videogram Industries)
    Lc(String),
}

impl IdProvider for Identifier {
    fn database_id(&self) -> String {
        match self {
            Identifier::Bmbo(_) => "bmbo".to_string(),
            Identifier::Mbid(_) => "mbid".to_string(),
            Identifier::Isrc(_) => "isrc".to_string(),
            Identifier::Iswc(_) => "iswc".to_string(),
            Identifier::Ipi(_) => "ipi".to_string(),
            Identifier::Isni(_) => "insi".to_string(),
            Identifier::Lc(_) => "lc".to_string(),
        }
    }

    fn database_value(&self) -> String {
        match self {
            Identifier::Bmbo(id) => id.to_string(),
            Identifier::Mbid(id) => id.to_string(),
            Identifier::Isrc(id) => id.clone(),
            Identifier::Iswc(id) => id.clone(),
            Identifier::Ipi(id) => id.clone(),
            Identifier::Isni(id) => id.clone(),
            Identifier::Lc(id) => id.clone(),
        }
    }
}
