use nprint_dsl::{contract, prop, method};
use nprint_runtime::deploy;
use tokio::runtime::Runtime;

#[contract]
struct HashPuzzle {
    #[prop]
    hash: [u8; 32],
}

impl HashPuzzle {
    #[method]
    pub fn unlock(&self, preimage: Vec<u8>) {
        assert_eq!(sha256(&preimage), self.hash);
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let contract = HashPuzzle { hash: sha256(b"secret") };
        let privkey = PrivateKey::from_wif("...").unwrap();
        let provider = Provider::new("https://api.whatsonchain.com/v1/bsv/main");
        let txid = deploy(contract, privkey, provider).await.unwrap();
        println!("Deployed: {}", txid);
    });
}
