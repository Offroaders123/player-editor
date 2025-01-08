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
        let (key, value): (Vec<u8>, Vec<u8>) = match iter.next() {
            None => break,
            Some(entry) => entry,
        };
        let key_string: String = String::from_utf8(key).expect("Couldn't parse as string");
        println!("Key: {:?}", key_string);
    }

    Ok(())
}
