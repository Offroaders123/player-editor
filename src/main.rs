use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;
    let mut db: DB = DB::open("./test/world/Chromebook Survival/db", options)
        .expect("Failed to open the database");

    let mut iter: DBIterator = db.new_iter().expect("Failed to create iterator");

    while iter.valid() {
        let (key, value) = match iter.next() {
            None => Err(Error::new(ErrorKind::NotFound, "Empty iteration")),
            Some(entry) => Ok(entry),
        }?;
        println!("Key: {:?}, Value, {:?}", key, value);
    }

    Ok(())
}
