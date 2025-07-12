use nprint_types::{SmartContract, Artifact, ToScript, FixedArray, PubKey, Sig, Sha256};
use sha2::{Digest, Sha256 as Sha256Digest};
use std::collections::HashMap;

/// Template fn type.
pub type Template = fn(&HashMap<String, Vec<u8>>) -> Artifact;

// Placeholder functions
fn check_sig(_sig: Sig, _pk: PubKey) -> bool { true } // Stub
fn sha256(msg: &Vec<u8>) -> Sha256 { Sha256Digest::digest(msg).into() } // Stub
fn compute_sha_gate(input: &Vec<u8>) -> Sha256 { Sha256Digest::digest(input).into() } // Stub
fn merkle_proof(_branch: &Vec<u8>, _proof: &Vec<u8>) -> Sha256 { [0; 32] } // Stub
struct Ctx { sequence: i128 } // Stub
static ctx: Ctx = Ctx { sequence: 0 }; // Stub

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
    struct P2PKH { pkh: [u8; 20], }
    impl P2PKH {
        pub fn unlock(&self, _sig: Sig, pk: PubKey) { assert_eq!(Sha256Digest::digest(&pk)[12..].try_into::<[u8; 20]>().unwrap(), self.pkh); assert!(true); }
    }
    impl SmartContract for P2PKH {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.pkh.to_script());
            Artifact { script, props: vec!["pkh".to_string()] }
        }
    }
    P2PKH { pkh: pkh.try_into().unwrap() }.compile()
}

fn multisig(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let pubkeys = FixedArray::new([vec![], vec![], vec![]]); // Stub for compilation
    let m = params["m"][0] as usize;
    struct Multisig { pubkeys: FixedArray<PubKey, 3>, m: usize, }
    impl Multisig {
        pub fn unlock(&self, _sigs: FixedArray<Sig, 2>) { /* check m sigs */ }
    }
    impl SmartContract for Multisig {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.pubkeys.to_script());
            script.extend(self.m.to_script());
            Artifact { script, props: vec!["pubkeys".to_string(), "m".to_string()] }
        }
    }
    Multisig { pubkeys, m }.compile()
}

fn timelock(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let timeout = params["timeout"].clone();
    struct Timelock { timeout: i128, }
    impl Timelock {
        pub fn unlock(&self) { assert!(ctx.sequence > self.timeout); }
    }
    impl SmartContract for Timelock {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.timeout.to_script());
            Artifact { script, props: vec!["timeout".to_string()] }
        }
    }
    Timelock { timeout: i128::from_le_bytes(timeout.try_into().unwrap()) }.compile()
}

fn hashlock(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let hash = params["hash"].clone();
    struct Hashlock { hash: Sha256, }
    impl Hashlock {
        pub fn unlock(&self, _msg: Vec<u8>) { assert_eq!([0; 32], self.hash); }
    }
    impl SmartContract for Hashlock {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.hash.to_script());
            Artifact { script, props: vec!["hash".to_string()] }
        }
    }
    Hashlock { hash: hash.try_into().unwrap() }.compile()
}

fn rabin_sig(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let rabin_pk = params["rabin_pk"].clone();
    struct RabinSig { rabin_pk: i128, }
    impl RabinSig {
        pub fn unlock(&self, _msg: Vec<u8>, _sig: Vec<u8>) { /* verify rabin */ }
    }
    impl SmartContract for RabinSig {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.rabin_pk.to_script());
            Artifact { script, props: vec!["rabin_pk".to_string()] }
        }
    }
    RabinSig { rabin_pk: i128::from_le_bytes(rabin_pk.try_into().unwrap()) }.compile()
}

fn coin_toss(_params: &HashMap<String, Vec<u8>>) -> Artifact {
    struct CoinToss {}
    impl CoinToss {
        pub fn toss(&self, _commit1: Vec<u8>, _commit2: Vec<u8>) { /* hash compare */ }
    }
    impl SmartContract for CoinToss {
        fn compile(&self) -> Artifact {
            let script = Vec::new();
            Artifact { script, props: vec![] }
        }
    }
    CoinToss {}.compile()
}

fn bsv20_token(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let tick = params["tick"].clone();
    let max = i128::from_le_bytes(params["max"].clone().try_into().unwrap());
    struct BSV20 { tick: Vec<u8>, max: i128, }
    impl BSV20 {
        pub fn transfer(&self, _to: PubKey, _amt: i128) { /* token logic */ }
    }
    impl SmartContract for BSV20 {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.tick.to_script());
            script.extend(self.max.to_script());
            Artifact { script, props: vec!["tick".to_string(), "max".to_string()] }
        }
    }
    BSV20 { tick, max }.compile()
}

fn ordinals(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let data = params["data"].clone();
    struct Ordinals { data: Vec<u8>, }
    impl Ordinals {
        pub fn inscribe(&self) { assert!(true); }
    }
    impl SmartContract for Ordinals {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.data.to_script());
            Artifact { script, props: vec!["data".to_string()] }
        }
    }
    Ordinals { data }.compile()
}

fn tic_tac_toe(_params: &HashMap<String, Vec<u8>>) -> Artifact {
    struct TicTacToe { board: FixedArray<i128, 9>, }
    impl TicTacToe {
        pub fn move_pos(&self, _pos: i128, _player: PubKey) { /* update, check win with unrolled loop */ }
    }
    impl SmartContract for TicTacToe {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.board.to_script());
            Artifact { script, props: vec!["board".to_string()] }
        }
    }
    TicTacToe { board: FixedArray::new([0; 9]) }.compile()
}

fn battleship(_params: &HashMap<String, Vec<u8>>) -> Artifact {
    struct Battleship {}
    impl Battleship {
        pub fn place(&self, _proof: Vec<u8>) { /* zk verify */ }
    }
    impl SmartContract for Battleship {
        fn compile(&self) -> Artifact {
            let script = Vec::new();
            Artifact { script, props: vec![] }
        }
    }
    Battleship {}.compile()
}

fn oracle(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let oracle_pk = params["oracle_pk"].clone();
    struct Oracle { oracle_pk: PubKey, }
    impl Oracle {
        pub fn use_data(&self, _data: Vec<u8>, sig: Sig) { assert!(check_sig(sig, self.oracle_pk.clone())); }
    }
    impl SmartContract for Oracle {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.oracle_pk.to_script());
            Artifact { script, props: vec!["oracle_pk".to_string()] }
        }
    }
    Oracle { oracle_pk }.compile()
}

fn counter(_params: &HashMap<String, Vec<u8>>) -> Artifact {
    struct Counter { count: i128, }
    impl Counter {
        pub fn increment(&mut self) { self.count += 1; }
    }
    impl SmartContract for Counter {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.count.to_script());
            Artifact { script, props: vec!["count".to_string()] }
        }
    }
    Counter { count: 0 }.compile()
}

fn sha_gate(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let hash = params["hash"].clone();
    struct SHAGate { hash: Sha256, }
    impl SHAGate {
        pub fn unlock(&self, input: Vec<u8>) { assert_eq!(compute_sha_gate(&input), self.hash); }
    }
    impl SmartContract for SHAGate {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.hash.to_script());
            Artifact { script, props: vec!["hash".to_string()] }
        }
    }
    SHAGate { hash: hash.try_into().unwrap() }.compile()
}

fn drive_chain(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let peg_hash = params["peg_hash"].clone();
    struct DriveChain { peg_hash: Sha256, }
    impl DriveChain {
        pub fn verify_peg(&self, _proof: Vec<u8>) { /* cross-chain */ }
    }
    impl SmartContract for DriveChain {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.peg_hash.to_script());
            Artifact { script, props: vec!["peg_hash".to_string()] }
        }
    }
    DriveChain { peg_hash: peg_hash.try_into().unwrap() }.compile()
}

fn mast(params: &HashMap<String, Vec<u8>>) -> Artifact {
    let root = params["root"].clone();
    struct MAST { root: Sha256, }
    impl MAST {
        pub fn execute_branch(&self, branch: Vec<u8>, proof: Vec<u8>) { assert_eq!(merkle_proof(&branch, &proof), self.root); }
    }
    impl SmartContract for MAST {
        fn compile(&self) -> Artifact {
            let mut script = Vec::new();
            script.extend(self.root.to_script());
            Artifact { script, props: vec!["root".to_string()] }
        }
    }
    MAST { root: root.try_into().unwrap() }.compile()
}
