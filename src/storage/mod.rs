// ── storage/mod.rs ─────────────────────────────────────────
//
// Storage verification module (Phase 6).  Provides:
//
//   • challenge.rs — deterministic challenge generation (validator side)
//   • merkle.rs   — Merkle tree construction and path extraction
//   • proof.rs    — target-side proof generation (disk I/O + hashing)
//
// The key insight: the `chunk_index` is deterministic per
// (resource_id, epoch), so every validator targeting the same peer
// in the same epoch arrives at the identical challenge.  Proofs are
// reusable and instantly cross-verifiable — no coordination needed.

pub mod challenge;
pub mod merkle;
pub mod proof;

pub use proof::ProofEngine;
