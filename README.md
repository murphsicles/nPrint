# nPrint: Rust Framework for BSV Scripts 🛠️

[![CI](https://github.com/murphsicles/nPrint/actions/workflows/ci.yml/badge.svg)](https://github.com/murphsicles/nPrint/actions/workflows/ci.yml)

nPrint is a modern Rust DSL for BSV smart contracts. Supports all script types, media protocols, async runtime.

## Features ✨
nPrint leverages a powerful macro expansion system and supports a wide range of Bitcoin SV script types, enabling flexible and efficient smart contract development.

### Macro Logic 🧠
nPrint implements the following macros for compile-time script unrolling, ensuring efficiency and verifiability:
- **OP_XSWAP_n**: Swaps the top stack item with the nth item below it. Example: `xswap!(3)` expands to `[<2>, OP_PICK, <2>, OP_ROLL, OP_SWAP, OP_DROP]`.
- **OP_XDROP_n**: Removes the nth item from the stack. Example: `xdrop!(3)` expands to `[<2>, OP_ROLL, OP_DROP]`.
- **OP_XROT_n**: Rotates the nth item to the top. Example: `xrot!(3)` expands to `[<3>, OP_ROLL]`.
- **OP_HASHCAT**: Duplicates the top item, applies HASH160 to one, and concatenates with OP_CAT. Example: `hashcat!()` expands to `[OP_DUP, OP_HASH160, OP_CAT]`.
- **LOOP[n]{body}**: Unrolls a script body n times statically. Example: `loop_unroll!(2, { OP_DUP })` expands to `[OP_DUP, OP_DUP]`.

These macros are hygienically expanded at compile-time, supporting BSV’s restored opcodes (e.g., OP_CAT) and verified via symbolic execution for stack safety.

**Example Composite Contract**:
```rust
#[contract]
struct Composite {
    #[prop]
    hash: [u8; 32],
}
impl Composite {
    #[method]
    pub fn unlock(&self, data: Vec<u8>, n: i32) {
        let script = bsv_script! {
            { loop_unroll!(2, { OP_DUP }) }  // Duplicate twice
            { xswap!(n) }                    // Swap with nth
            { hashcat!() }                   // Hash and concat
            OP_EQUALVERIFY                   // Verify against prop
        };
        let mut stack = Stack::default();
        stack.push(data);
        stack.execute(&script).unwrap();
        assert_eq!(stack.pop(), self.hash);
    }
}
```
### Supported Script Types 📜
nPrint supports all major BSV script categories via reusable templates:
- **Payments**: P2PKH, Multisig, Timelock.
- **Puzzles/Locks**: Hashlock, Rabin Signature, Coin Toss.
- **Tokens/Assets**: BSV-20/21 fungible tokens, Ordinals.
- **Games**: Tic-Tac-Toe, zkSNARK-based Battleship.
- **Oracles/Advanced**: ECDSA-based Oracle, Counter, SHA-Gate, DriveChain, MAST.

### Media Protocols 🎥
Process and verify images, documents, music, and video streams on-chain with async off-chain handling:
- **Image**: Hash-verified processing (e.g., PNG/JPEG).
- **Documents**: Chunked verification for PDFs.
- **Music**: WAV streaming with sample hashing.
- **Video**: Merkle-tree-based chunked streaming.

## Crates 📦
- **core**: Opcodes, macros, stack.
- **dsl**: Proc macros for contracts.
- **templates**: Reusable scripts.
- **protocols**: Media processing.
- **runtime**: Deploy/call async.
- **verification**: Proofs/sim.
- **cli**: `nprint` bin tool.

## Installation 🔧
Clone the repo: `git clone https://github.com/murphsicles/nPrint.git`
Build: `cargo build`
Install CLI: `cargo install --path cli`

## Usage 🚀
`nprint compile src.rs --output artifact.json`
`nprint deploy artifact.json --key <wif> --node <rpc>`

## Contributing 🤝
See [CONTRIBUTING.md](CONTRIBUTING.md) for details on code style, testing, and submitting pull requests.

## Publishing to crates.io 📤
For each crate: Update version in Cargo.toml, then `cargo publish --allow-dirty` (repeat dependencies first).

## Optimization ⚡
- Use `cargo bench` for perf.
- no_std in core.

License: MIT
