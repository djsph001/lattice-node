// Phase 8 — Agent execution checkpoints.
//
// A checkpoint captures the minimal state needed to resume an agent
// task from a specific step in its execution graph. Checkpoints are
// stored in the AgentRegistry alongside the task record.

use serde::{Deserialize, Serialize};

/// A saved execution checkpoint — the minimal state needed to resume
/// an agent task from a specific step in its execution graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// The task this checkpoint belongs to.
    pub task_id: String,
    /// The graph_hash from the original AgentTask — verified on resume
    /// to ensure the graph hasn't been tampered with.
    pub graph_hash: [u8; 32],
    /// Which step in the execution graph this checkpoint represents.
    pub step_index: u32,
    /// Serialized execution state at this step. Schema is application-defined.
    pub state_blob: Vec<u8>,
    /// Blake3 hash of `state_blob` — verifiable by any peer.
    pub state_hash: [u8; 32],
    /// Epoch number when this checkpoint was created.
    pub epoch: u64,
    /// Unix epoch seconds when this checkpoint was written.
    pub timestamp: u64,
}

impl Checkpoint {
    /// Compute the state_hash from a state_blob.
    pub fn compute_state_hash(blob: &[u8]) -> [u8; 32] {
        blake3::hash(blob).into()
    }

    /// Verify that this checkpoint's state_hash matches its state_blob.
    pub fn verify(&self) -> bool {
        Self::compute_state_hash(&self.state_blob) == self.state_hash
    }
}
