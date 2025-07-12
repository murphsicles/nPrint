use nprint_types::{Sha256, Artifact, SmartContract};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio_stream::Stream;
use image::ImageReader;
use hound::WavReader;
use bytes::Bytes;
use sha2::{Digest, Sha256 as Sha256Digest};
use std::pin::Pin;

/// Trait for media processors: Verify on-chain, process off-chain async.
pub trait MediaProcessor {
    fn verify(&self, data: Vec<u8>, hash: Sha256) -> bool { Sha256Digest::digest(&data) == hash }
    fn process_stream(&self, stream: impl AsyncRead) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;
}

/// Image protocol template.
struct ImageProtocol { hash: Sha256 }
impl ImageProtocol {
    pub fn verify_image(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}
impl SmartContract for ImageProtocol {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.hash.to_script());
        Artifact { script, props: vec!["hash".to_string()] }
    }
}

/// Doc protocol (e.g., PDF hash verify; stub proc).
struct DocProtocol { hash: Sha256 }
impl DocProtocol {
    pub fn verify_doc(&self, chunks: Vec<Vec<u8>>) { let mut h = Sha256Digest::digest(&chunks[0]); for c in &chunks[1..] { h = Sha256Digest::digest(&[h.as_slice(), c.as_slice()].concat()); } assert_eq!(h.into(), self.hash); }
}
impl SmartContract for DocProtocol {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.hash.to_script());
        Artifact { script, props: vec!["hash".to_string()] }
    }
}

/// Music protocol (WAV hash, stream samples).
struct MusicProtocol { hash: Sha256 }
impl MusicProtocol {
    pub fn verify_music(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}
impl SmartContract for MusicProtocol {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.hash.to_script());
        Artifact { script, props: vec!["hash".to_string()] }
    }
}

/// Video streaming (chunked UTXOs, merkle verify).
struct VideoProtocol { root_hash: Sha256 }
impl VideoProtocol {
    pub fn unlock_chunk(&self, _chunk: Vec<u8>, _proof: Vec<u8>, _index: i128) { /* merkle verify stub */ }
}
impl SmartContract for VideoProtocol {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.root_hash.to_script());
        Artifact { script, props: vec!["root_hash".to_string()] }
    }
}
