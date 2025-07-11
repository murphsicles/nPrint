use nprint_dsl::{contract, prop, method};
use nprint_runtime::{deploy, Provider, PrivateKey};
use tokio::runtime::Runtime;
use sha2::{Digest, Sha256};

#[contract]
struct HashPuzzle {
    #[prop]
    hash: [u8; 32],
}

impl HashPuzzle {
    #[method]
    pub fn unlock(&self, preimage: Vec<u8>) {
        assert_eq!(Sha256::digest(&preimage).as_slice(), &self.hash);
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let contract = HashPuzzle { hash: Sha256::digest(b"secret").into() };
        let privkey = PrivateKey::from_wif("...").unwrap();
        let provider = Provider::new("https://api.whatsonchain.com/v1/bsv/main");
        let txid = deploy(&contract, &privkey, &provider).await.unwrap();
        println!("Deployed: {}", txid);
    });
}
