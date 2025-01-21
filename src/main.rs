mod expect_exit;
mod mojang_options;

use std::env::args;
use std::fs::{create_dir_all, read, read_dir, write, DirEntry};
use std::io::{ErrorKind, Result};
use std::path::{Path, PathBuf};

use crate::expect_exit::ExpectExit;
use crate::mojang_options::mojang_options;
use rusty_leveldb::{DBIterator, LdbIterator, Options, DB};

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

    let mut options: Options = mojang_options();
    options.create_if_missing = false;

    println!("Opening world {:?}\n", world_dir);

    let mut db: DB = DB::open(db_dir, options).expect_exit("Failed to open database");

    match mode {
        EditMode::Read => read_mode(player_dir, &mut db)?,
        EditMode::Write => write_mode(player_dir, &mut db)?,
    }

    db.close().expect_exit("Failed to close database");

    Ok(())
}

fn read_mode(player_dir: PathBuf, db: &mut DB) -> Result<()> {
    let mut iter: DBIterator = db
        .new_iter()
        .expect_exit("Failed to create database iterator");
    iter.seek_to_first();

    println!("Searching for player entries...");

    create_dir_all(&player_dir)?;

    while iter.valid() {
        let (key, value): (String, Vec<u8>) = match iter.next() {
            Some((key, value)) => (
                String::from_utf8(key).expect_exit("Could not convert key to UTF-8"),
                value,
            ),
            None => break,
        };

        if !key.contains("player") {
            continue;
        }

        println!("{key}");

        let player_path: PathBuf = player_dir.join(format!("{key}.nbt"));
        write(player_path, value)?;
    }

    Ok(())
}

fn write_mode(player_dir: PathBuf, db: &mut DB) -> Result<()> {
    println!("Looking for edited player files...");

    let entries: Vec<DirEntry> = read_dir(player_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    for entry in entries {
        let path: PathBuf = entry.path();

        if !path.is_file() {
            continue;
        }

        if !path
            .extension()
            .map_or(false, |extension| extension == "nbt")
        {
            continue;
        }

        if !path
            .to_str()
            .expect_exit("Could not convert file name to UTF-8")
            .contains("player")
        {
            continue;
        }

        let key: &str = path
            .file_stem()
            .expect_exit("File does not have a basename")
            .to_str()
            .expect_exit("Could not convert file name to UTF-8");

        if db.get(key.as_bytes()).is_none() {
            continue;
        }

        let edited: &[u8] = &read(&path)?;

        let original: &[u8] = &db
            .get(key.as_bytes())
            .expect_exit("Could not find key in database");

        if edited != original {
            println!("{key} <EDITED>");

            db.put(key.as_bytes(), edited)
                .expect_exit("Could not write file to database");
        } else {
            println!("{key}");
        }
    }

    Ok(())
}
