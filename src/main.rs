use rusty_leveldb::{compressor::SnappyCompressor, CompressorId, Options, DB};
use std::io::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;
    let db: DB = DB::open("./test/world/Chromebook Survival/db", options)
        .expect("Failed to open the database");
    Ok(())
}
