use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::io::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;
    let mut db: DB = DB::open("./test/world/Chromebook Survival/db", options)
        .expect("Failed to open the database");

    let mut iter: DBIterator = db.new_iter().expect("Failed to create iterator");
    iter.seek_to_first();

    while iter.valid() {
        let (key, mut value): (String, Vec<u8>) = match iter.next() {
            None => break,
            Some((key, value)) => (String::from_utf8_lossy(&key).to_string(), value),
        };
        if !key.to_lowercase().contains("player") {
            continue;
        }
        value.truncate(12);
        println!("Key: {:?}\nValue: {:?}\n", key, value);
    }

    Ok(())
}
