use nprint_dsl::{contract, prop, method};
use nprint_core::{bsv_script, xswap, hashcat, loop_unroll, execute_script}; // Add execute_script
use nprint_runtime::{deploy, Provider, PrivateKey};
use tokio::runtime::Runtime;
use sha2::{Digest, Sha256};
use sv::script::stack::Stack;

#[contract]
struct Composite {
    #[prop]
    hash: [u8; 32],
}

impl Composite {
    #[method]
    pub fn unlock(&self, data: Vec<u8>, n: i32) {
        let script = bsv_script! {
            { loop_unroll!(2, { OP_DUP }) }  // Unroll DUP twice
            { xswap!(n) }                    // Swap with nth
            { hashcat!() }                   // Dup, hash, cat
            OP_EQUALVERIFY                   // Compare with prop
        };
        // Simulate execution
        let mut stack = Stack::default();
        stack.push(data);
        execute_script(&mut stack, &script).unwrap(); // Use custom execute
        assert_eq!(stack.pop().unwrap_or_default(), self.hash.to_vec());
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let contract = Composite { hash: Sha256::digest(b"test").into() };
        let privkey = PrivateKey::from_wif("...").unwrap();
        let provider = Provider::new("https://api.whatsonchain.com/v1/bsv/main");
        let txid = deploy(&contract, &privkey, &provider).await.unwrap();
        println!("Deployed: {}", txid);
    });
}
