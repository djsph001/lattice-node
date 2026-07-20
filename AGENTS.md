# Lattice Node — Agent Guide

This file is for AI assistants working on `lattice-node`. Read it before making changes.

## Project

`lattice-node` is a Rust peer-to-peer mesh node for a sovereign economic layer. It runs libp2p gossipsub, a Blake3 hash-chain ledger, Georgist-style resource accounting, witness sortition, and a distributed agent harness.

Current production state (2026-07-19):
- **Era Two v1 is live**: single-producer `RatificationBlock` epoch summaries flow on `lattice/block/v1`.
- **QC path is dormant**: the 0x02 quorum-certificate path is implemented and tested but not activated in production.
- **Agent harness is Phase 8**: tasks propagate, registry persists, execution supports Ollama and OpenAI-compatible backends.
- **G2 catch-up is proven**: nodes can delete their ledger and catch up from peers via `ChainRangeRequest`.

## Build & Test

Always run the relevant tests before declaring work done:

```bash
# Fast library tests
cargo test --lib

# Binary tests (includes node.rs integration-style unit tests)
cargo test --bin lattice-node

# Full check
cargo check
```

Note: `src/node.rs` is part of the binary (`src/main.rs`), not `src/lib.rs`. Tests in `node.rs` only run with `cargo test --bin lattice-node`.

## Architecture Boundaries

- **Wire formats**: Do not change `BlockFrame`, `RatificationBlock` encoding, or gossip topic wire layouts without explicit user approval. Backward compatibility is critical.
- **Ledger**: `CommitManager` in `src/commit.rs` owns the append-only hash chain. `src/ledger/state.rs` owns balances and thickness. Do not bypass them.
- **Economic engine**: `src/economics/` runs the epoch cycle. Be careful with ordering-dependent mutations (most are causal, not commutative).
- **Agent harness**: `src/agent/` contains registry, state, checkpoints, and executors. New executor backends go in `src/agent/executor.rs` and follow the existing `Executor` enum pattern.
- **Sortition**: `src/sortition.rs` is deterministic; inputs must be identical across nodes.

## What NOT to Do

- Do not run `git commit`, `git push`, `git reset`, `git rebase`, or any git mutation. Ask the user for confirmation each time.
- Do not add new dependencies without checking `Cargo.toml` first and justifying the addition.
- Do not change CLI flags in a breaking way unless the user explicitly asks.
- Do not delete ledger files, database files, or running node state.
- Do not treat `node.rs` tests as library tests.

## Coding Style

- Match the surrounding file: comment density, naming, and structure.
- Use `tracing` for logs; prefer structured fields over string interpolation.
- Keep edits minimal and scoped. A tidy, reviewable diff beats opportunistic cleanup.
- Run `cargo check` and fix new warnings you introduce.

## Coordination Between Agents

If multiple agents are working on this repo:

1. **Scope tasks narrowly**: one agent per file or per feature. Avoid overlapping edits on `src/node.rs`.
2. **Use this file**: if you introduce a new convention or restriction, update `AGENTS.md` so other agents see it.
3. **Leave the repo clean**: don't leave uncommitted work for another agent unless the user asked for it.
4. **State the delta**: when summarizing work, list exact files changed and test results.

## Common Pitfalls

- `BlockFrame.signatures` is `(PeerId, Vec<u8>)` — public keys travel in QC gossip, not the ledger.
- `seen_nonces` stores the highest applied nonce per signer; gapped transactions go to `pending` and trigger `tx_rpc` fetch.
- ` outstanding_fetches` is cleaned up when gossip self-heals; don't assume a non-zero count is a bug.
- Genesis can be self-authored now (`--auto-genesis` or `--submit-genesis` without `--genesis-root`).
- `node.rs` async code spawns tasks with `tokio::spawn`; clones of `Executor` are cheap.

## Useful Commands

```bash
# Run a fresh mesh node with auto-genesis
./lattice-node --auto-genesis --genesis-amortize-over 100

# Run with OpenAI agent backend
./lattice-node --agent-mode --openai-api-key "$OPENAI_API_KEY"

# Run integration test
./tests/agent-integration.sh
```

## When in Doubt

Ask the user. This project has active human partners and a running production mesh; ambiguity is expensive.
