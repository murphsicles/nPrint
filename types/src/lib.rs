use serde::{Serialize, Deserialize};
use sv::script::op_codes::{OP_FALSE, OP_1};
use sv::script::stack::encode_num;
use nprint_core::bsv_script;

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

#[derive(Clone, Copy, Debug)]
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
    fn to_script(&self) -> Vec<u8> { bsv_script! { *self } }  // Simplify, note: consider full big int for large values
}

impl ToScript for i64 {
    fn to_script(&self) -> Vec<u8> { bsv_script! { *self as i32 } }
}
