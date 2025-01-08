use rusty_leveldb::{Options, DB};
use std::io::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    let db: DB = DB::open(
        "./test/world/Chromebook Survival/db",
        Options {
            ..Default::default()
        },
    )
    .unwrap();
    Ok(())
}
