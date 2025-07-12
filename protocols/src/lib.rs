use nprint_dsl::{contract, prop, method, SmartContract};
use nprint_templates::REGISTRY;
use nprint_core::{bsv_script, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio_stream::Stream;
use image::ImageReader;
use hound::WavReader;
use bytes::Bytes;
use std::pin::Pin;
use std::task::{Context, Poll};
use sha2::{Digest, Sha256 as Sha256Digest};

/// Trait for media processors: Verify on-chain, process off-chain async.
pub trait MediaProcessor {
    fn verify(&self, data: Vec<u8>, hash: Sha256) -> bool { Sha256Digest::digest(&data) == hash }
    fn process_stream(&self, stream: impl AsyncRead) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;
}

/// Image protocol template.
#[contract]
struct ImageProtocol { #[prop] hash: Sha256, }
impl ImageProtocol {
    #[method]
    pub fn verify_image(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}

/// Doc protocol (e.g., PDF hash verify; stub proc).
#[contract]
struct DocProtocol { #[prop] hash: Sha256, }
impl DocProtocol {
    #[method]
    pub fn verify_doc(&self, chunks: Vec<Vec<u8>>) { let mut h = Sha256Digest::digest(&chunks[0]); for c in &chunks[1..] { h = Sha256Digest::digest(&[h.as_slice(), c.as_slice()].concat()); } assert_eq!(h.into(), self.hash); }
}

/// Music protocol (WAV hash, stream samples).
#[contract]
struct MusicProtocol { #[prop] hash: Sha256, }
impl MusicProtocol {
    #[method]
    pub fn verify_music(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}

/// Video streaming (chunked UTXOs, merkle verify).
#[contract]
struct VideoProtocol { #[prop] root_hash: Sha256, }
impl VideoProtocol {
    /// Usage: let proto = VideoProtocol { root_hash: ... }; let stream = proto.process_stream(file);
    #[method]
    pub fn unlock_chunk(&self, _chunk: Vec<u8>, _proof: Vec<u8>, _index: i128) { /* merkle verify stub */ }
}
