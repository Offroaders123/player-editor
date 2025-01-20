use std::rc::Rc;

use miniz_oxide::{
    deflate::{compress_to_vec, compress_to_vec_zlib, CompressionLevel},
    inflate::{decompress_to_vec, decompress_to_vec_zlib},
};
use rusty_leveldb::{
    compressor::NoneCompressor, Compressor, CompressorList, Options, Result, Status, StatusCode,
};

pub fn mojang_options() -> Options {
    let mut options: Options = Options::default();

    let compression_level: u8 = CompressionLevel::DefaultLevel as u8;

    let mut list: CompressorList = CompressorList::new();
    list.set_with_id(0, NoneCompressor {});
    list.set_with_id(2, ZlibCompressor::new(compression_level));
    list.set_with_id(4, RawZlibCompressor::new(compression_level));
    options.compressor_list = Rc::new(list);

    options.compressor = 4;

    options
}

struct ZlibCompressor(u8);

impl ZlibCompressor {
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for ZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> Result<Vec<u8>> {
        Ok(compress_to_vec_zlib(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> Result<Vec<u8>> {
        decompress_to_vec_zlib(&block).map_err(|err| Status {
            code: StatusCode::CompressionError,
            err: err.to_string(),
        })
    }
}

struct RawZlibCompressor(u8);

impl RawZlibCompressor {
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for RawZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> Result<Vec<u8>> {
        Ok(compress_to_vec(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> Result<Vec<u8>> {
        decompress_to_vec(&block).map_err(|err| Status {
            code: StatusCode::CompressionError,
            err: err.to_string(),
        })
    }
}
