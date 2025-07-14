use nprint_types::{SmartContract, Artifact, ToScript, Sha256};
use nprint_core::bsv_script;
use sha2::{Digest, Sha256 as Sha256Digest};
use std::collections::HashMap;
use std::vec::Vec;

/// Placeholder functions
fn compute_sha_gate(input: &Vec<u8>) -> Sha256 { Sha256Digest::digest(input).into() } // Stub
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

#[derive(Clone, Debug)]
pub struct Multisig {
    pub pubkeys: Vec<Vec<u8>>,
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

#[derive(Clone, Debug)]
pub struct RabinSig {
    pub rabin_pk: i128,
}

impl SmartContract for RabinSig {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.rabin_pk.to_script());
        Artifact { script, props: vec!["rabin_pk".to_string()] }
    }
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

#[derive(Clone, Debug)]
pub struct DriveChain {
    pub peg_hash: Sha256,
}

impl SmartContract for DriveChain {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.peg_hash.to_script());
        Artifact { script, props: vec!["peg_hash".to_string()] }
    }
}

#[derive(Clone, Debug)]
pub struct MAST {
    pub root: Sha256,
}

impl SmartContract for MAST {
    fn compile(&self) -> Artifact {
        let mut script = Vec::new();
        script.extend(self.root.to_script());
        Artifact { script, props: vec!["root".to_string()] }
    }
}

pub type TemplateFn = fn(HashMap<String, Vec<u8>>) -> Artifact;

lazy_static::lazy_static! {
    pub static ref REGISTRY: HashMap<String, TemplateFn> = {
        let mut m = HashMap::new();
        m.insert("p2pkh".to_string(), |params| P2PKH { pkh: params["pkh"].clone().try_into().unwrap() }.compile());
        m.insert("multisig".to_string(), |params| Multisig { pubkeys: vec![], m: params["m"][0] as usize }.compile());
        m.insert("timelock".to_string(), |params| Timelock { timeout: i128::from_le_bytes(params["timeout"].clone().try_into().unwrap()) }.compile());
        m.insert("hashlock".to_string(), |params| Hashlock { hash: params["hash"].clone().try_into().unwrap() }.compile());
        m.insert("rabinsig".to_string(), |params| RabinSig { rabin_pk: i128::from_le_bytes(params["rabin_pk"].clone().try_into().unwrap()) }.compile());
        m.insert("token".to_string(), |params| Token { tick: params["tick"].clone(), max: i128::from_le_bytes(params["max"].clone().try_into().unwrap()), data: params["data"].clone() }.compile());
        m.insert("nft".to_string(), |params| NFT { id: params["id"].clone() }.compile());
        m.insert("loopunroll".to_string(), |params| LoopUnroll { count: i128::from_le_bytes(params["count"].clone().try_into().unwrap()) }.compile());
        m.insert("shagate".to_string(), |params| SHAGate { hash: params["hash"].clone().try_into().unwrap() }.compile());
        m.insert("drivechain".to_string(), |params| DriveChain { peg_hash: params["peg_hash"].clone().try_into().unwrap() }.compile());
        m.insert("mast".to_string(), |params| MAST { root: params["root"].clone().try_into().unwrap() }.compile());
        m
    };
}
