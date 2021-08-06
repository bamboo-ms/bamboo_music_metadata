use std::fs::File;
use std::io::Read;

use bamboo_music_metadata::entities::artist::Artist;

fn main() -> std::io::Result<()> {
    let mut json_file = File::open("examples/artist/billie_eilish.json")?;
    let mut contents = String::new();
    json_file.read_to_string(&mut contents)?;

    let billie_elish: Artist = serde_json::from_str(&contents)?;
    println!("Successfully deserialized file.");

    println!("{:#?}", &billie_elish);
    Ok(())
}
