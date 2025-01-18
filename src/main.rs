mod expect_exit;

use crate::expect_exit::ExpectExit;
use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::env::args;
use std::fs::{create_dir_all, write};
use std::io::{ErrorKind, Result};
use std::path::{Path, PathBuf};

enum EditMode {
    Read,
    Write,
}

fn main() -> Result<()> {
    println!("player-editor");

    let args: Vec<String> = args().collect();

    let world_dir: &Path = Path::new(
        args.get(1)
            .expect_exit("Please pass the world folder you'd like to extract from"),
    );

    let mode: EditMode = match args
        .get(2)
        .expect_exit("Please specify the action you'd like to make; '--read' or '--write'")
        .as_str()
    {
        "--read" => Ok(EditMode::Read),
        "--write" => Ok(EditMode::Write),
        _ => Err(ErrorKind::InvalidInput),
    }
    .expect_exit("Invalid action; '--read' or '--write'");

    let player_dir: PathBuf = world_dir.join("_player");
    let db_dir: PathBuf = world_dir.join("db");

    let mut options: Options = Options::default();
    options.create_if_missing = false;
    options.compressor = SnappyCompressor::ID;

    println!("Opening world {:?}\n", world_dir);

    let mut db: DB = DB::open(db_dir, options).expect_exit("Failed to open database");

    let mut iter: DBIterator = db
        .new_iter()
        .expect_exit("Failed to create database iterator");
    iter.seek_to_first();

    match mode {
        EditMode::Read => {
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

                let player_path: PathBuf = player_dir.join(format!("{key}.nbt"));
                write(player_path, value)?;
            }
        }
        EditMode::Write => {}
    }

    db.close().expect_exit("Failed to close database");

    Ok(())
}
