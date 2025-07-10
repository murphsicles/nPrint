use clap::{Parser, Subcommand};
use nprint_runtime::{deploy, call, Provider, Signer, PrivateKey};
use nprint_verification::{verify_script};
use nprint_templates::REGISTRY;
use nprint_protocols::{ImageProtocol, MediaProcessor};
use nprint_dsl::Artifact;
use std::path::PathBuf;
use tokio::fs::File;
use thiserror::Error;

#[derive(Error, Debug)]
enum CliError {
    #[error("Invalid command: {0}")]
    Invalid(String),
    #[error("Runtime: {0}")]
    Runtime(nprint_runtime::RuntimeError),
}

#[derive(Parser)]
#[command(name = "nprint", about = "nPrint CLI: Build, deploy, verify BSV scripts")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Compile { file: PathBuf },  // Compile Rust to artifact
    Deploy { artifact: PathBuf, key: String, node: String },
    Call { artifact: PathBuf, method: String, args: Vec<String>, utxo: String, key: String, node: String },
    Verify { script: PathBuf, inputs: Vec<String> },
    Template { name: String, params: Vec<String> },  // Generate from template
    Stream { media_type: String, file: PathBuf, hash: String },  // Media example
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    match cli.command {
        Command::Compile { file } => {
            // Stub: Load Rust file, compile via dsl (in practice, use build script)
            let artifact = Artifact { script: vec![], props: vec![] };
            println!("{}", serde_json::to_string(&artifact).unwrap());
            Ok(())
        }
        Command::Deploy { artifact, key, node } => {
            let art: Artifact = serde_json::from_str(&std::fs::read_to_string(artifact)?).unwrap();
            let privkey = PrivateKey::from_wif(&key).unwrap();
            let provider = Provider::new(&node);
            let txid = deploy(/* contract from art */, privkey, provider).await.map_err(CliError::Runtime)?;
            println!("Deployed: {}", txid);
            Ok(())
        }
        Command::Call { artifact, method, args, utxo, key, node } => {
            let art: Artifact = serde_json::from_str(&std::fs::read_to_string(artifact)?).unwrap();
            let privkey = PrivateKey::from_wif(&key).unwrap();
            let provider = Provider::new(&node);
            let arg_bytes: Vec<Vec<u8>> = args.iter().map(|s| s.as_bytes().to_vec()).collect();
            let txid = call(/* contract */, &method, arg_bytes, utxo, privkey, provider).await.map_err(CliError::Runtime)?;
            println!("Called: {}", txid);
            Ok(())
        }
        Command::Verify { script, inputs } => {
            let script_bytes = std::fs::read(script)?;
            let input_bytes: Vec<Vec<u8>> = inputs.iter().map(|s| s.as_bytes().to_vec()).collect();
            let valid = verify_script(&script_bytes, input_bytes).map_err(|e| CliError::Invalid(e.to_string()))?;
            println!("Valid: {}", valid);
            Ok(())
        }
        Command::Template { name, params } => {
            if let Some(tmpl) = REGISTRY.get(&name) {
                let mut param_map = HashMap::new();
                for p in params { let parts: Vec<&str> = p.split(':').collect(); param_map.insert(parts[0].to_string(), parts[1].as_bytes().to_vec()); }
                let artifact = tmpl(&param_map);
                println!("{}", serde_json::to_string(&artifact).unwrap());
                Ok(())
            } else {
                Err(CliError::Invalid("Unknown template".to_string()))
            }
        }
        Command::Stream { media_type, file, hash } => {
            let file = File::open(file).await.unwrap();
            match media_type.as_str() {
                "image" => {
                    let proto = ImageProtocol { hash: hex::decode(hash).unwrap().try_into().unwrap() };
                    let handle = nprint_runtime::stream_media(proto, file);
                    handle.await.unwrap().map_err(CliError::Runtime)?;
                }
                // Add doc/music/video
                _ => Err(CliError::Invalid("Unknown media".to_string())),
            }
            Ok(())
        }
    }
}
