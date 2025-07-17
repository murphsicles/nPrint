# nPrint: A Bitcoin Script Smart Contract Framework in Rust

nPrint is a modular Rust framework for building, deploying, verifying, and interacting with smart contracts on a Bitcoin Script Virtual Machine (BSV). It provides tools for script generation, macro expansion, stack simulation, template-based contract creation, runtime deployment, verification, media protocols, and a CLI for user interaction. The framework is divided into several crates for separation of concerns.

This documentation explains the purpose of each crate, key types, functions, and methods. It is based on best practices for Rust crate documentation, including front-page summaries, core concepts, and usage examples.

## Crate Overview

- **core**: Core Bitcoin Script logic, including script macros, stack model, and execution.
- **dsl**: Procedural macros for deriving smart contract traits.
- **types**: Shared types for artifacts, Sha256, and ToScript trait.
- **cli**: Command-line interface for deploying, calling, and streaming contracts.
- **runtime**: Runtime for deploying and calling contracts, with provider and signer traits.
- **verification**: Tools for verifying scripts and macros.
- **protocols**: Media protocols for image, video, audio, and doc verification and streaming.
- **templates**: Predefined smart contract templates (e.g., P2PKH, Multisig).

## Installation

Add the crates to your `Cargo.toml` as needed, e.g.:

```toml
[dependencies]
nprint-core = { path = "core" }  # Replace with published version if available
nprint-dsl = { path = "dsl" }
# Add other crates similarly
```

## Usage

See each crate section for detailed usage.

### Core

The `core` crate provides fundamental tools for Bitcoin Script manipulation and execution.

#### Key Types

- **Stack**: Simulates the Bitcoin Script stack with main and alt stacks.
  - Methods:
    - `push(&mut self, value: Vec<u8>)`: Pushes a value onto the main stack.
    - `pop(&mut self) -> Vec<u8>`: Pops a value from the main stack.
    - `execute(&mut self, script: &[u8]) -> Result<(), String>`: Executes the script on the stack, handling opcodes like OP_DUP, OP_SWAP, OP_PICK, OP_ROLL, OP_DROP, and push operations.

- **MacroElem**: Enum for macro elements (Op(u8) or Param(usize)).
- **MacroDef**: Struct for macro definitions, with name, param_count, and template (Vec<MacroElem>).

#### Key Functions

- **bsv_script!**: Macro to create a script from opcodes and integers.
  - Example:
    ```rust
    use nprint_core::bsv_script;
    use sv::script::op_codes::OP_DUP;

    let script = bsv_script! { OP_DUP, 1 };
    ```

- **expand_macro(def: &MacroDef, args: &[i32]) -> Vec<u8>**: Expands a macro with arguments into a script.

- **loop_unroll!**: Macro to unroll a script body a specified number of times.
  - Example:
    ```rust
    use nprint_core::loop_unroll;
    use sv::script::op_codes::OP_DUP;

    let script = loop_unroll!(3, { OP_DUP });
    ```

- **xswap!**, **xdrop!**, **xrot!**, **hashcat!**: Macros for common script operations.

- **parse_script(input: &[u8]) -> IResult<&[u8], Vec<u8>>**: Stub for parsing scripts (returns empty vec).

#### Examples
See `core/examples/hash_puzzle.rs` and `core/examples/composite.rs` for script creation examples.

### DSL

The `dsl` crate provides procedural macros for deriving traits.

#### Key Macros

- **#[derive(SmartContract)]**: Derives the `SmartContract` trait for structs, implementing `compile` to generate an `Artifact` from fields.

### Types

The `types` crate defines shared types for the framework.

#### Key Types

- **Artifact**: Struct with script (Vec<u8>) and props (Vec<String>).
- **SmartContract**: Trait for compiling to Artifact.
  - Methods:
    - `compile(&self) -> Artifact`: Compiles the contract to an Artifact.

- **ToScript**: Trait for converting types to script bytes.
  - Methods:
    - `to_script(&self) -> Vec<u8>`: Converts the type to a script push.

- **Sha256**: Struct for SHA256 hashes ([u8; 32]).

Implementations of `ToScript` are provided for Sha256, i32, i64, i128, usize, u8, Vec<u8>, and [u8; 20].

### CLI

The `cli` crate provides a command-line interface for interacting with contracts.

#### Key Commands

- **deploy**: Deploys a contract using a template and parameters.
  - Usage: `cargo run -- deploy <template> [params]`
- **call**: Calls a contract method with arguments and UTXO.
  - Usage: `cargo run -- call <template> <method> [args] <utxo>`
- **stream**: Streams media using a protocol, file, and hash.
  - Usage: `cargo run -- stream <protocol> <file> <hash>`

#### Key Types

- **CliError**: Enum for CLI errors (TemplateNotFound, Runtime).

The CLI uses dummy signer and contract for demonstration.

### Runtime

The `runtime` crate handles contract deployment and execution.

#### Key Types

- **RuntimeError**: Enum for runtime errors (Network, Script, Wallet).
- **Provider**: Struct for node interaction.
  - Methods:
    - `new(node: &str) -> Self`: Creates a provider.
    - `broadcast(&self, _tx: Transaction) -> Result<String, RuntimeError>`: Broadcasts a transaction (stub).
    - `get_utxo(&self, _addr: String) -> Result<OutPoint, RuntimeError>`: Gets a UTXO (stub).

- **Signer**: Trait for signing transactions.
  - Methods:
    - `sign(&self, tx: &mut Transaction) -> Result<(), RuntimeError>`: Signs a transaction.

#### Key Functions

- **deploy(contract: impl SmartContract, signer: impl Signer, provider: Provider) -> Result<String, RuntimeError>**: Deploys a contract and returns the txid.
- **call(contract: impl SmartContract, _method: &str, _args: Vec<Vec<u8>>, _utxo: String, signer: impl Signer, provider: Provider) -> Result<String, RuntimeError>**: Calls a contract method and returns the txid.
- **stream_media(proto: impl MediaProtocol + Send + 'static, mut source: impl AsyncRead + Unpin + Send + 'static) -> JoinHandle<Result<(), RuntimeError>>**: Streams media using a protocol.

### Verification

The `verification` crate provides verification tools for scripts and macros.

#### Key Types

- **VerifyError**: Enum for verification errors (Underflow, InvalidOp, Failed).

#### Key Functions

- **verify_macro(def: &MacroDef, args: &[i32], inputs: Vec<Vec<u8>>) -> Result<(), VerifyError>**: Verifies a macro expansion on a stack with inputs.
- **verify_script(script: &[u8], inputs: Vec<Vec<u8>>) -> Result<bool, VerifyError>**: Verifies a script on a stack with inputs, returning true if verification succeeds.

### Protocols

The `protocols` crate defines media protocols for verification and streaming.

#### Key Traits

- **MediaProtocol**: Trait for media verification.
  - Methods:
    - `verify(&self, data: Vec<u8>, hash: Sha256) -> bool`: Verifies data against a hash.
    - `get_hash(&self) -> Sha256`: Returns the hash.

#### Key Types

- **ImageProtocol**: Protocol for images.
  - Methods:
    - `verify_image(&self, data: Vec<u8>)`: Asserts data hash matches.
    - `stream_image(&self, _file: impl AsyncRead + Unpin) -> impl Stream<Item = ImageBuffer<Rgba<u8>, Vec<u8>>>`: Streams image (stub).
- **VideoProtocol**: Protocol for videos.
  - Methods:
    - `verify_video(&self, data: Vec<u8>)`: Asserts data hash matches.
    - `stream_video(&self, _file: impl AsyncRead + Unpin) -> impl Stream<Item = Bytes>`: Streams video (stub).
- **AudioProtocol**: Protocol for audio.
  - Methods:
    - `verify_audio(&self, data: Vec<u8>)`: Asserts data hash matches.
    - `stream_audio(&self, _file: impl AsyncRead + Unpin) -> impl Stream<Item = AudioBuffer<f32>>`: Streams audio (stub).
- **DocProtocol**: Protocol for documents.
  - Methods:
    - `verify_doc(&self, data: Vec<u8>)`: Asserts data hash matches.
    - `stream_doc(&self, _file: impl AsyncRead + Unpin) -> impl Stream<Item = String>`: Streams doc (stub).

### Templates

The `templates` crate provides predefined smart contract templates.

#### Key Types

- **P2PKH**: Pay-to-Public-Key-Hash contract.
  - Fields: `pkh: [u8; 20]`
  - Implements `SmartContract`.
- **Multisig**: Multi-signature contract.
  - Fields: `pubkeys: Vec<Vec<u8>>`, `m: usize`
  - Implements `SmartContract`.
- **Timelock**: Time-locked contract.
  - Fields: `timeout: i128`
  - Implements `SmartContract`.
- **Hashlock**: Hash-locked contract.
  - Fields: `hash: Sha256`
  - Implements `SmartContract`.
  - Methods:
    - `unlock(&self, _msg: Vec<u8>)`: Asserts unlock (stub).
- **RabinSig**: Rabin signature contract.
  - Fields: `rabin_pk: i128`
  - Implements `SmartContract`.
- **Token**: Token contract.
  - Fields: `tick: Vec<u8>`, `max: i128`, `data: Vec<u8>`
  - Implements `SmartContract`.
- **NFT**: NFT contract.
  - Fields: `id: Vec<u8>`
  - Implements `SmartContract`.
- **LoopUnroll**: Loop unroll contract.
  - Fields: `count: i128`
  - Implements `SmartContract`.
- **SHAGate**: SHA gate contract.
  - Fields: `hash: Sha256`
  - Implements `SmartContract`.
  - Methods:
    - `unlock(&self, input: Vec<u8>)`: Asserts unlock with SHA gate.
- **DriveChain**: Drive chain contract.
  - Fields: `peg_hash: Sha256`
  - Implements `SmartContract`.
- **MAST**: Merkle Abstract Syntax Tree contract.
  - Fields: `root: Sha256`
  - Implements `SmartContract`.
  - Methods:
    - `execute_branch(&self, branch: Vec<u8>, proof: Vec<u8>)`: Asserts branch execution (stub).

#### Key Functions

- **compute_sha_gate(input: &Vec<u8>) -> Sha256**: Computes SHA256 gate.
- **merkle_proof(_branch: &[u8], _proof: &[u8]) -> Sha256**: Stub for Merkle proof.
- **REGISTRY**: Lazy static HashMap of template names to functions that generate Artifacts from parameters.

- Template functions (e.g., `p2pkh(params: HashMap<String, Vec<u8>>) -> Artifact`): Create contracts from parameters.

### Additional Notes

- Use `cargo doc --open` to generate and view Rustdoc documentation.
- For contributing, see `CONTRIBUTING.md`.
- CI workflows in `.github/workflows` ensure building, testing, and releasing.
