mod expect_exit;

use crate::expect_exit::ExpectExit;
use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::env::args;
use std::fs::{create_dir_all, write};
use std::io::Result;

fn main() -> Result<()> {
    println!("player-editor");

    let args: Vec<String> = args().collect();

    let world_dir: &String = &args
        .get(1)
        .expect_exit("Please pass the world folder you'd like to extract from");
    let player_dir: String = format!("{world_dir}/_player");
    let db_dir: String = format!("{world_dir}/db");

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;

    println!("Opening world {world_dir}\n");

    let mut db: DB = DB::open(db_dir, options).expect_exit("Failed to open database");

    let mut iter: DBIterator = db
        .new_iter()
        .expect_exit("Failed to create database iterator");
    iter.seek_to_first();

    println!("Searching for player entries...");

    create_dir_all(&player_dir)?;

    while iter.valid() {
        let (key, value): (String, Vec<u8>) = match iter.next() {
            Some((key, value)) => (String::from_utf8_lossy(&key).to_string(), value),
            None => break,
        };

        if !key.contains("player") {
            continue;
        }

        println!("{key}");

        let path: String = format!("{player_dir}/{key}.nbt");
        write(path, value)?;
    }

    Ok(())
}
