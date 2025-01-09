use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::env::args;
use std::fs::{create_dir_all, write};
use std::io::Result;
use std::process::exit;

fn main() -> Result<()> {
    println!("player-editor");

    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("Please pass the world folder you'd like to extract from");
        exit(1);
    }

    let world_dir: &String = &args[1];
    let player_dir: String = format!("{world_dir}/_player");
    let db_dir: String = format!("{world_dir}/db");

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;

    println!("Opening world {world_dir}\n");

    let mut db: DB = DB::open(db_dir, options).expect("Failed to open database");

    let mut iter: DBIterator = db.new_iter().expect("Failed to create database iterator");
    iter.seek_to_first();

    println!("Searching for player entries...");

    create_dir_all(&player_dir)?;

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
