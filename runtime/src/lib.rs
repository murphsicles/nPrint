use nprint_types::{SmartContract, Artifact};
use nprint_core::{bsv_script};
use sv::messages::{Tx, TxIn, TxOut, OutPoint};
use sv::script::Script;
use sv::util::Hash256;
use tokio::{spawn, task::JoinHandle};
use tokio::io::AsyncRead;
use reqwest::Client;
use serde_json::json;
use thiserror::Error;
use tokio_stream::StreamExt;
use sv::util::Serializable;
use std::io::Write as IoWrite;
use hex;
use sv::wallet::extended_key::ExtendedKey;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Tx build failed: {0}")]
    TxBuild(String),
    #[error("RPC failed: {0}")]
    Rpc(reqwest::Error),
    #[error("IO error: {0}")]
    Io(std::io::Error),
}

/// Signer trait: e.g., private key.
pub trait Signer {
    fn sign(&self, tx: &mut Tx) -> Result<(), RuntimeError>;
}

impl Signer for ExtendedKey {
    fn sign(&self, tx: &mut Tx) -> Result<(), RuntimeError> {
        // Stub for compilation; implement based on library
        Ok(())
    }
}

/// Provider: BSV node RPC.
pub struct Provider {
    url: String,
    client: Client,
}
impl Provider {
    pub fn new(url: &str) -> Self { Self { url: url.to_string(), client: Client::new() } }

    pub async fn broadcast(&self, tx: Tx) -> Result<String, RuntimeError> {
        let mut v = Vec::new();
        tx.write(&mut v).map_err(|e| RuntimeError::TxBuild(e.to_string()))?;
        let hex_tx = hex::encode(&v);
        let resp = self.client.post(&self.url).json(&json!({ "method": "sendrawtransaction", "params": [hex_tx] })).send().await.map_err(RuntimeError::Rpc)?;
        resp.text().await.map_err(RuntimeError::Rpc)
    }
}

/// Deploy contract async.
pub async fn deploy<C: SmartContract + Send + 'static>(contract: C, signer: impl Signer + Send + 'static, provider: Provider) -> Result<String, RuntimeError> {
    let artifact = contract.compile();
    let mut tx = Tx { version: 2, lock_time: 0, inputs: vec![], outputs: vec![] };
    let out = TxOut { satoshis: 1, lock_script: Script(artifact.script) };
    tx.outputs.push(out);
    signer.sign(&mut tx)?;
    provider.broadcast(tx).await
}

/// Call method async (build tx spending UTXO).
pub async fn call<C: SmartContract>(contract: C, method: &str, args: Vec<Vec<u8>>, utxo_txid: String, signer: impl Signer, provider: Provider) -> Result<String, RuntimeError> {
    let artifact = contract.compile();
    let unlocking_script = bsv_script! { /* args pushes + method script */ };
    let mut tx = Tx { version: 2, lock_time: 0, inputs: vec![], outputs: vec![] };
    let input = TxIn { prev_output: OutPoint { hash: Hash256::decode(&utxo_txid).unwrap(), index: 0 }, unlock_script: Script(unlocking_script), sequence: 0xffffffff };
    tx.inputs.push(input);
    signer.sign(&mut tx)?;
    provider.broadcast(tx).await
}

/// Async streaming for media (integrate protocols).
pub fn stream_media(proto: impl nprint_protocols::MediaProcessor + Send + 'static, source: impl AsyncRead + Unpin + Send + 'static) -> JoinHandle<Result<(), RuntimeError>> {
    spawn(async move {
        let mut stream = proto.process_stream(source);
        while let Some(chunk) = stream.next().await {
            // Simulate on-chain verify per chunk
            let _ = chunk.map_err(RuntimeError::Io)?;  // Process
        }
        Ok(())
    })
}
