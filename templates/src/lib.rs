use nprint_dsl::{contract, prop, method, SmartContract, Artifact};
use nprint_core::{bsv_script, Sha256};  // Assume core has Sha256 type

/// Template registry: Fn to generate Artifact from params.
pub type Template = fn(params: &std::collections::HashMap<String, Vec<u8>>) -> Artifact;

pub static REGISTRY: std::sync::LazyLock<std::collections::HashMap<String, Template>> = std::sync::LazyLock::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert("P2PKH".to_string(), p2pkh_template as Template);
    map.insert("Hashlock".to_string(), hashlock_template as Template);
    // Add more: Multisig, Timelock, etc.
    map
});

fn p2pkh_template(params: &std::collections::HashMap<String, Vec<u8>>) -> Artifact {
    let pkh = params.get("pkh").unwrap().clone();
    #[contract]
    struct P2PKH {
        #[prop]
        pkh: [u8; 20],
    }
    impl P2PKH {
        #[method]
        pub fn unlock(&self, sig: Vec<u8>, pk: Vec<u8>) {
            let pk_hash = hash160(&pk);
            assert_eq!(pk_hash, self.pkh);
            check_sig(sig, pk);
        }
    }
    P2PKH { pkh: pkh.try_into().unwrap() }.compile()
}

fn hashlock_template(params: &std::collections::HashMap<String, Vec<u8>>) -> Artifact {
    let hash = params.get("hash").unwrap().clone();
    #[contract]
    struct Hashlock {
        #[prop]
        hash: [u8; 32],
    }
    impl Hashlock {
        #[method]
        pub fn unlock(&self, preimage: Vec<u8>) {
            assert_eq!(sha256(&preimage), self.hash);
        }
    }
    Hashlock { hash: hash.try_into().unwrap() }.compile()
}

// Helpers: hash160, sha256, check_sig as macros or fns mapping to ops.
// Add more templates: Token, Game, etc.
