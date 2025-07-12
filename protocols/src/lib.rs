use nprint_dsl::{contract, method};
use nprint_types::{Sha256, PubKey, Sig};
use image::io::Reader as ImageReader;
use hound::WavReader;
use bytes::Bytes;
use std::pin::Pin;
use std::task::{Context, Poll};
use sha2::Digest;

#[contract]
struct ImageProtocol { hash: Sha256, }
impl ImageProtocol {
    #[method]
    pub fn verify_image(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}

#[contract]
struct DocProtocol { hash: Sha256, }
impl DocProtocol {
    #[method]
    pub fn verify_doc(&self, chunks: Vec<Vec<u8>>) { let mut h = Sha256Digest::digest(&chunks[0]); for c in &chunks[1..] { h = Sha256Digest::digest(&[h.as_slice(), c.as_slice()].concat()); } assert_eq!(h.into(), self.hash); }
}

#[contract]
struct MusicProtocol { hash: Sha256, }
impl MusicProtocol {
    #[method]
    pub fn verify_music(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}

#[contract]
struct VideoProtocol { root_hash: Sha256, }
impl VideoProtocol {
    #[method]
    pub fn unlock_chunk(&self, chunk: Vec<u8>, proof: Vec<u8>, index: i128) { /* merkle verify stub */ }
}
