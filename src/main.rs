use rusty_leveldb::{compressor::SnappyCompressor, CompressorId, DBIterator, Options, DB};
use std::io::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;
    let mut db: DB = DB::open("./test/world/Chromebook Survival/db", options)
        .expect("Failed to open the database");

    let iter: DBIterator = db.new_iter().expect("Failed to create iterator");

    for (key, value) in iter {
        println!("Key: {}, Value, {}", key, value);
    }

    Ok(())
}
