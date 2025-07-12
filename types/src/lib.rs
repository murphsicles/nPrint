use serde::{Serialize, Deserialize, Serializer, ser::SerializeSeq};
use nprint_core::bsv_script;  // Import from core

/// sCrypt-like data types as traits/generics.
pub trait ScryptType: ToScript + Serialize {}

impl ScryptType for i128 {}  // BigInt
impl ScryptType for Vec<u8> {}  // ByteString
impl ScryptType for usize {}  // For m in Multisig
impl ScryptType for [u8; 20] {}  // For pkh in P2PKH
impl ScryptType for [u8; 32] {}  // For hash in Hashlock, SHAGate, etc.

pub type PubKey = Vec<u8>;
impl ScryptType for PubKey {}

pub type Sig = Vec<u8>;
impl ScryptType for Sig {}

pub type Sha256 = [u8; 32];
impl ScryptType for Sha256 {}

pub struct FixedArray<T, const N: usize>([T; N]);

impl<T: ScryptType, const N: usize> ScryptType for FixedArray<T, N> where T: Serialize {}

impl<T: ScryptType, const N: usize> FixedArray<T, N> {
    pub fn new(arr: [T; N]) -> Self {
        FixedArray(arr)
    }
}

/// Trait to convert to BSV script pushes.
pub trait ToScript {
    fn to_script(&self) -> Vec<u8>;
}
impl ToScript for i128 {
    fn to_script(&self) -> Vec<u8> { bsv_script! { *self as i32 } }  // Simplify
}
impl ToScript for Vec<u8> {
    fn to_script(&self) -> Vec<u8> { self.clone() }
}
impl ToScript for usize {
    fn to_script(&self) -> Vec<u8> { bsv_script! { *self as i32 } }
}
impl ToScript for [u8; 20] {
    fn to_script(&self) -> Vec<u8> { self.to_vec() }
}
impl ToScript for [u8; 32] {
    fn to_script(&self) -> Vec<u8> { self.to_vec() }
}
impl ToScript for PubKey {
    fn to_script(&self) -> Vec<u8> { self.clone() }
}
impl ToScript for Sig {
    fn to_script(&self) -> Vec<u8> { self.clone() }
}
impl<T: ToScript, const N: usize> ToScript for FixedArray<T, N> where T: Serialize {
    fn to_script(&self) -> Vec<u8> {
        let mut script = Vec::new();
        for item in &self.0 { script.extend(item.to_script()); }
        script
    }
}

impl<T: Serialize, const N: usize> Serialize for FixedArray<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(N))?;
        for item in &self.0 {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

/// Artifact: Compiled contract output (JSON serializable).
#[derive(Serialize, Deserialize)]
pub struct Artifact {
    pub script: Vec<u8>,
    pub props: Vec<String>,  // Prop names
}

/// Trait for contracts.
pub trait SmartContract {
    fn compile(&self) -> Artifact;
}
