use serde::{Serialize, Deserialize};
use nprint_core::bsv_script;  // Import from core

/// sCrypt-like data types as traits/generics.
pub trait ScryptType: ToScript + Serialize {}

impl ScryptType for i128 {}  // BigInt
impl ScryptType for Vec<u8> {}  // ByteString

// Wrapper for fixed arrays to implement Serialize
#[derive(Serialize)]
pub struct FixedArray<T, const N: usize>([T; N]);

impl<T: ScryptType + Serialize, const N: usize> ScryptType for FixedArray<T, N> {}

// ToScript for FixedArray
impl<T: ToScript + Serialize, const N: usize> ToScript for FixedArray<T, N> {
    fn to_script(&self) -> Vec<u8> {
        let mut script = Vec::new();
        for item in &self.0 { script.extend(item.to_script()); }
        script
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
