use nprint_protocols::{MediaProtocol};
use nprint_types::SmartContract;
use sv::messages::{Tx as Transaction, TxIn, TxOut, OutPoint};
use sv::script::Script;
use sv::transaction::sighash::{sighash, SigHashCache as SighashCache};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::task::JoinHandle;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Network: {0}")]
    Network(String),
    #[error("Script: {0}")]
    Script(String),
    #[error("Wallet: {0}")]
    Wallet(String),
}

pub struct Provider {
    #[allow(dead_code)]
    url: String,
}

impl Provider {
    pub fn new(node: &str) -> Self {
        Self { url: node.to_string() }
    }

    async fn broadcast(&self, _tx: Transaction) -> Result<String, RuntimeError> {
        Ok("txid".to_string()) // Stub
    }

    #[allow(dead_code)]
    async fn get_utxo(&self, _addr: String) -> Result<OutPoint, RuntimeError> {
        Ok(OutPoint::default()) // Stub
    }
}

pub trait Signer {
    fn sign(&self, tx: &mut Transaction) -> Result<(), RuntimeError>;
}

pub async fn deploy(contract: impl SmartContract, signer: impl Signer, provider: Provider) -> Result<String, RuntimeError> {
    let artifact = contract.compile();
    let mut tx = Transaction::default();
    let out = TxOut { satoshis: 0, lock_script: Script(artifact.script) };
    tx.outputs.push(out);
    signer.sign(&mut tx)?;
    provider.broadcast(tx).await
}

pub async fn call(contract: impl SmartContract, _method: &str, _args: Vec<Vec<u8>>, _utxo: String, signer: impl Signer, provider: Provider) -> Result<String, RuntimeError> {
    let artifact = contract.compile();
    let mut tx = Transaction::default();
    let inp = TxIn { prev_output: OutPoint::default(), unlock_script: Script(vec![]), sequence: 0 };
    tx.inputs.push(inp);
    let out = TxOut { satoshis: 0, lock_script: Script(artifact.script) };
    tx.outputs.push(out);
    signer.sign(&mut tx)?;
    provider.broadcast(tx).await
}

/// Stream media per protocol (image/video/audio/doc).
pub fn stream_media(proto: impl MediaProtocol + Send + 'static, mut source: impl AsyncRead + Unpin + Send + 'static) -> JoinHandle<Result<(), RuntimeError>> {
    tokio::spawn(async move {
        let mut data = Vec::new();
        source.read_to_end(&mut data).await.unwrap();
        proto.verify(data, proto.get_hash());
        Ok(())
    })
}
