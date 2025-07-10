use nprint_dsl::{contract, prop, method, SmartContract, Artifact};
use nprint_core::{bsv_script, FixedArray, PubKey, Sig, Sha256};  // Assume core extensions for types
use std::collections::HashMap;

/// Template fn type.
pub type Template = fn(&HashMap<String, Vec<u8>>) -> Artifact;

/// Lazy registry with all script types.
pub static REGISTRY: std::sync::LazyLock<HashMap<String, Template>> = std::sync::LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("P2PKH".to_string(), p2pkh as Template);
    map.insert("Multisig".to_string(), multisig as Template);
    map.insert("Timelock".to_string(), timelock as Template);
    map.insert("Hashlock".to_string(), hashlock as Template);
    map.insert("RabinSig".to_string(), rabin_sig as Template);
    map.insert("CoinToss".to_string(), coin_toss as Template);
    map.insert("BSV20Token".to_string(), bsv20_token as Template);
    map.insert("Ordinals".to_string(), ordinals as Template);
    map.insert("TicTacToe".to_string(), tic_tac_toe as Template);
    map.insert("Battleship".to_string(), battleship as Template);
    map.insert("Oracle".to_string(), oracle as Template);
    map.insert("Counter".to_string(), counter as Template);
    map.insert("SHAGate".to_string(), sha_gate as Template);
    map.insert("DriveChain".to_string(), drive_chain as Template);
    map.insert("MAST".to_string(), mast as Template);
    map
});

fn p2pkh(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let pkh = params["pkh"].clone();
    #[contract]
    struct P2PKH { #[prop] pkh: [u8; 20]; }
    impl P2PKH {
        #[method]
        pub fn unlock(&self, sig: Sig, pk: PubKey) { assert_eq!(hash160(&pk), self.pkh); assert!(check_sig(sig, pk)); }
    }
    P2PKH { pkh: pkh.try_into().unwrap() }.compile()
}

fn multisig(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let pubkeys: FixedArray<PubKey, 3> = params["pubkeys"].clone().try_into().unwrap();
    let m = params["m"][0] as usize;
    #[contract]
    struct Multisig { #[prop] pubkeys: FixedArray<PubKey, 3>; #[prop] m: usize; }
    impl Multisig {
        #[method]
        pub fn unlock(&self, sigs: FixedArray<Sig, 2>) { /* check m sigs */ }
    }
    Multisig { pubkeys, m }.compile()
}

fn timelock(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let timeout = params["timeout"].clone();
    #[contract]
    struct Timelock { #[prop] timeout: i128; }
    impl Timelock {
        #[method]
        pub fn unlock(&self) { assert!(ctx.sequence > self.timeout); }
    }
    Timelock { timeout: i128::from_le_bytes(timeout.try_into().unwrap()) }.compile()
}

fn hashlock(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let hash = params["hash"].clone();
    #[contract]
    struct Hashlock { #[prop] hash: Sha256; }
    impl Hashlock {
        #[method]
        pub fn unlock(&self, msg: Vec<u8>) { assert_eq!(sha256(&msg), self.hash); }
    }
    Hashlock { hash: hash.try_into().unwrap() }.compile()
}

fn rabin_sig(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let rabin_pk = params["rabin_pk"].clone();
    #[contract]
    struct RabinSig { #[prop] rabin_pk: i128; }
    impl RabinSig {
        #[method]
        pub fn unlock(&self, msg: Vec<u8>, sig: Vec<u8>) { /* verify rabin */ }
    }
    RabinSig { rabin_pk: i128::from_le_bytes(rabin_pk.try_into().unwrap()) }.compile()
}

fn coin_toss(params: &HashMap<String, Vec<u8>>) -> Artifact {
    #[contract]
    struct CoinToss {}
    impl CoinToss {
        #[method]
        pub fn toss(&self, commit1: Vec<u8>, commit2: Vec<u8>) { /* hash compare */ }
    }
    CoinToss {}.compile()
}

fn bsv20_token(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let tick = params["tick"].clone();
    let max = i128::from_le_bytes(params["max"].clone().try_into().unwrap());
    #[contract]
    struct BSV20 { #[prop] tick: Vec<u8>; #[prop] max: i128; }
    impl BSV20 {
        #[method]
        pub fn transfer(&self, to: PubKey, amt: i128) { /* token logic */ }
    }
    BSV20 { tick, max }.compile()
}

fn ordinals(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let data = params["data"].clone();
    #[contract]
    struct Ordinals { #[prop] data: Vec<u8>; }
    impl Ordinals {
        #[method]
        pub fn inscribe(&self) { assert!(true); }  // Inscription via UTXO
    }
    Ordinals { data }.compile()
}

fn tic_tac_toe(params: &HashMap<String, Vec<u8>>) -> Artifact {
    #[contract]
    struct TicTacToe { #[prop(mutable = true)] board: FixedArray<i128, 9>; }
    impl TicTacToe {
        #[method]
        pub fn move_pos(&self, pos: i128, player: PubKey) { /* update, check win with unrolled loop */ }
    }
    TicTacToe { board: [0; 9] }.compile()
}

fn battleship(params: &HashMap<String, Vec<u8>>) -> Artifact {
    #[contract]
    struct Battleship {}
    impl Battleship {
        #[method]
        pub fn place(&self, proof: Vec<u8>) { /* zk verify */ }
    }
    Battleship {}.compile()
}

fn oracle(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let oracle_pk = params["oracle_pk"].clone();
    #[contract]
    struct Oracle { #[prop] oracle_pk: PubKey; }
    impl Oracle {
        #[method]
        pub fn use_data(&self, data: Vec<u8>, sig: Sig) { assert!(check_sig(sig, self.oracle_pk)); }
    }
    Oracle { oracle_pk }.compile()
}

fn counter(params: &HashMap<String, Vec<u8>>) -> Artifact {
    #[contract]
    struct Counter { #[prop(mutable = true)] count: i128; }
    impl Counter {
        #[method]
        pub fn increment(&self) { self.count += 1; }
    }
    Counter { count: 0 }.compile()
}

fn sha_gate(params: &HashMap<String, Vec<u8>>) -> Artifact {
    #[contract]
    struct SHAGate { #[prop] hash: Sha256; }
    impl SHAGate {
        #[method]
        pub fn unlock(&self, input: Vec<u8>) { assert_eq!(sha_gate(&input), self.hash); }
    }
    SHAGate { hash: params["hash"].clone().try_into().unwrap() }.compile()
}

fn drive_chain(params: &HashMap<String, Vec<u8>>) -> Artifact {
    #[contract]
    struct DriveChain { #[prop] peg_hash: Sha256; }
    impl DriveChain {
        #[method]
        pub fn verify_peg(&self, proof: Vec<u8>) { /* cross-chain */ }
    }
    DriveChain { peg_hash: params["peg_hash"].clone().try_into().unwrap() }.compile()
}

fn mast(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let root = params["root"].clone();
    #[contract]
    struct MAST { #[prop] root: Sha256; }
    impl MAST {
        #[method]
        pub fn execute_branch(&self, branch: Vec<u8>, proof: Vec<u8>) { assert_eq!(merkle_proof(&branch, &proof), self.root); }
    }
    MAST { root: root.try_into().unwrap() }.compile()
}
