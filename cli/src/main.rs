use clap::{Parser, Subcommand};
use nprint_runtime::{deploy, call, Provider, stream_media, Signer, RuntimeError};
use nprint_templates::REGISTRY;
use nprint_types::{SmartContract, Artifact, Sha256};
use std::collections::HashMap;
use std::vec::Vec;
use thiserror::Error;
use tokio::fs::File as AsyncFile;
use tokio::runtime::Runtime;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Template not found")]
    TemplateNotFound,
    #[error("Runtime: {0}")]
    Runtime(nprint_runtime::RuntimeError),
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Deploy {
        #[arg(short, long)]
        template: String,
        #[arg(short, long)]
        params: Vec<String>,
    },
    Call {
        #[arg(short, long)]
        template: String,
        #[arg(short, long)]
        method: String,
        #[arg(short, long)]
        args: Vec<String>,
        #[arg(short, long)]
        utxo: String,
    },
    Stream {
        #[arg(short, long)]
        protocol: String,
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        hash: String,
    },
}

struct DummySigner;

impl Signer for DummySigner {
    fn sign(&self, _tx: &mut sv::messages::Tx) -> Result<(), RuntimeError> {
        Ok(())
    }
}

struct DummyContract;

impl SmartContract for DummyContract {
    fn compile(&self) -> Artifact { Artifact { script: vec![], props: vec![] } }
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let provider = Provider::new("http://node.example.com");
        let signer = DummySigner;
        let dummy_contract = DummyContract;
        match cli.command {
            Commands::Deploy { template: _template, params: _params } => {
                let txid = deploy(dummy_contract, signer, provider).await.map_err(CliError::Runtime)?;
                println!("Deployed: {txid}");
            }
            Commands::Call { template: _template, method, args, utxo } => {
                let arg_bytes: Vec<Vec<u8>> = args.iter().map(|a| a.as_bytes().to_vec()).collect();
                let txid = call(dummy_contract, &method, arg_bytes, utxo, signer, provider).await.map_err(CliError::Runtime)?;
                println!("Called: {txid}");
            }
            Commands::Stream { protocol, file, hash } => {
                let mut param_map = HashMap::new();
                param_map.insert("hash".to_string(), hex::decode(&hash).unwrap());
                let tmpl = REGISTRY.get(&protocol).ok_or(CliError::TemplateNotFound)?;
                let _artifact = tmpl(param_map);
                let file = AsyncFile::open(file).await.unwrap();
                let proto = ImageProtocol { hash: Sha256(hex::decode(&hash).unwrap().try_into().unwrap()) };
                let handle = stream_media(proto, file);
                handle.await.unwrap().unwrap();
            }
        }
        Ok(())
    })
}
