use super::{artist::Artist, work::Work};
use crate::identifier::Identifier;
use chrono::NaiveDate;
use iso639_enum::Iso639;
use isocountry::CountryCode;

// "A MusicBrainz release represents the unique release (i.e. issuing) of a product on a specified
// date with specific release information such as the country, label, barcode, and packing."

pub struct Release {
    identifiers: Vec<Identifier>,
    title: String,
    // "The artist(s) that the release is primarily credited to, as credited on the release." [1]
    artists: Vec<Artist>,
    // "The date the release was issued." [1]
    date: NaiveDate,
    // "The country the release was issued in." [1]
    country: CountryCode,
    // "The Language a release's track list is written in." [1]
    language: Iso639,
    works: Vec<Work>,
}

// [1] https://wiki.musicbrainz.org/Release
