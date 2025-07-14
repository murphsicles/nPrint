use nprint_types::{SmartContract, Artifact, ToScript, Sha256};
use nprint_core::bsv_script;
use sha2::{Sha256 as Sha256Digest, Digest};
use alloc::collections::BTreeMap as HashMap;
use alloc::vec::Vec;

fn sha256(msg: &Vec<u8>) -> Sha256 { 
    let digest = Sha256Digest::digest(msg);
    let mut array = [0u8; 32];
    array.copy_from_slice(digest.as_slice());
    Sha256(array)
}

fn compute_sha_gate(input: &Vec<u8>) -> Sha256 { sha256(input) }

fn merkle_proof(_branch: &Vec<u8>, _proof: &Vec<u8>) -> Sha256 { Sha256([0; 32]) } // Stub

pub use sv::script::op_codes::{OP_DUP, OP_HASH160, OP_EQUALVERIFY, OP_CHECKSIG, OP_EQUAL, OP_CHECKSEQUENCEVERIFY, OP_DROP, OP_SHA256, OP_CAT, OP_NUM2BIN, OP_BIN2NUM, OP_SPLIT, OP_SUBSTR, OP_LEFT, OP_RIGHT, OP_SIZE, OP_INVERT, OP_AND, OP_OR, OP_XOR, OP_LSHIFT, OP_RSHIFT, OP_2DROP, OP_2DUP, OP_3DUP, OP_2OVER, OP_2ROT, OP_2SWAP, OP_IFDUP, OP_DEPTH, OP_NIP, OP_OVER, OP_PICK, OP_ROLL, OP_ROT, OP_SWAP, OP_TUCK};

#[derive(Clone, Debug)]
pub struct P2PKH {
    pub pkh: [u8; 20],
}

impl SmartContract for P2PKH {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(bsv_script! { OP_DUP, OP_HASH160 });
        script.extend(self.pkh.to_script());
        script.extend(bsv_script! { OP_EQUALVERIFY, OP_CHECKSIG });
        Artifact { script, props: vec!["pkh".to_string()] }
    }
}

impl P2PKH {
    pub fn unlock(&self, _pubkey: PubKey, _sig: Sig) {}
}

impl ToScript for [u8; 20] {
    fn to_script(&self) -> Vec<u8> {
        let mut v = vec![0x14];
        v.extend_from_slice(self);
        v
    }
}

#[derive(Clone, Debug)]
pub struct Multisig {
    pub pubkeys: Vec<PubKey>,
    pub m: usize,
}

impl SmartContract for Multisig {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.m.to_script());
        for pk in &self.pubkeys {
            script.extend(pk.to_script());
        }
        script.extend((self.pubkeys.len() as u8).to_script());
        script.extend(bsv_script! { OP_CHECKMULTISIG });
        Artifact { script, props: vec!["pubkeys".to_string(), "m".to_string()] }
    }
}

impl Multisig {
    pub fn unlock(&self, _sigs: Vec<Sig>) {}
}

impl ToScript for usize {
    fn to_script(&self) -> Vec<u8> { bsv_script! { *self as i32 } }
}

impl ToScript for u8 {
    fn to_script(&self) { bsv_script! { *self as i32 } }
}

#[derive(Clone, Debug)]
pub struct Timelock {
    pub timeout: i128,
}

impl SmartContract for Timelock {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.timeout.to_script());
        script.extend(bsv_script! { OP_CHECKSEQUENCEVERIFY, OP_DROP });
        Artifact { script, props: vec!["timeout".to_string()] }
    }
}

impl ToScript for i128 {
    fn to_script(&self) -> Vec<u8> { bsv_script! { *self as i32 } }
}

#[derive(Clone, Debug)]
pub struct Hashlock {
    pub hash: Sha256,
}

impl SmartContract for Hashlock {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(bsv_script! { OP_SHA256 });
        script.extend(self.hash.to_script());
        script.extend(bsv_script! { OP_EQUAL });
        Artifact { script, props: vec!["hash".to_string()] }
    }
}

impl Hashlock {
    pub fn unlock(&self, _msg: Vec<u8>) { assert_eq!(sha256(&_msg), self.hash); }
}

#[derive(Clone, Debug)]
pub struct RabinSig {
    pub rabin_pk: i128,
}

impl SmartContract for RabinSig {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.rabin_pk.to_script());
        script.extend(bsv_script! { /* rabin verify stub */ });
        Artifact { script, props: vec!["rabin_pk".to_string()] }
    }
}

impl RabinSig {
    pub fn unlock(&self, _sig: i128, _msg: Vec<u8>) { /* rabin verify stub */ }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tick: Vec<u8>,
    pub max: i128,
    pub data: Vec<u8>,
}

impl SmartContract for Token {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.tick.to_script());
        script.extend(self.max.to_script());
        script.extend(self.data.to_script());
        Artifact { script, props: vec!["tick".to_string(), "max".to_string(), "data".to_string()] }
    }
}

impl Token {
    pub fn mint(&self, _amount: i128, _to: Vec<u8>) { /* mint stub */ }
    pub fn transfer(&self, _amount: i128, _to: Vec<u8>) { /* transfer stub */ }
}

impl ToScript for Vec<u8> {
    fn to_script(&self) -> Vec<u8> {
        let mut v = vec![self.len() as u8];
        v.extend(self.clone());
        v
    }
}

#[derive(Clone, Debug)]
pub struct NFT {
    pub id: Vec<u8>,
}

impl SmartContract for NFT {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.id.to_script());
        Artifact { script, props: vec!["id".to_string()] }
    }
}

impl NFT {
    pub fn transfer(&self, _to: Vec<u8>) { /* transfer stub */ }
}

#[derive(Clone, Debug)]
pub struct LoopUnroll {
    pub count: i128,
}

impl SmartContract for LoopUnroll {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.count.to_script());
        Artifact { script, props: vec!["count".to_string()] }
    }
}

#[derive(Clone, Debug)]
pub struct SHAGate {
    pub hash: Sha256,
}

impl SmartContract for SHAGate {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(bsv_script! { OP_CAT, OP_SHA256 });
        script.extend(self.hash.to_script());
        script.extend(bsv_script! { OP_EQUAL });
        Artifact { script, props: vec!["hash".to_string()] }
    }
}

impl SHAGate {
    pub fn unlock(&self, a: Vec<u8>, b: Vec<u8>) { assert_eq!(compute_sha_gate(&[a, b].concat()), self.hash); }
}

#[derive(Clone, Debug)]
pub struct DriveChain {
    pub peg_hash: Sha256,
}

impl SmartContract for DriveChain {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.peg_hash.to_script());
        script.extend(bsv_script! { /* peg verify stub */ });
        Artifact { script, props: vec!["peg_hash".to_string()] }
    }
}

impl DriveChain {
    pub fn peg_in(&self, _amount: i128, _proof: Vec<u8>) { /* peg in stub */ }
    pub fn peg_out(&self, _amount: i128, _to: Vec<u8>) { /* peg out stub */ }
}

#[derive(Clone, Debug)]
pub struct MAST {
    pub root: Sha256,
}

impl SmartContract for MAST {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.root.to_script());
        script.extend(bsv_script! { /* mast verify stub */ });
        Artifact { script, props: vec!["root".to_string()] }
    }
}

impl MAST {
    pub fn execute_branch(&self, branch: Vec<u8>, proof: Vec<u8>) { assert_eq!(merkle_proof(&branch, &proof), self.root); }
}

pub type TemplateFn = fn(HashMap<String, Vec<u8>>) -> Artifact;

lazy_static::lazy_static! {
    pub static ref REGISTRY: HashMap<String, TemplateFn> = {
        let mut m = HashMap::new();
        m.insert("hashlock".to_string(), |params| Hashlock { hash: Sha256(params["hash"].clone().try_into().unwrap()) }.compile());
        m.insert("shagate".to_string(), |params| SHAGate { hash: Sha256(params["hash"].clone().try_into().unwrap()) }.compile());
        m.insert("drivechain".to_string(), |params| DriveChain { peg_hash: Sha256(params["peg_hash"].clone().try_into().unwrap()) }.compile());
        m.insert("mast".to_string(), |params| MAST { root: Sha256(params["root"].clone().try_into().unwrap()) }.compile());
        m
    };
}
