# nPrint: Rust Framework for BSV Scripts 🛠️

[![CI](https://github.com/murphsicles/nPrint/actions/workflows/ci.yml/badge.svg)](https://github.com/murphsicles/nPrint/actions/workflows/ci.yml)
[![Rust Version](https://img.shields.io/badge/rust-1.81.0%2B-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/nprint-core)](https://crates.io/crates/nprint-core)

nPrint is a modern Rust framework for building, deploying, verifying, and interacting with smart contracts on the Bitcoin Script Virtual Machine (BSV). It supports a wide range of script types, media protocols, and asynchronous runtime operations, with a focus on modularity and efficiency.

## Features ✨
nPrint provides a powerful macro system and reusable templates for Bitcoin SV smart contracts, enabling flexible and efficient development. Key features include:

- **Macro-based script generation**: Compile-time script unrolling for performance and safety.
- **Reusable templates**: Predefined contracts for common use cases.
- **Media protocols**: On-chain verification and off-chain streaming for images, videos, audio, and documents.
- **Asynchronous runtime**: Deploy and call contracts with node integration.
- **Verification tools**: Symbolic execution for stack safety.
- **CLI tool**: User-friendly interface for contract management.

### Macro Logic 🧠
nPrint implements the following macros for efficient script construction:

- **bsv_script!**: Creates a script from opcodes and integers.
  - Example:
    ```rust
    use nprint_core::bsv_script;
    use sv::script::op_codes::OP_DUP;

    let script = bsv_script! { OP_DUP, 1 };
    ```
- **xswap!(n)**: Swaps the top stack item with the (n-1)th item below it. Expands to `[<n-1>, OP_ROLL]`.
  - Example: `xswap!(3)` expands to `[2, OP_ROLL]`.
- **xdrop!(n)**: Removes the (n-1)th item from the stack. Expands to `[<n-1>, OP_ROLL, OP_DROP]`.
  - Example: `xdrop!(3)` expands to `[2, OP_ROLL, OP_DROP]`.
- **xrot!(n)**: Rotates the nth item to the top. Expands to `[<n>, OP_ROLL]`.
  - Example: `xrot!(3)` expands to `[3, OP_ROLL]`.
- **hashcat!()**: Duplicates the top item, applies HASH160, and concatenates. Expands to `[OP_DUP, OP_HASH160, OP_CAT]`.
- **loop_unroll!(n, {body})**: Unrolls a script body n times statically.
  - Example:
    ```rust
    use nprint_core::loop_unroll;
    use sv::script::op_codes::OP_DUP;

    let script = loop_unroll!(2, { OP_DUP });
    ```

These macros are hygienically expanded at compile-time, supporting BSV’s restored opcodes (e.g., OP_CAT) and verified via symbolic execution for stack safety.

**Example Contract**:
```rust
use nprint_core::{bsv_script, Stack};
use sv::script::op_codes::{OP_DUP, OP_HASH160, OP_EQUAL};

fn create_hash_puzzle(hash: [u8; 32]) -> Vec<u8> {
    let mut script = bsv_script! { OP_HASH160 };
    script.extend_from_slice(&[hash.len() as u8]);
    script.extend_from_slice(&hash);
    script.extend(bsv_script! { OP_EQUAL });
    script
}

fn verify_hash_puzzle(data: Vec<u8>, hash: [u8; 32]) {
    let script = create_hash_puzzle(hash);
    let mut stack = Stack::default();
    stack.push(data);
    stack.execute(&script).unwrap();
    assert_eq!(stack.pop(), vec![1]); // Success indicator
}
```

### Supported Script Types 📜
nPrint supports a variety of BSV script templates via the `templates` crate:
- **Payments**: P2PKH, Multisig, Timelock.
- **Puzzles/Locks**: Hashlock, Rabin Signature.
- **Tokens/Assets**: BSV-20/21 fungible tokens, Ordinals (NFTs).
- **Advanced**: SHA-Gate, DriveChain, MAST.

### Media Protocols 🎥
The `protocols` crate enables on-chain verification and off-chain streaming for:
- **Image**: Hash-verified processing (e.g., PNG/JPEG).
- **Documents**: Chunked verification for PDFs.
- **Music**: Audio streaming with sample hashing, supporting all popular audio containers and codecs.
- **Video**: Merkle-tree-based chunked streaming. Supports all modern video fontainers and codecs.

## Crates 📦
- **core**: Opcodes, macros, stack simulation.
- **dsl**: Procedural macros for contract derivation.
- **templates**: Reusable smart contract templates.
- **protocols**: Media processing and verification.
- **runtime**: Asynchronous contract deployment and execution.
- **verification**: Script and macro verification.
- **cli**: Command-line tool for contract management.

## Installation 🔧
Clone the repository:
```bash
git clone https://github.com/murphsicles/nPrint.git
cd nPrint
```
Build the project:
```bash
cargo build
```
Install the CLI:
```bash
cargo install --path cli
```

## Usage 🚀
The `nprint` CLI provides commands for compiling, deploying, and interacting with contracts:
```bash
# Deploy a contract
nprint deploy p2pkh --params pkh=1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b --node http://node.example.com

# Call a contract method
nprint call hashlock unlock --args data=secret_message --utxo txid123 --node http://node.example.com

# Stream media
nprint stream image image.png hash=1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2
```

For detailed usage, see [DOCUMENTATION.md](DOCUMENTATION.md).

## Contributing 🤝
Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details on code style, testing, and submitting pull requests.

## Publishing to crates.io 📤
To publish a crate:
1. Update the version in `Cargo.toml`.
2. Run `cargo publish --allow-dirty` for each crate, starting with dependencies (`core`, `types`, etc.).

## Optimization ⚡
- Run `cargo bench` in the `core` crate to measure performance.
- The `core` crate is `no_std` compatible for embedded environments.

## License
MIT
