// Phase 7 — Query & Sync API via Unix Domain Socket
//
// Exposes the lattice-node ledger state through a local UDS
// endpoint.  The API server runs as a Tokio task, receiving
// requests over the socket and forwarding them to the main
// event loop via an mpsc channel.  Responses are returned
// through oneshot channels.
//
// Endpoints:
//   GetHeight              → { height: u64 }
//   GetBlock { height }    → { block: { height, hash, cert, sigs } }
//   GetCertificate { id }  → { certificate: { ... } }
//   GetStats               → { stats: { height, committed, peers } }
//
// Wire protocol: newline-delimited JSON over UDS.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info};

// ── Request / Response types ──────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ApiRequest {
    GetHeight,
    GetBlock { height: u64 },
    GetCertificate { proposal_id: String },
    GetStats,
    /// Phase 8: Submit an agent task for distributed execution.
    AgentSubmit {
        task_id: String,
        model: String,
        graph_blob_b64: String,
        deadline_epoch: u64,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ApiResponse {
    Height { height: u64 },
    Block {
        height: u64,
        block_hash: String,
        cert_hash: String,
        sig_count: u16,
    },
    Certificate {
        proposal_id: String,
        enclave_id: String,
        rounds: u32,
        witness_seed: String,
        validation: String,
        bytes: u64,
    },
    Certificates {
        certificates: Vec<String>,
    },
    Stats {
        height: u64,
        committed_count: u64,
    },
    Error {
        message: String,
    },
    AgentSubmitted {
        task_id: String,
        graph_hash: String,
    },
    AgentError {
        task_id: String,
        error: String,
    },
}

/// A request sent from the API server task to the main event loop,
/// paired with a oneshot sender for the response.
pub struct ApiMessage {
    pub request: ApiRequest,
    pub reply: oneshot::Sender<ApiResponse>,
}

// ── Server ─────────────────────────────────────────────────────

/// Start the UDS API server.  Returns the receiver end of the
/// channel that the main event loop should poll.
///
/// Spawns a background Tokio task that listens on `socket_path`
/// and forwards parsed requests into the channel.
pub fn spawn_api_server(
    socket_path: PathBuf,
) -> mpsc::Receiver<ApiMessage> {
    let (tx, rx) = mpsc::channel::<ApiMessage>(32);

    tokio::spawn(async move {
        // Remove stale socket
        if socket_path.exists() {
            let _ = std::fs::remove_file(&socket_path);
        }

        // Ensure parent directory exists
        if let Some(parent) = socket_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let listener = match UnixListener::bind(&socket_path) {
            Ok(l) => l,
            Err(e) => {
                error!(error = %e, path = %socket_path.display(), "[api] Failed to bind socket");
                return;
            }
        };

        info!(path = %socket_path.display(), "[api] Unix socket listening");

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let tx = tx.clone();
                    tokio::spawn(handle_client(stream, tx));
                }
                Err(e) => {
                    error!(error = %e, "[api] Accept error");
                }
            }
        }
    });

    rx
}

async fn handle_client(stream: UnixStream, tx: mpsc::Sender<ApiMessage>) {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let request: ApiRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = ApiResponse::Error {
                    message: format!("Invalid JSON: {}", e),
                };
                let _ = writer
                    .write_all(format!("{}\n", serde_json::to_string(&resp).unwrap()).as_bytes())
                    .await;
                continue;
            }
        };

        debug!(?request, "[api] Request");

        let (reply_tx, reply_rx) = oneshot::channel();
        let msg = ApiMessage {
            request,
            reply: reply_tx,
        };

        if tx.send(msg).await.is_err() {
            break; // channel closed, node shutting down
        }

        match reply_rx.await {
            Ok(response) => {
                let json = match serde_json::to_string(&response) {
                    Ok(j) => j,
                    Err(e) => format!(
                        "{{\"type\":\"Error\",\"message\":\"Serialization: {}\"}}",
                        e
                    ),
                };
                if writer.write_all(format!("{}\n", json).as_bytes()).await.is_err() {
                    break;
                }
            }
            Err(_) => {
                // oneshot dropped — node shutting down
                break;
            }
        }
    }
}
