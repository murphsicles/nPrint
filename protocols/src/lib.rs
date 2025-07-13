use nprint_types::{SmartContract, Artifact, ToScript, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio_stream::Stream;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use bytes::Bytes;
use std::pin::Pin;
use sha2::{Digest, Sha256 as Sha256Digest};
use std::io::Cursor;
use async_stream::stream;

/// Trait for media processors: Verify on-chain, process off-chain async.
pub trait MediaProcessor {
    fn verify(&self, data: Vec<u8>, hash: Sha256) -> bool { Sha256Digest::digest(&data) == hash }
    fn process_stream(&self, stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;
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
impl MediaProcessor for DocProtocol {
    fn process_stream(&self, _stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        // Chunked read; yield parsed
        Box::pin(tokio_stream::empty())
    }
}

/// Audio protocol (multi-format audio hash, stream samples).
struct AudioProtocol { hash: Sha256 }
impl AudioProtocol {
    pub fn verify_audio(&self, data: Vec<u8>) { assert_eq!(Sha256Digest::digest(&data), self.hash); }
}
impl SmartContract for AudioProtocol {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.hash.to_script());
        Artifact { script, props: vec!["hash".to_string()] }
    }
}
impl MediaProcessor for AudioProtocol {
    fn process_stream(&self, mut stream: impl AsyncRead + Unpin + Send + 'static) -> Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>> {
        Box::pin(stream! {
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).await?;
            let cursor = Cursor::new(buf);
            let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

            let mut hint = Hint::new();
            // Set hint if known, e.g., hint.with_extension("mp3");

            let meta_opts: MetadataOptions = Default::default();
            let fmt_opts: FormatOptions = Default::default();

            let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            let mut format = probed.format;

            let track = format.tracks().iter().find(|t| t.codec_params.codec != CODEC_TYPE_NULL).ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No audio track"))?.clone();

            let dec_opts: DecoderOptions = Default::default();
            let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            loop {
                let packet = match format.next_packet() {
                    Ok(packet) => packet,
                    Err(symphonia::core::errors::Error::Eof) => break,
                    Err(e) => {
                        yield Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                        break;
                    }
                };

                if packet.track_id() != track.id {
                    continue;
                }

                match decoder.decode(&packet) {
                    Ok(decoded) => {
                        // Assume f32 samples for simplicity; convert if needed
                        if let Some(buffer) = decoded.frames() {
                            let samples = buffer.as_bytes().to_vec();
                            yield Ok(Bytes::from(samples));
                        }
                    }
                    Err(symphonia::core::errors::Error::IoError(_)) => continue,
                    Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
                    Err(e) => {
                        yield Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                        break;
                    }
                }
            }
        })
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
