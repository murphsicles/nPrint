# nPrint: Rust Framework for BSV Scripts 🛠️

nPrint is a modern Rust DSL for BSV smart contracts. Supports all script types, media protocols, async runtime.

## Crates 📦
- **core**: Opcodes, macros, stack.
- **dsl**: Proc macros for contracts.
- **templates**: Reusable scripts.
- **protocols**: Media processing.
- **runtime**: Deploy/call async.
- **verification**: Proofs/sim.
- **cli**: `nprint` bin tool.

## Usage 🚀
`cargo install --path cli`
`nprint compile src.rs --output artifact.json`
`nprint deploy artifact.json --key <wif> --node <rpc>`

## Optimization ⚡
- Use `cargo bench` for perf.
- no_std in core.

License: MIT
