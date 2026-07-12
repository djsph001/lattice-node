// Phase 8 — Agent state types for the distributed agent harness.
//
// These types represent the lifecycle of an agent task as it moves
// through the mesh: submitted → assigned → executing → checkpointed
// → completed (or failed and re-routed).

use serde::{Deserialize, Serialize};

/// The lifecycle status of an agent running on a node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentStatus {
    /// Task accepted, not yet started.
    Idle,
    /// Actively executing. Carries the step index within the graph.
    Running { step: u32 },
    /// Execution paused, state saved to checkpoint. Peers can resume.
    AwaitingCheckpoint { step: u32 },
    /// Execution failed. Carries error metadata for diagnosis.
    Failed { step: u32, reason: String },
    /// Execution completed successfully.
    Completed,
}

/// An agent task — the unit of work submitted to the mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    /// Unique identifier for this task (UUID v4).
    pub task_id: String,
    /// The PeerId of the node that submitted the task (string form).
    pub origin: String,
    /// The model required to execute this task (e.g. "nemotron-3-nano").
    pub model: String,
    /// Version of the agent harness this task was built for.
    pub harness_version: u32,
    /// Opaque task payload — the execution graph serialized as bytes.
    /// Schema is application-defined; the lattice-node routes it, doesn't interpret it.
    pub graph_blob: Vec<u8>,
    /// Blake3 hash of `graph_blob` — used by checkpoints to verify integrity.
    pub graph_hash: [u8; 32],
    /// Deadline epoch. If the task isn't completed by this epoch number,
    /// it's eligible for re-routing (Phase 8b).
    pub deadline_epoch: u64,
    /// Timestamp when the task was created (Unix epoch seconds).
    pub created_at: u64,
}

/// The full state record for an agent tracked in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRecord {
    /// The task definition (immutable after creation).
    pub task: AgentTask,
    /// Which node is currently assigned to execute this task.
    pub assigned_node: String,
    /// Current lifecycle status.
    pub status: AgentStatus,
    /// The most recent checkpoint, if any.
    pub last_checkpoint: Option<super::checkpoint::Checkpoint>,
    /// When the status last changed (Unix epoch seconds).
    pub updated_at: u64,
}

/// Request-response types for agent state queries over the
/// /lattice/agent-state/v1 protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStateQuery {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStateReply {
    /// None if the task isn't known to the responding node.
    pub record: Option<AgentRecord>,
}
