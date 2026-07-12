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

pub mod checkpoint;
pub mod codec;
pub mod registry;
pub mod state;

pub use checkpoint::Checkpoint;
pub use codec::{AgentStateCodec, AGENT_STATE_PROTOCOL};
pub use registry::AgentRegistry;
pub use state::{AgentRecord, AgentStateQuery, AgentStateReply, AgentStatus, AgentTask};
