# Phase 8: Agent Harness — Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Evolve the lattice-node from hardware heartbeat mesh to distributed agent runtime. Nodes track not just whether peers are alive, but what agent tasks they're executing, at which checkpoint, with automatic re-routing on heartbeat failure.

**Architecture:** Three new modules under `src/agent/` — state types, file-backed registry, and task routing. A new `AgentTask` variant on the `LatticeMessage` wire enum, a new `lattice/agent/v1` gossipsub topic, and an `agent_task` request-response protocol. The existing economic engine taxes agent compute consumption as a contribution metric. Deadline monitoring hooks into the epoch timer; heartbeat-failure recovery hooks into the peer expiry path.

**Tech Stack:** Rust (existing), serde + serde_json for registry persistence, Blake3 for checkpoint hashing, libp2p request-response for agent state queries, existing gossipsub for task broadcast.

---

## Phase 8a: Core Agent Infrastructure (this plan)

This phase delivers the minimum viable agent harness: types, registry, checkpointing, wire protocol, and CLI surface. Tasks can be submitted, checkpoints recorded, and state queried. Multi-node task routing and heartbeat-failure migration are Phase 8b.

---

### Task 1: Create agent module scaffold and state types

**Objective:** Create `src/agent/` with `mod.rs`, `state.rs`, and `checkpoint.rs`. Define the core data types that every other task builds on.

**Files:**
- Create: `src/agent/mod.rs`
- Create: `src/agent/state.rs`
- Create: `src/agent/checkpoint.rs`
- Modify: `src/main.rs` (add `mod agent;`)

**Step 1: Create `src/agent/state.rs`**

Core types that represent an agent's lifecycle:

```rust
// src/agent/state.rs
use serde::{Deserialize, Serialize};
use libp2p::PeerId;

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
    /// The PeerId of the node that submitted the task.
    pub origin: String,
    /// The model required to execute this task (e.g. "nemotron-3-nano").
    pub model: String,
    /// Version of the agent harness this task was built for.
    pub harness_version: u32,
    /// Opaque task payload — the execution graph serialized as bytes.
    /// Up to the submitting application to define the schema.
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
    pub last_checkpoint: Option<Checkpoint>,
    /// When the status last changed (Unix epoch seconds).
    pub updated_at: u64,
}
```

**Step 2: Create `src/agent/checkpoint.rs`**

```rust
// src/agent/checkpoint.rs
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
```

**Step 3: Create `src/agent/mod.rs`**

```rust
// src/agent/mod.rs
pub mod checkpoint;
pub mod registry;
pub mod state;

pub use checkpoint::Checkpoint;
pub use registry::AgentRegistry;
pub use state::{AgentRecord, AgentStatus, AgentTask};
```

**Step 4: Register module in `src/main.rs`**

Add `mod agent;` after the existing `mod api;` line.

**Verification:** `cargo check` passes with the new module. No logic yet — just types compiling.

---

### Task 2: Create agent state codec for request-response protocol

**Objective:** Add `AgentStateRequest` / `AgentStateResponse` message types and a `request_response::Codec` impl so nodes can query each other's agent state.

**Files:**
- Create: `src/agent/codec.rs`
- Modify: `src/agent/mod.rs` (add `pub mod codec;`)
- Modify: `src/message/types.rs` (add AgentStateRequest / AgentStateResponse)

**Step 1: Add message types to `src/message/types.rs`**

Append after the existing `StorageProofData` struct:

```rust
// ── Phase 8: agent harness ──────────────────────────────────

/// Request an agent's current state from the assigned node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStateRequest {
    /// The task_id being queried.
    pub task_id: String,
}

/// Response to an AgentStateRequest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStateResponse {
    /// The full agent record, or None if the task isn't known.
    pub record: Option<super::super::agent::state::AgentRecord>,
}
```

Wait — we need to avoid circular module references. Better to put the response type directly in the agent module and reference it from `message/types.rs` only for the codec.

Actually, the cleanest approach: put `AgentStateRequest` and `AgentStateResponse` in `src/agent/state.rs` alongside the other agent types. They're agent types, not message types.

Let me revise. The `message/types.rs` file holds the wire protocol types used by `LatticeMessage` and the codecs. The agent types should stay in `agent/`. The codec just needs to reference both.

Let me restructure:

In `src/agent/state.rs`, add:

```rust
/// Request-response types for agent state queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStateQuery {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStateReply {
    pub record: Option<AgentRecord>,
}
```

**Step 2: Create `src/agent/codec.rs`**

```rust
// src/agent/codec.rs
// Phase 8 — request-response codec for agent state queries.
// Protocol: /lattice/agent-state/v1
use async_trait::async_trait;
use libp2p::request_response;
use libp2p::StreamProtocol;

use super::state::{AgentStateQuery, AgentStateReply};

#[derive(Debug, Clone)]
pub struct AgentStateCodec;

/// Protocol name for agent state queries.
pub const AGENT_STATE_PROTOCOL: &str = "/lattice/agent-state/v1";

#[async_trait]
impl request_response::Codec for AgentStateCodec {
    type Protocol = StreamProtocol;
    type Request = AgentStateQuery;
    type Response = AgentStateReply;

    async fn read_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Request>
    where
        T: futures::AsyncRead + Unpin + Send,
    {
        use futures::AsyncReadExt;
        let mut len_buf = [0u8; 4];
        io.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        io.read_exact(&mut buf).await?;
        serde_json::from_slice(&buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    async fn read_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Response>
    where
        T: futures::AsyncRead + Unpin + Send,
    {
        use futures::AsyncReadExt;
        let mut len_buf = [0u8; 4];
        io.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        io.read_exact(&mut buf).await?;
        serde_json::from_slice(&buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    async fn write_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> std::io::Result<()>
    where
        T: futures::AsyncWrite + Unpin + Send,
    {
        use futures::AsyncWriteExt;
        let bytes = serde_json::to_vec(&req)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        io.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
        io.write_all(&bytes).await?;
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> std::io::Result<()>
    where
        T: futures::AsyncWrite + Unpin + Send,
    {
        use futures::AsyncWriteExt;
        let bytes = serde_json::to_vec(&res)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        io.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
        io.write_all(&bytes).await?;
        Ok(())
    }
}
```

**Step 3: Update `src/agent/mod.rs`**

Add `pub mod codec;`.

**Verification:** `cargo check` — the codec compiles. No integration yet.

---

### Task 3: Create file-backed AgentRegistry

**Objective:** Implement `AgentRegistry` in `src/agent/registry.rs` — an in-memory registry backed by a JSON file on disk. Matches the simplicity of the existing `CommitManager` pattern (file-backed, no external DB dependency).

**Files:**
- Create: `src/agent/registry.rs`
- Modify: `src/agent/mod.rs` (already exports `AgentRegistry`)

**Implementation: `src/agent/registry.rs`**

```rust
// Phase 8 — Agent Registry
// File-backed JSON store for agent task state. One JSON object per line
// (JSONL format) so the registry can be appended to without rewriting
// the entire file. On load, the last record for each task_id wins.
//
// Stored at: <storage_dir>/agent_registry.jsonl

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use tracing::{debug, info};

use super::state::{AgentRecord, AgentStatus};

/// File-backed agent state registry.
#[derive(Debug)]
pub struct AgentRegistry {
    /// In-memory state — keyed by task_id.
    records: HashMap<String, AgentRecord>,
    /// Path to the JSONL file on disk.
    file_path: PathBuf,
}

impl AgentRegistry {
    /// Open or create the registry in the given storage directory.
    pub fn open(storage_dir: &PathBuf) -> Self {
        let file_path = storage_dir.join("agent_registry.jsonl");
        let mut records = HashMap::new();

        if file_path.exists() {
            if let Ok(file) = File::open(&file_path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(record) = serde_json::from_str::<AgentRecord>(&line) {
                            // Last write wins for each task_id
                            records.insert(record.task.task_id.clone(), record);
                        }
                    }
                }
            }
            info!(
                count = records.len(),
                "[agent-registry] Loaded existing registry"
            );
        } else {
            info!("[agent-registry] No existing registry — starting fresh");
        }

        Self { records, file_path }
    }

    /// Number of tracked agent tasks.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Look up an agent by task_id.
    pub fn get(&self, task_id: &str) -> Option<&AgentRecord> {
        self.records.get(task_id)
    }

    /// List all task_ids assigned to a specific node.
    pub fn tasks_for_node(&self, peer_id: &str) -> Vec<&AgentRecord> {
        self.records
            .values()
            .filter(|r| r.assigned_node == peer_id)
            .collect()
    }

    /// Register a new task. Returns error if task_id already exists.
    pub fn register(&mut self, record: AgentRecord) -> Result<(), String> {
        if self.records.contains_key(&record.task.task_id) {
            return Err(format!(
                "Task {} already registered",
                record.task.task_id
            ));
        }
        self.records.insert(record.task.task_id.clone(), record.clone());
        self.append_to_file(&record);
        debug!(
            task_id = %record.task.task_id,
            node = %record.assigned_node,
            "[agent-registry] Task registered"
        );
        Ok(())
    }

    /// Update an existing task's status and checkpoint.
    pub fn update_status(
        &mut self,
        task_id: &str,
        status: AgentStatus,
        checkpoint: Option<super::checkpoint::Checkpoint>,
    ) -> Result<(), String> {
        let record = self
            .records
            .get_mut(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;

        record.status = status;
        record.last_checkpoint = checkpoint;
        record.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.append_to_file(record);
        debug!(
            task_id = %task_id,
            status = ?record.status,
            "[agent-registry] Status updated"
        );
        Ok(())
    }

    /// Reassign a task to a new node (used on heartbeat failure — Phase 8b).
    pub fn reassign(&mut self, task_id: &str, new_node: &str) -> Result<(), String> {
        let record = self
            .records
            .get_mut(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;

        record.assigned_node = new_node.to_string();
        record.status = AgentStatus::Idle;
        record.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.append_to_file(record);
        info!(
            task_id = %task_id,
            new_node = %new_node,
            "[agent-registry] Task reassigned"
        );
        Ok(())
    }

    /// Return all records for iteration.
    pub fn all(&self) -> impl Iterator<Item = &AgentRecord> {
        self.records.values()
    }

    // ── Internal ────────────────────────────────────────────

    fn append_to_file(&self, record: &AgentRecord) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            if let Ok(json) = serde_json::to_string(record) {
                let _ = writeln!(file, "{}", json);
                let _ = file.sync_all();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::state::{AgentStatus, AgentTask};

    fn make_test_record(task_id: &str, node: &str) -> AgentRecord {
        AgentRecord {
            task: AgentTask {
                task_id: task_id.to_string(),
                origin: "test-origin".to_string(),
                model: "test-model".to_string(),
                harness_version: 1,
                graph_blob: vec![1, 2, 3],
                graph_hash: blake3::hash(b"test-graph").into(),
                deadline_epoch: 100,
                created_at: 0,
            },
            assigned_node: node.to_string(),
            status: AgentStatus::Idle,
            last_checkpoint: None,
            updated_at: 0,
        }
    }

    #[test]
    fn test_register_and_lookup() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        let record = make_test_record("task-001", "node-alpha");
        reg.register(record).unwrap();

        assert_eq!(reg.len(), 1);
        let found = reg.get("task-001").unwrap();
        assert_eq!(found.task.task_id, "task-001");
        assert_eq!(found.assigned_node, "node-alpha");
    }

    #[test]
    fn test_duplicate_register_fails() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        reg.register(make_test_record("task-001", "node-alpha")).unwrap();
        assert!(reg.register(make_test_record("task-001", "node-beta")).is_err());
    }

    #[test]
    fn test_update_status() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        reg.register(make_test_record("task-001", "node-alpha")).unwrap();
        reg.update_status("task-001", AgentStatus::Running { step: 3 }, None).unwrap();

        let record = reg.get("task-001").unwrap();
        assert_eq!(record.status, AgentStatus::Running { step: 3 });
    }

    #[test]
    fn test_tasks_for_node() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();
        let mut reg = AgentRegistry::open(&storage);

        reg.register(make_test_record("task-001", "node-alpha")).unwrap();
        reg.register(make_test_record("task-002", "node-alpha")).unwrap();
        reg.register(make_test_record("task-003", "node-beta")).unwrap();

        let alpha_tasks = reg.tasks_for_node("node-alpha");
        assert_eq!(alpha_tasks.len(), 2);

        let beta_tasks = reg.tasks_for_node("node-beta");
        assert_eq!(beta_tasks.len(), 1);
    }

    #[test]
    fn test_persistence_across_opens() {
        let dir = tempfile::tempdir().unwrap();
        let storage = dir.path().to_path_buf();

        {
            let mut reg = AgentRegistry::open(&storage);
            reg.register(make_test_record("task-001", "node-alpha")).unwrap();
            reg.update_status("task-001", AgentStatus::Completed, None).unwrap();
        }

        // Re-open — should recover state
        let reg = AgentRegistry::open(&storage);
        assert_eq!(reg.len(), 1);
        let record = reg.get("task-001").unwrap();
        assert_eq!(record.status, AgentStatus::Completed);
    }
}
```

**Verification:** `cargo test agent::registry` — all 5 tests pass. `cargo check` clean.

---

### Task 4: Wire the agent task into LatticeMessage and gossipsub

**Objective:** Add an `AgentTask` variant to the `LatticeMessage` enum, a new gossipsub topic `lattice/agent/v1`, and the broadcast/receive plumbing.

**Files:**
- Modify: `src/message/types.rs` (add `AgentTaskMsg` struct, `AgentTask` variant to `LatticeMessage`)
- Modify: `src/network/protocol.rs` (add `AGENT_TOPIC` constant)
- Modify: `src/node.rs` (subscribe to agent topic, handle inbound agent messages)

**Step 1: Add `AgentTaskMsg` to `src/message/types.rs`**

After the `LatticeMessage::Transaction` variant, add:

```rust
/// A message carrying an agent task for distributed execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskMsg {
    /// The task definition (serialized AgentRecord.task).
    pub task_id: String,
    pub origin: String,
    pub model: String,
    pub harness_version: u32,
    pub graph_blob: Vec<u8>,
    pub graph_hash: [u8; 32],
    pub deadline_epoch: u64,
    pub created_at: u64,
}
```

Then update the `LatticeMessage` enum:

```rust
pub enum LatticeMessage {
    Heartbeat(Heartbeat),
    Status(StatusReport),
    Transaction(SignedTransaction),
    /// Phase 8: Agent task submission.
    AgentTask(AgentTaskMsg),
}
```

**Step 2: Add topic constant to `src/network/protocol.rs`**

```rust
/// Gossipsub topic for agent task propagation.
pub const LATTICE_AGENT_TOPIC: &str = "lattice/agent/v1";
```

**Step 3: Subscribe to agent topic in `node.rs`**

In the `new()` constructor, after the cert topic subscription, add:

```rust
let agent_topic = gossipsub::IdentTopic::new(LATTICE_AGENT_TOPIC);
gossipsub
    .subscribe(&agent_topic)
    .map_err(|e| anyhow::anyhow!("gossipsub agent subscribe: {e}"))?;
```

Also add the import for `LATTICE_AGENT_TOPIC` at the top of `node.rs`.

**Step 4: Handle inbound agent messages in the event loop**

In `handle_gossip_message()` (locate the existing handler), add a branch before the CBOR decode fallback that tries to decode `LatticeMessage` and matches on `AgentTask`:

```rust
// Phase 8: check for agent task messages
if message.topic == LATTICE_AGENT_TOPIC {
    match serde_cbor::from_slice::<LatticeMessage>(&message.data) {
        Ok(LatticeMessage::AgentTask(msg)) => {
            info!(
                task_id = %msg.task_id,
                origin = %msg.origin,
                model = %msg.model,
                "[agent] Received agent task"
            );
            // Phase 8a: log and acknowledge. Phase 8b: add to local registry
            // and begin execution if this node is the assigned executor.
            return;
        }
        Ok(other) => {
            debug!(topic = %message.topic, "Non-agent message on agent topic");
        }
        Err(e) => {
            warn!(error = %e, "[agent] Failed to decode agent task message");
        }
    }
}
```

**Verification:** `cargo build` compiles. Run a two-node simulation, publish an agent task on the new topic, verify both nodes log receipt.

---

### Task 5: Add AgentRegistry and agent_task RPC to LatticeNode

**Objective:** Integrate the agent registry and request-response protocol into the LatticeNode struct, constructor, and event loop.

**Files:**
- Modify: `src/node.rs` (add fields, constructor params, event loop branches)
- Modify: `src/main.rs` (add CLI flags, pass to constructor)
- Modify: `src/network/protocol.rs` (add `agent_rpc` behaviour + event)

**Step 1: Add `agent_rpc` to `LatticeBehaviour` in `src/network/protocol.rs`**

```rust
use crate::agent::codec::{AgentStateCodec, AGENT_STATE_PROTOCOL};
use crate::agent::state::{AgentStateQuery, AgentStateReply};

// Inside LatticeBehaviour struct:
pub agent_rpc: request_response::Behaviour<AgentStateCodec>,

// In LatticeBehaviourEvent enum:
AgentRpc(request_response::Event<AgentStateQuery, AgentStateReply>),

// Add From impl:
impl From<request_response::Event<AgentStateQuery, AgentStateReply>>
    for LatticeBehaviourEvent
{
    fn from(event: request_response::Event<AgentStateQuery, AgentStateReply>) -> Self {
        LatticeBehaviourEvent::AgentRpc(event)
    }
}
```

And in `LatticeBehaviour::new()` — add the `agent_rpc` param:

```rust
pub fn new(
    // ... existing params ...
    agent_rpc: request_response::Behaviour<AgentStateCodec>,
) -> Self {
    Self {
        // ... existing fields ...
        agent_rpc,
    }
}
```

**Step 2: Add fields to `LatticeNode` struct**

```rust
// ── Phase 8: agent harness ────────────────────────────────
/// File-backed registry of agent tasks and their state.
agent_registry: crate::agent::registry::AgentRegistry,
```

**Step 3: Add CLI flags to `main.rs`**

```rust
// ── Phase 8: agent harness ─────────────────────────────────
/// Enable agent mode — this node can accept and execute agent tasks.
#[arg(long, default_value_t = false)]
agent_mode: bool,
```

**Step 4: Update `LatticeNode::new()`**

Add `agent_mode: bool` param. In the constructor body, after the Kademlia behaviour build:

```rust
// Phase 8: agent state RPC
let agent_rpc = request_response::Behaviour::new(
    [(
        StreamProtocol::new(AGENT_STATE_PROTOCOL),
        request_response::ProtocolSupport::Full,
    )],
    request_response::Config::default(),
);
```

Pass `agent_rpc` to `LatticeBehaviour::new()`.

After the swarm is built, initialize the agent registry:

```rust
let storage_path = storage_dir
    .clone()
    .unwrap_or_else(|| PathBuf::from("./lattice-storage"));
let agent_registry = crate::agent::registry::AgentRegistry::open(&storage_path);
```

Add `agent_registry` and `agent_mode` fields to `Self { ... }`.

**Step 5: Handle `AgentRpc` events in the event loop**

In the `tokio::select!` block, when handling `SwarmEvent::Behaviour(LatticeBehaviourEvent::AgentRpc(event))`:

```rust
LatticeBehaviourEvent::AgentRpc(event) => {
    match event {
        request_response::Event::Message { peer, message } => {
            match message {
                request_response::Message::Request {
                    request_id, request, ..
                } => {
                    debug!(
                        task_id = %request.task_id,
                        from = %peer,
                        "[agent] Agent state query received"
                    );
                    let record = self.agent_registry.get(&request.task_id).cloned();
                    let reply = AgentStateReply { record };
                    if let Err(e) = self.swarm.behaviour_mut().agent_rpc
                        .send_response(request_id, reply)
                    {
                        warn!(error = %e, "[agent] Failed to send agent state response");
                    }
                }
                request_response::Message::Response { response, .. } => {
                    if let Some(record) = &response.record {
                        info!(
                            task_id = %record.task.task_id,
                            status = ?record.status,
                            node = %record.assigned_node,
                            "[agent] Agent state response"
                        );
                    } else {
                        debug!("[agent] Agent state query returned None");
                    }
                }
            }
        }
        request_response::Event::OutboundFailure { peer, request_id, error } => {
            warn!(%peer, ?request_id, %error, "[agent] Agent state query failed");
        }
        request_response::Event::InboundFailure { peer, request_id, error } => {
            warn!(%peer, ?request_id, %error, "[agent] Inbound agent state request failed");
        }
        _ => {}
    }
}
```

**Verification:** `cargo build` compiles. `cargo test` — existing tests still pass (no regression).

---

### Task 6: Add agent task submission method

**Objective:** Add a `submit_agent_task()` method to `LatticeNode` that creates an `AgentRecord`, serializes it as a `LatticeMessage::AgentTask`, broadcasts on the agent topic, and stores it in the local registry.

**Files:**
- Modify: `src/node.rs` (add method)
- Modify: `src/message/types.rs` (add `From<AgentTaskMsg>` conversion if needed)

**Implementation in `src/node.rs`:**

```rust
/// Submit an agent task to the mesh and store it in the local registry.
/// The task is broadcast on the agent gossipsub topic. Any node with
/// agent_mode enabled can pick it up and execute it.
pub fn submit_agent_task(
    &mut self,
    task_id: String,
    model: String,
    graph_blob: Vec<u8>,
    deadline_epoch: u64,
) -> Result<()> {
    let graph_hash: [u8; 32] = blake3::hash(&graph_blob).into();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let msg = crate::message::types::AgentTaskMsg {
        task_id: task_id.clone(),
        origin: self.local_peer_id.to_string(),
        model,
        harness_version: 1,
        graph_blob: graph_blob.clone(),
        graph_hash,
        deadline_epoch,
        created_at: now,
    };

    // Store locally
    let record = crate::agent::state::AgentRecord {
        task: crate::agent::state::AgentTask {
            task_id: task_id.clone(),
            origin: self.local_peer_id.to_string(),
            model: msg.model.clone(),
            harness_version: msg.harness_version,
            graph_blob,
            graph_hash,
            deadline_epoch,
            created_at: now,
        },
        assigned_node: self.local_peer_id.to_string(),
        status: crate::agent::state::AgentStatus::Idle,
        last_checkpoint: None,
        updated_at: now,
    };

    self.agent_registry
        .register(record)
        .map_err(|e| anyhow::anyhow!(e))?;

    // Broadcast on agent topic
    let envelope = crate::message::types::LatticeMessage::AgentTask(msg);
    let bytes = serde_cbor::to_vec(&envelope)?;

    match self.swarm.behaviour_mut().gossipsub.publish(
        gossipsub::IdentTopic::new(crate::network::protocol::LATTICE_AGENT_TOPIC),
        bytes,
    ) {
        Ok(_id) => {
            info!(
                task_id = %task_id,
                "[agent] Task submitted and broadcast"
            );
        }
        Err(gossipsub::PublishError::InsufficientPeers) => {
            debug!(
                task_id = %task_id,
                "[agent] Task stored locally (no peers to broadcast to yet)"
            );
        }
        Err(e) => {
            warn!(error = %e, "[agent] Failed to publish agent task");
        }
    }

    Ok(())
}
```

**Import additions to node.rs:**
```rust
use crate::network::protocol::LATTICE_AGENT_TOPIC;
use crate::message::types::AgentTaskMsg;
use crate::agent::state::{AgentRecord, AgentStatus, AgentTask};
```

**Verification:** `cargo build`. The method exists but isn't called yet — integration test will exercise it.

---

### Task 7: Add agent task submission from the UDS API

**Objective:** Extend the existing Unix Domain Socket API (`src/api.rs`) with an `AgentSubmit` endpoint so external tools (e.g., the Python sandbox orchestrator) can submit agent tasks programmatically.

**Files:**
- Modify: `src/api.rs` (add variants, handler)

**Step 1: Add to `ApiRequest` enum**

```rust
/// Submit an agent task for distributed execution.
AgentSubmit {
    task_id: String,
    model: String,
    /// Base64-encoded graph blob.
    graph_blob_b64: String,
    deadline_epoch: u64,
},
```

**Step 2: Add to `ApiResponse` enum**

```rust
AgentSubmitted {
    task_id: String,
    graph_hash: String,
},
AgentError {
    task_id: String,
    error: String,
},
```

**Step 3: Handle in the API message dispatch**

In the main event loop where `ApiMessage` is handled from the mpsc channel, add:

```rust
ApiRequest::AgentSubmit { task_id, model, graph_blob_b64, deadline_epoch } => {
    use base64::{Engine as _, engine::general_purpose};
    let graph_blob = match general_purpose::STANDARD.decode(&graph_blob_b64) {
        Ok(b) => b,
        Err(e) => {
            let _ = tx.send(ApiResponse::AgentError {
                task_id: task_id.clone(),
                error: format!("Base64 decode failed: {}", e),
            });
            continue;
        }
    };

    match self.submit_agent_task(task_id.clone(), model, graph_blob, deadline_epoch) {
        Ok(()) => {
            let graph_hash = blake3::hash(&graph_blob);
            let _ = tx.send(ApiResponse::AgentSubmitted {
                task_id,
                graph_hash: hex::encode(graph_hash.as_bytes()),
            });
        }
        Err(e) => {
            let _ = tx.send(ApiResponse::AgentError {
                task_id,
                error: e.to_string(),
            });
        }
    }
}
```

Note: This requires `base64` in `Cargo.toml` dependencies. Add `base64 = "0.22"`.

**Verification:** `cargo build`. Test via the UDS socket:

```bash
echo '{"AgentSubmit":{"task_id":"test-001","model":"nemotron-3-nano","graph_blob_b64":"AQID","deadline_epoch":100}}' | nc -U ./lattice-storage/lattice.sock
```

---

### Task 8: Integration test — agent task lifecycle

**Objective:** Write an integration test that starts two nodes, submits an agent task, verifies it propagates via gossipsub, queries agent state via the RPC protocol, and checks the registry persists across restarts.

**Files:**
- Create: `tests/agent_integration_test.rs`

**Test structure:**

```rust
// tests/agent_integration_test.rs
// Phase 8 — Agent harness integration test.
//
// Spins up two nodes, submits an agent task on alpha, verifies
// bravo receives it via gossipsub, queries agent state via RPC,
// and checks registry persistence.

#[tokio::test]
async fn test_agent_task_submit_and_query() {
    // 1. Start alpha node with agent_mode
    // 2. Start bravo node with agent_mode, bootstrap to alpha
    // 3. Wait for peer discovery (mDNS or bootstrap)
    // 4. Submit agent task on alpha
    // 5. Wait for bravo to receive via gossipsub (log grep)
    // 6. Query agent state on alpha via RPC
    // 7. Verify registry persistence
}
```

Given the complexity of spawning real libp2p nodes in a test, this integration test should be a shell script (like `simulate-lattice.sh`) rather than a Rust `#[test]`. The shell script:

```bash
#!/bin/bash
# tests/agent-integration.sh
set -uo pipefail

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

ALPHA_DIR="$TMPDIR/alpha"
BRAVO_DIR="$TMPDIR/bravo"
mkdir -p "$ALPHA_DIR" "$BRAVO_DIR"

BINARY="./target/release/lattice-node"

# Start alpha
"$BINARY" \
    --name alpha \
    --identity-dir "$ALPHA_DIR/identity" \
    --storage-dir "$ALPHA_DIR/storage" \
    --fresh-identity \
    --port 0 \
    --agent-mode \
    &> "$TMPDIR/alpha.log" &
ALPHA_PID=$!

sleep 2

# Extract alpha's address
CLEAN=$(sed $'s/\x1b\[[0-9;]*m//g' "$TMPDIR/alpha.log")
ALPHA_ADDR=$(echo "$CLEAN" | grep -oP 'listening on \K[^ ]+' | head -1)
ALPHA_PEER=$(echo "$CLEAN" | grep -oP 'peer_id=\K[^ ]+' | head -1)

if [ -z "$ALPHA_ADDR" ] || [ -z "$ALPHA_PEER" ]; then
    echo "FAIL: Could not extract alpha address/peer"
    kill $ALPHA_PID 2>/dev/null
    exit 1
fi

echo "Alpha: $ALPHA_ADDR/p2p/$ALPHA_PEER"

# Start bravo
"$BINARY" \
    --name bravo \
    --identity-dir "$BRAVO_DIR/identity" \
    --storage-dir "$BRAVO_DIR/storage" \
    --fresh-identity \
    --port 0 \
    --no-mdns \
    --bootstrap-peer "$ALPHA_ADDR/p2p/$ALPHA_PEER" \
    --agent-mode \
    &> "$TMPDIR/bravo.log" &
BRAVO_PID=$!

sleep 5

# Submit agent task via alpha's UDS socket
echo '{"AgentSubmit":{"task_id":"itest-001","model":"test-model","graph_blob_b64":"AQIDBAU=","deadline_epoch":999}}' | nc -U "$ALPHA_DIR/storage/lattice.sock" &
sleep 2

# Check alpha log for submission
if grep -q "Task submitted and broadcast" "$TMPDIR/alpha.log"; then
    echo "PASS: Task submitted on alpha"
else
    echo "FAIL: Task not submitted"
    kill $ALPHA_PID $BRAVO_PID 2>/dev/null
    exit 1
fi

# Check bravo log for receipt
if grep -q "Received agent task" "$TMPDIR/bravo.log"; then
    echo "PASS: Bravo received agent task via gossipsub"
else
    echo "FAIL: Bravo did not receive agent task"
    kill $ALPHA_PID $BRAVO_PID 2>/dev/null
    exit 1
fi

# Cleanup
kill $ALPHA_PID $BRAVO_PID 2>/dev/null
wait $ALPHA_PID $BRAVO_PID 2>/dev/null

echo "ALL TESTS PASSED"
```

**Verification:** `bash tests/agent-integration.sh` — all checks pass.

---

### Task 9: Update Cargo.toml with new dependencies

**Objective:** Add `base64` dependency for the UDS API's base64 graph blob decoding.

**Files:**
- Modify: `Cargo.toml`

Add under `[dependencies]`:
```toml
base64 = "0.22"
```

**Verification:** `cargo check` — no missing crate errors.

---

### Task 10: Documentation and module exports

**Objective:** Ensure all public types are exported, add module-level docs, and mark integration points for Phase 8b.

**Files:**
- Modify: `src/agent/mod.rs` (add module docs)
- Modify: `src/lib.rs` if it exists, otherwise `src/main.rs` (ensure agent types are accessible)

**Step 1: Add module docs to `src/agent/mod.rs`**

```rust
//! Agent Harness — Phase 8
//!
//! The agent harness extends the Lattice mesh from hardware heartbeat
//! monitoring to distributed agent execution. Key concepts:
//!
//! - **AgentTask**: A unit of work submitted to the mesh. Contains the
//!   model requirement, execution graph, and deadline.
//!
//! - **AgentRegistry**: File-backed JSONL store for tracking agent
//!   state across the mesh. Survives node restarts.
//!
//! - **Checkpoint**: A saved execution state at a specific graph step.
//!   Enables task migration between nodes on heartbeat failure.
//!
//! ## Phase 8a (current): Core Infrastructure
//! - Agent state types and registry
//! - Gossipsub propagation of agent tasks
//! - Request-response agent state queries
//! - UDS API endpoint for task submission
//!
//! ## Phase 8b (future): Task Routing
//! - Automatic task assignment to available nodes
//! - Heartbeat-failure detection triggers task migration
//! - Deadline monitoring at epoch boundaries
//! - Economic integration: agent compute = taxable contribution
```

**Verification:** `cargo doc --no-deps` — agent module appears with correct docs.

---

## Phase 8b: Task Routing & Heartbeat-Failure Migration (Future)

This phase closes the loop: heartbeat failure → agent re-routing → economic accounting.

Key tasks:
1. **Deadline monitor** — at each epoch tick, scan registry for tasks past `deadline_epoch` in non-terminal states → flag as failed
2. **Heartbeat-failure hook** — when a peer expires from the peer table, call `agent_registry.tasks_for_node(failed_peer)` → `reassign()` each to self or next available
3. **Auto-assignment** — when an `AgentTaskMsg` arrives and this node is in `agent_mode`, add to registry with `assigned_node = self`
4. **Economic integration** — agent compute time contributes to `EconomicEngine.metrics`, taxed at epoch boundary
5. **Sortition-based executor selection** — reuse `select_witness_panel()` pattern to deterministically pick which node executes a task (prevents everyone from executing the same task)

---

## Risks and Open Questions

1. **Registry format stability.** JSONL is append-only with last-write-wins. If the file grows large (thousands of tasks), load time increases. Mitigation: periodic compaction in a future phase. Not a problem for Phase 8a's scope (dozens of tasks).

2. **Gossipsub message size.** Agent task graph blobs could be large (megabytes). Gossipsub has a default 1 MiB message size limit. Mitigation: document that graphs should stay under 512 KiB; add a size check in `submit_agent_task()`. Alternative: use request-response for large payloads (Phase 8b).

3. **Duplicate task execution.** If multiple nodes are in `agent_mode`, they'll all receive the gossipsub message and could all try to execute. Mitigation: Phase 8b's sortition-based executor selection ensures only one node acts. For Phase 8a, nodes log receipt but don't auto-execute.

4. **No cryptographic task authentication.** The `origin` field is self-reported and not signed. A malicious node could submit tasks claiming to be someone else. Mitigation: Phase 8b adds Ed25519 task signatures (same pattern as `SignedTransaction`).

5. **Checkpoint blob storage.** Checkpoints are stored inline in the JSONL registry. This is fine for small state blobs (kilobytes) but won't scale to large model weights. Mitigation: for model-weight checkpointing, add a content-addressed blob store (Phase 9).

---

## Verification Checklist

After all Phase 8a tasks are complete:

- [ ] `cargo build` — zero errors, zero warnings
- [ ] `cargo test` — all existing tests pass + 5 new registry unit tests pass
- [ ] `cargo clippy` — no new warnings
- [ ] `bash tests/agent-integration.sh` — all integration checks pass
- [ ] `echo '{"AgentSubmit":{...}}' | nc -U lattice.sock` — task appears in registry
- [ ] Two-node manual test: alpha submits task, bravo logs receipt
- [ ] Registry file persists across node restart: re-open finds same tasks
