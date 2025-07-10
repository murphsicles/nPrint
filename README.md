# nPrint: Rust Framework for BSV Scripts 🛠️

[![CI](https://github.com/murphsicles/nPrint/actions/workflows/ci.yml/badge.svg)](https://github.com/murphsicles/nPrint/actions/workflows/ci.yml)

nPrint is a modern Rust DSL for BSV smart contracts. Supports all script types, media protocols, async runtime.

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
