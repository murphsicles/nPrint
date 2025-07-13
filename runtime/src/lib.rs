use nprint_types::{SmartContract, Artifact};
use nprint_core::{Stack, bsv_script};
use sv::messages::{Tx, TxIn, TxOut, OutPoint};
use sv::script::Script;
use sv::network::Network;
use sv::wallet::extended_key::ExtendedPrivKey;
use sv::util::hash256::Hash256;
use tokio::{spawn, task::JoinHandle};
use tokio::io::AsyncRead;
use reqwest::Client;
use serde_json::json;
use thiserror::Error;
use tokio_stream::StreamExt;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Tx build failed: {0}")]
    TxBuild(String),
    #[error("RPC failed: {0}")]
    Rpc(reqwest::Error),
}

/// Signer trait: e.g., private key.
pub trait Signer {
    fn sign(&self, tx: &mut Tx) -> Result<(), RuntimeError>;
}
impl Signer for ExtendedPrivKey {
    fn sign(&self, tx: &mut Tx) -> Result<(), RuntimeError> {
        tx.sign(self).map_err(|e| RuntimeError::TxBuild(e.to_string()))
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
        let hex_tx = tx.to_hex();
        let resp = self.client.post(&self.url).json(&json!({ "method": "sendrawtransaction", "params": [hex_tx] })).send().await.map_err(RuntimeError::Rpc)?;
        resp.text().await.map_err(RuntimeError::Rpc)
    }
}

/// Deploy contract async.
pub async fn deploy<C: SmartContract + Send + 'static>(contract: C, signer: impl Signer + Send + 'static, provider: Provider) -> Result<String, RuntimeError> {
    let artifact = contract.compile();
    let mut tx = Tx::new(Network::Mainnet);  // Adjust network
    let out = TxOut { satoshis: 1, lock_script: Script::from(artifact.script) };
    tx.add_output(&out);
    signer.sign(&mut tx)?;
    provider.broadcast(tx).await
}

/// Call method async (build tx spending UTXO).
pub async fn call<C: SmartContract>(contract: C, method: &str, args: Vec<Vec<u8>>, utxo_txid: String, signer: impl Signer, provider: Provider) -> Result<String, RuntimeError> {
    let artifact = contract.compile();
    let unlocking_script = bsv_script! { /* args pushes + method script */ };
    let mut tx = Tx::new(Network::Mainnet);
    let input = TxIn { prev_output: OutPoint { hash: Hash256::decode(&utxo_txid).unwrap(), index: 0 }, unlock_script: unlocking_script, sequence: 0xffffffff };
    tx.add_input(&input);
    signer.sign(&mut tx)?;
    provider.broadcast(tx).await
}

/// Async streaming for media (integrate protocols).
pub fn stream_media(proto: impl nprint_protocols::MediaProcessor + Send + 'static, source: impl AsyncRead + Unpin + Send + 'static) -> JoinHandle<Result<(), RuntimeError>> {
    spawn(async move {
        let mut stream = proto.process_stream(source);
        while let Some(chunk) = stream.next().await {
            // Simulate on-chain verify per chunk
            let _ = chunk?;  // Process
        }
        Ok(())
    })
}
