use nprint_core::bsv_script;
use sha2::{Digest, Sha256};
use sv::script::op_codes::{OP_SHA256, OP_EQUAL};

fn main() {
    let secret = b"secret_message";
    let hash = Sha256::digest(secret);
    let hash_bytes = hash.as_slice();
    let script = bsv_script! { OP_SHA256, { hash_bytes }, OP_EQUAL };
    println!("Hash Puzzle Script: {:?}", script);
}
