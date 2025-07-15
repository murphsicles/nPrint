use async_stream::stream;
use bytes::Bytes;
use image::{ImageBuffer, Rgba};
use nprint_core::Stack;
use nprint_types::Sha256;
use sha2::{Digest, Sha256 as Sha256Digest};
use std::vec::Vec;
use symphonia::core::audio::AudioBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
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
    pub fn stream_image(&self, file: impl AsyncRead + Unpin) -> Stream<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        stream! { yield ImageBuffer::new(1, 1); } // Stub
    }
}

pub struct VideoProtocol {
    pub hash: Sha256,
}

impl VideoProtocol {
    pub fn verify_video(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data).as_slice(), self.hash.0.as_slice()); }
    pub fn stream_video(&self, file: impl AsyncRead + Unpin) -> Stream<Bytes> {
        stream! { yield Bytes::new(); } // Stub
    }
}

pub struct AudioProtocol {
    pub hash: Sha256,
}

impl AudioProtocol {
    pub fn verify_audio(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data).as_slice(), self.hash.0.as_slice()); }
    pub fn stream_audio(&self, file: impl AsyncRead + Unpin) -> Stream<AudioBuffer<f32>> {
        stream! { yield AudioBuffer::new(1, Default::default()); } // Stub
    }
}

pub struct DocProtocol {
    pub hash: Sha256,
}

impl DocProtocol {
    pub fn verify_doc(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data).as_slice(), self.hash.0.as_slice()); }
    pub fn stream_doc(&self, file: impl AsyncRead + Unpin) -> Stream<String> {
        stream! { yield String::new(); } // Stub
    }
}
