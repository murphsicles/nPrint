use nprint_dsl::{contract, prop, method, SmartContract};
use nprint_templates::REGISTRY;
use nprint_core::{bsv_script, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio_stream::Stream;
use image::io::Reader as ImageReader;
use hound::WavReader;
use bytes::Bytes;
use std::io::Cursor;
use sha2::{Digest, Sha256 as Sha256Digest};
use std::pin::Pin;
use async_stream::stream;

/// Trait for media processors: Verify on-chain, process off-chain async.
pub trait MediaProcessor {
    fn verify(&self, data: Vec<u8>, hash: Sha256) -> bool { Sha256Digest::digest(&data) == hash }
    fn process_stream(&self, stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;
}

/// Image protocol template.
#[contract]
struct ImageProtocol { #[prop] hash: Sha256, }

#[method]
impl ImageProtocol {
    pub fn verify_image(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}

impl MediaProcessor for ImageProtocol {
    fn process_stream(&self, mut stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        Box::pin(stream! {
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).await?;
            let img = ImageReader::new(Cursor::new(buf)).with_guessed_format()?.decode()?;
            // Process (e.g., resize); yield chunks
            yield Ok(Bytes::from(img.into_bytes()));
        })
    }
}

/// Doc protocol (e.g., PDF hash verify; stub proc).
#[contract]
struct DocProtocol { #[prop] hash: Sha256, }

#[method]
impl DocProtocol {
    pub fn verify_doc(&self, chunks: Vec<Vec<u8>>) { let mut h = Sha256Digest::digest(&chunks[0]); for c in &chunks[1..] { h = Sha256Digest::digest(&[h.as_slice(), c.as_slice()].concat()); } assert_eq!(h.into(), self.hash); }
}

impl MediaProcessor for DocProtocol {
    fn process_stream(&self, _stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        // Chunked read; yield parsed
        Box::pin(tokio_stream::empty())
    }
}

/// Music protocol (WAV hash, stream samples).
#[contract]
struct MusicProtocol { #[prop] hash: Sha256, }

#[method]
impl MusicProtocol {
    pub fn verify_music(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}

impl MediaProcessor for MusicProtocol {
    fn process_stream(&self, stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        Box::pin(stream! {
            let reader = WavReader::new(stream)?;
            for sample in reader.samples::<i16>() {
                yield Ok(Bytes::from(sample?.to_le_bytes().to_vec()));
            }
        })
    }
}

/// Video streaming (chunked UTXOs, merkle verify).
#[contract]
struct VideoProtocol { #[prop] root_hash: Sha256, }

#[method]
impl VideoProtocol {
    pub fn unlock_chunk(&self, _chunk: Vec<u8>, _proof: Vec<u8>, _index: i128) { /* merkle verify stub */ }
}

impl MediaProcessor for VideoProtocol {
    fn process_stream(&self, mut stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        Box::pin(stream! {
            let mut buf = [0; 4096];  // Chunk size
            loop {
                let n = stream.read(&mut buf).await?;
                if n == 0 { break; }
                // Verify chunk hash on-chain sim
                yield Ok(Bytes::from(buf[..n].to_vec()));
            }
        })
    }
}
