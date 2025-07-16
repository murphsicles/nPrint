use nprint_core::bsv_script;
use sha2::{Digest, Sha256};
use sv::script::op_codes::{OP_SHA256, OP_EQUAL};

fn main() {
    let secret = b"secret_message";
    let hash = Sha256::digest(secret);
    let mut script = bsv_script! { OP_SHA256 };
    script.extend_from_slice(&[hash.len() as u8]);
    script.extend_from_slice(&hash);
    script.extend(bsv_script! { OP_EQUAL });
    println!("Hash Puzzle Script: {script:?}");
}
