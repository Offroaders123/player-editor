use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::env::args;
use std::fs::{create_dir_all, write};
use std::io::Result;

fn main() -> Result<()> {
    println!("player-editor");

    let args: Vec<String> = args().collect();
    println!("{:?}", args);

    let world_dir: &String = &args
        .get(1)
        .expect("Please pass the world folder you'd like to extract from");
    let player_dir: String = format!("{world_dir}/_player");
    println!("{player_dir}");
    let db_dir: String = format!("{world_dir}/db");

    create_dir_all(&player_dir)?;

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;

    let mut db: DB = DB::open(db_dir, options).expect("Failed to open the database");

    let mut iter: DBIterator = db.new_iter().expect("Failed to create iterator");
    iter.seek_to_first();

    println!("Finding player entries...");

    while iter.valid() {
        let (key, value): (String, Vec<u8>) = match iter.next() {
            Some((key, value)) => (String::from_utf8_lossy(&key).to_string(), value),
            None => break,
        };

        // This needs to be case-sensitive actually!
        if !key.to_lowercase().contains("player") {
            continue;
        }

        println!("{key}");

        let path: String = format!("{player_dir}/{key}.nbt");
        write(path, value)?;
    }

    Ok(())
}
