use async_stream::stream;
use bytes::Bytes;
use image::{ImageBuffer, Rgba};
use nprint_types::Sha256;
use sha2::{Digest, Sha256 as Sha256Digest};
use std::vec::Vec;
use symphonia::core::audio::{AudioBuffer, SignalSpec, Channels};
use tokio::io::AsyncRead;
use tokio_stream::Stream;

pub trait MediaProtocol {
    fn verify(&self, data: Vec<u8>, hash: Sha256) -> bool;
}

pub struct ImageProtocol {
    pub hash: Sha256,
}

impl MediaProtocol for ImageProtocol {
    fn verify(&self, data: Vec<u8>, hash: Sha256) -> bool { Sha256Digest::digest(&data).as_slice() == hash.0.as_slice() } 
}

impl ImageProtocol {
    pub fn verify_image(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data).as_slice(), self.hash.0.as_slice()); }
    pub fn stream_image(&self, file: impl AsyncRead + Unpin) -> impl Stream<Item = ImageBuffer<Rgba<u8>, Vec<u8>>> {
        stream! { yield ImageBuffer::new(1, 1); } // Stub
    }
}

pub struct VideoProtocol {
    pub hash: Sha256,
}

impl VideoProtocol {
    pub fn verify_video(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data).as_slice(), self.hash.0.as_slice()); }
    pub fn stream_video(&self, file: impl AsyncRead + Unpin) -> impl Stream<Item = Bytes> {
        stream! { yield Bytes::new(); } // Stub
    }
}

pub struct AudioProtocol {
    pub hash: Sha256,
}

impl AudioProtocol {
    pub fn verify_audio(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data).as_slice(), self.hash.0.as_slice()); }
    pub fn stream_audio(&self, file: impl AsyncRead + Unpin) -> impl Stream<Item = AudioBuffer<f32>> {
        let spec = SignalSpec::new(44100, Channels::FRONT_LEFT);
        stream! { yield AudioBuffer::new(1, spec); } // Stub
    }
}

pub struct DocProtocol {
    pub hash: Sha256,
}

impl DocProtocol {
    pub fn verify_doc(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data).as_slice(), self.hash.0.as_slice()); }
    pub fn stream_doc(&self, file: impl AsyncRead + Unpin) -> impl Stream<Item = String> {
        stream! { yield String::new(); } // Stub
    }
}
