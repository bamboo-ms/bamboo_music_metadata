use super::{artist::Artist, work::Work};
use crate::identifier::Identifier;
use bamboo_metadata::language::Language;
use chrono::NaiveDate;
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

// "A MusicBrainz release represents the unique release (i.e. issuing) of a product on a specified
// date with specific release information such as the country, label, barcode, and packing."

#[derive(Serialize, Deserialize)]
pub struct Release {
    pub identifiers: Vec<Identifier>,
    pub title: String,
    // "The artist(s) that the release is primarily credited to, as credited on the release." [1]
    pub artists: Vec<Artist>,
    // "The date the release was issued." [1]
    pub date: NaiveDate,
    // "The country the release was issued in." [1]
    pub country: CountryCode,
    // "The Language a release's track list is written in." [1]
    pub language: Language,
    pub works: Vec<Work>,
}

// [1] https://wiki.musicbrainz.org/Release
