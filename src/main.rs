use rusty_leveldb::{
    compressor::SnappyCompressor, CompressorId, DBIterator, LdbIterator, Options, DB,
};
use std::io::Result;

pub enum ChunkKey {
    Data3D = 43,
    Version,
    Data2D,
    Data2DLegacy,
    SubChunkPrefix,
    LegacyTerrain,
    BlockEntity,
    Entity,
    PendingTicks,
    LegacyBlockExtraData,
    BiomeState,
    FinalizedState,
    ConversionData,
    BorderBlocks,
    HardcodedSpawners,
    RandomTicks,
    CheckSums,
    GenerationSeed,
    GeneratedPreCavesAndCliffsBlending,
    BlendingBiomeHeight,
    MetaDataHash,
    BlendingData,
    ActorDigestVersion,
    LegacyVersion = 118,
}

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
        let (key, mut value): (Vec<u8>, Vec<u8>) = match iter.next() {
            None => break,
            Some(entry) => entry,
        };
        value.truncate(12);
        println!(
            "Key: {:?}\nValue: {:?}\n",
            String::from_utf8_lossy(&key),
            value
        );
    }

    Ok(())
}
