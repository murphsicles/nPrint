use nprint_dsl::{contract, prop, method};
use nprint_core::{bsv_script, xswap, hashcat, loop_unroll};
use nprint_runtime::{deploy, Provider, PrivateKey};
use tokio::runtime::Runtime;

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
        // Simulate execution (in practice, script is compiled)
        let mut stack = Stack::default();
        stack.push(data);
        stack.execute(&script).unwrap();
        assert_eq!(stack.pop(), self.hash);
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let contract = Composite { hash: sha256(b"test") };
        let privkey = PrivateKey::from_wif("...").unwrap();
        let provider = Provider::new("https://api.whatsonchain.com/v1/bsv/main");
        let txid = deploy(contract, privkey, provider).await.unwrap();
        println!("Deployed: {}", txid);
    });
}
