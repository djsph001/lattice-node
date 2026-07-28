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

/// A peer entry in the GetPeers response.
#[derive(Debug, Serialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub name: Option<String>,
    pub heartbeats: u64,
    pub silence_secs: u64,
    pub is_dead: bool,
    pub queue_depth: u64,
    /// Thickness earned by this peer — derived from the
    /// ThicknessGraph at read time. None if thickness has
    /// never been computed for this peer (fresh node).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,
    /// Number of distinct peers that have witnessed claims
    /// for this peer. None if no claims have been accepted yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_witnesses: Option<u64>,
}

/// A peer balance entry in the GetEconomicState response.
#[derive(Debug, Serialize)]
pub struct PeerBalance {
    pub peer_id: String,
    pub balance: u64,
    pub nonce: u64,
}

// ── Request / Response types ──────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ApiRequest {
    GetHeight,
    GetBlock { height: u64 },
    GetCertificate { proposal_id: String },
    GetStats,
    GetPeers,
    /// Return the node's current state_root hash and epoch.
    /// Computed on demand from in-memory state — reflects the
    /// ledger at query time, which may be mid-epoch.
    GetStateRoot,
    GetEpochState,
    GetEconomicState,
    GetNodeInfo,
    GetPersistenceState,
    /// Phase 8: Submit an agent task for distributed execution.
    AgentSubmit {
        task_id: String,
        model: String,
        model_size: String,  // "tiny"|"small"|"medium"|"large" — Phase 10a
        /// Minimum VRAM in bytes required (default: 0 = no VRAM requirement).
        #[serde(default)]
        vram_bytes: u64,
        graph_blob_b64: String,
        deadline_epoch: u64,
    },
    SubmitClaim {
        claim_id: String,
        domain_tag: String,
        claim_type: String,
        bound_commit: String,
        content: String,
        evidence: Vec<String>,
        thickness: f64,
    },
    /// Initiate a service attestation claim — the node identifies
    /// witnesses, collects signatures, assembles and accepts the
    /// claim. Returns immediately with a claim_id; poll ClaimStatus
    /// to see results.
    WitnessClaimService,
    /// Query the status of a previously submitted witness claim.
    GetClaimStatus { claim_id: String },
    /// Submit an objection against a witnessed claim.
    /// The node signs with its own identity key — the caller
    /// only provides the target and reason.  Submitted objections
    /// go through the same validate → dedup → cap → persist
    /// → gossip pipeline as received ones.
    SubmitObjection {
        target_claim_id: String,
        reason: String,
    },
    /// Retrieve objections against a specific claim.
    GetObjections { claim_id: String },
    /// Retrieve all objections known to this node.
    GetAllObjections,
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
    /// Dashboard peers list — heartbeat liveness, silence, queue depth.
    Peers {
        peers: Vec<PeerInfo>,
    },
    /// Current state_root hash and epoch. Two nodes at the same
    /// epoch should produce identical roots if their state agrees.
    StateRoot {
        state_root: String,
        epoch: u64,
    },
    /// Epoch state — last completed epoch's economic parameters.
    EpochState {
        epoch: u64,
        ratio: Option<f64>,
        tax_calculated: Option<u64>,
        tax_collected: Option<u64>,
        minted: Option<u64>,
        redistributed_to: Option<u64>,
    },
    /// Economic state — balances, nonces.
    EconomicState {
        own_balance: u64,
        own_nonce: u64,
        peers: Vec<PeerBalance>,
    },
    /// Node identity and build info.
    NodeInfo {
        peer_id: String,
        name: String,
        genesis_root_id: String,
        chain_tip: u64,
        uptime_secs: u64,
        build_commit: String,
        /// This node's own earned thickness. None if no claims
        /// have been credited yet.
        #[serde(skip_serializing_if = "Option::is_none")]
        thickness: Option<f64>,
        /// Number of distinct peers that have witnessed claims
        /// accepted by this node. None if no claims accepted yet.
        #[serde(skip_serializing_if = "Option::is_none")]
        distinct_witnesses: Option<u64>,
        /// This node's earned (non-genesis) thickness — sum of
        /// VerifiedContribution edges. None when zero earned.
        /// Distinct from total thickness (which includes genesis
        /// and vouches), this is what was actually credited
        /// through work — witness claims, relay receipts,
        /// service attestations.
        #[serde(skip_serializing_if = "Option::is_none")]
        earned_thickness: Option<f64>,
    },
    /// Persistence state — WAL and snapshot status.
    PersistenceState {
        last_snapshot_epoch: u64,
        wal_bytes: u64,
        wal_entries: u64,
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
    ClaimSigned {
        claim_id: String,
        signature: String,
    },
    ClaimRefused {
        claim_id: String,
        reason: String,
        refused_because: String,
    },
    /// Witness claim successfully initiated — poll ClaimStatus for results.
    ClaimSubmitted { claim_id: String },
    /// A claim is already collecting for this (self, claim_type).
    AlreadyCollecting { claim_id: String },
    /// No claimable window exists — either the node is fresh or a
    /// claim was already submitted in the current epoch.
    NothingToClaim,
    /// Status of a previously submitted witness claim.
    ClaimStatus {
        claim_id: String,
        status: String,              // "collecting" | "accepted" | "rejected"
        candidates: usize,
        signatures_collected: usize,
        declines: Vec<ClaimDeclineInfo>,
        result: Option<ClaimResultInfo>,
    },
    /// Objection successfully submitted and gossiped.
    ObjectionSubmitted { claim_id: String },
    /// Objections for a specific claim.
    Objections {
        claim_id: String,
        objections: Vec<ObjectionInfo>,
    },
    /// All objections known to this node, keyed by claim ID.
    AllObjections {
        objections: std::collections::HashMap<String, Vec<ObjectionInfo>>,
    },
}

/// Public-facing objection info — excludes cryptographic payload
/// so the API response is readable and compact.
#[derive(Debug, Serialize)]
pub struct ObjectionInfo {
    pub target_claim_id: String,
    pub objector: String,
    pub reason: String,
    pub timestamp: String,
}

/// A request sent from the API server task to the main event loop,
/// paired with a oneshot sender for the response.
pub struct ApiMessage {
    pub request: ApiRequest,
    pub reply: oneshot::Sender<ApiResponse>,
}

/// Information about a witness that declined to sign.
#[derive(Debug, Serialize)]
pub struct ClaimDeclineInfo {
    pub witness: String,
    pub reason: String,
}

/// Result of a completed witness claim.
#[derive(Debug, Serialize)]
pub struct ClaimResultInfo {
    pub thickness: Option<f64>,
    pub acceptance: String,  // "accepted" | "rejected"
    pub reason: Option<String>,
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
