use nprint_types::{SmartContract, Artifact, ToScript, Sha256};
use image::ImageReader;
use hound::WavReader;
use bytes::Bytes;
use std::pin::Pin;
use std::task::{Context, Poll};
use sha2::{Digest, Sha256 as Sha256Digest};

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

struct VideoProtocol { root_hash: Sha256 }
impl VideoProtocol {
    pub fn unlock_chunk(&self, chunk: Vec<u8>, proof: Vec<u8>, index: i128) { /* merkle verify stub */ }
}
impl SmartContract for VideoProtocol {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.root_hash.to_script());
        Artifact { script, props: vec!["root_hash".to_string()] }
    }
}
