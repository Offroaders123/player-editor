use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::fs::{create_dir_all, write};
use std::io::Result;

fn main() -> Result<()> {
    println!("player-editor");

    create_dir_all("./test/world/Chromebook Survival/_player")?;

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;

    let mut db: DB = DB::open("./test/world/Chromebook Survival/db", options)
        .expect("Failed to open the database");

    let mut iter: DBIterator = db.new_iter().expect("Failed to create iterator");
    iter.seek_to_first();

    println!("Finding player entries...");

    while iter.valid() {
        let (key, value): (String, Vec<u8>) = match iter.next() {
            Some((key, value)) => (String::from_utf8_lossy(&key).to_string(), value),
            None => break,
        };

        if !key.to_lowercase().contains("player") {
            continue;
        }

        println!("{key}");

        let path: String = format!("./test/world/Chromebook Survival/_player/{key}.nbt");
        write(path, value)?;
    }

    Ok(())
}
