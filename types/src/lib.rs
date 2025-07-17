use nprint_core::bsv_script;
use serde::{Deserialize, Serialize};
use sv::script::stack::encode_num;

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    pub script: Vec<u8>,
    pub props: Vec<String>,
}

pub trait SmartContract {
    fn compile(&self) -> Artifact;
}

pub trait ToScript {
    fn to_script(&self) -> Vec<u8>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sha256(pub [u8; 32]);

impl ToScript for Sha256 {
    fn to_script(&self) -> Vec<u8> {
        let mut script = Vec::new();
        script.push(32);
        script.extend_from_slice(&self.0);
        script
    }
}

impl ToScript for i32 {
    fn to_script(&self) -> Vec<u8> {
        let n = *self as i64;
        bsv_script! { n }
    }
}

impl ToScript for i64 {
    fn to_script(&self) -> Vec<u8> {
        let n = *self;
        bsv_script! { n }
    }
}

impl ToScript for i128 {
    fn to_script(&self) -> Vec<u8> {
        let bytes = encode_num(*self as i64).unwrap();
        let mut script = Vec::new();
        script.push(bytes.len() as u8);
        script.extend(bytes);
        script
    }
}

impl ToScript for usize {
    fn to_script(&self) -> Vec<u8> {
        let n = *self as i64;
        bsv_script! { n }
    }
}

impl ToScript for u8 {
    fn to_script(&self) -> Vec<u8> {
        let n = *self as i64;
        bsv_script! { n }
    }
}

impl ToScript for Vec<u8> {
    fn to_script(&self) -> Vec<u8> {
        let mut script = Vec::new();
        script.push(self.len() as u8);
        script.extend_from_slice(self);
        script
    }
}

impl ToScript for [u8; 20] {
    fn to_script(&self) -> Vec<u8> {
        let mut script = Vec::new();
        script.push(20);
        script.extend_from_slice(self);
        script
    }
}
