# Verified Protocol Behavior

**Updated:** 2026-07-27
**Authority:** Every entry sourced to a test run, harness execution, or live
node observation. No claims without evidence.

This document records what's been demonstrated to work end-to-end. It is not
a design doc, not a roadmap, and not aspirational. If it's here, it was
executed on real nodes and produced the expected result.

---

## Genesis Lifecycle (Closed)

Full sequence verified on a two-node local mesh with snapshot rotation.

| Step | Produces |
|---|---|
| `--auto-genesis` | `SignedGenesis` persisted to WAL |
| `submit_genesis` | Published to `/lattice/genesis/v1` |
| Receive + validate | Root signer check, duplicate rejection |
| `handle_genesis_message` | Accept + persist on witness node |
| Recovery on restart | Genesis recovered from snapshot or WAL re-seed |
| Snapshot at epoch 10 | WAL rotated, fresh WAL seeded with Genesis |
| Restart after rotation | Genesis survives, `verify_consistency` passes |

**Known issues resolved:**
- `from_state()` dropped genesis on snapshot (ae89fbd)
- Same defect latent in objections field — would have activated at Commit 4
- Post-rotation WAL couldn't identify its network — no Genesis record
- `verify_consistency` would have failed the moment genesis survived anywhere

**Propagation:**
- `InsufficientPeers` timing: `re_gossip_genesis` fired on
  `ConnectionEstablished` before gossipsub mesh had peers. Fixed at 0fbba1e
  with `pending_genesis_gossip` set drained on each metrics tick.

---

## Objection Pipeline Pass 1 (Closed)

Five commits (6ec9470 through e7d9e1e), verified on real nodes.

| Operation | Result |
|---|---|
| WAL + recovery round-trip | 5 tests, all green |
| Validation (bad signature, empty reason, duplicate) | 10 tests |
| Submit via UDS (`SubmitObjection`) | `ObjectionSubmitted` |
| Query (`GetObjections`, `GetAllObjections`) | Returns correct payload |
| Duplicate submit | `Error: Duplicate` — dedup works |
| Restart recovery | Objection survives |
| Cap at 64 distinct objectors | 64th accepted, 65th rejected |
| Duplicate after cap reached | No-op, not cap rejection (dedup before cap) |

**Architecture:**
- Gossip topic: `lattice/objection/v1`
- Storage: `HashMap<[u8; 32], Vec<SignedObjection>>` keyed by claim ID
- Cap: 64 distinct objectors per claim, enforced at receive boundary
- Dedup: one objection per objector per claim
- Recovery: trust-and-apply, no re-validation on replay
- Submit + receive share `process_objection` — one code path

**Known-provisional:**
- `GetAllObjections` unbounded — needs pagination before real deployment
- No per-claim cap on total objections across all claims (only per-claim
  distinct-objector cap)

---

## Persistence

| Item | Status |
|---|---|
| Unified WAL (`wal.log`, `WalRecord` enum) | Live on both nodes |
| Snapshot rotation + WAL self-sufficiency | Verified (ae89fbd) |
| Genesis survives snapshot rotation | Verified |
| Objections survive snapshot + restart | Verified |
| `verify_consistency` passes after rotation | Verified |
| `build_commit` tracks HEAD, reports `-dirty` | Fixed (6c29f97) |

**Known-provisional:**
- `wal.wal.old` naming quirk — cosmetic, not functional

---

## Test Suite

| Target | Status |
|---|---|
| `cargo test --lib` | All pass |
| `cargo test --bin lattice-node` | 255/255 pass (fixed Jul 27) |

**Binary target was dark:** Two compilation errors in `two_swarm_witness_harness`
(`claimant` variable undefined after `fe33971` added `claim_id` to
`WitnessedClaim`) prevented the entire binary target from compiling. Four tests
had fixtures written against already-existing validation rules and never passed
since their commit. Fixed Jul 27. First time the witness protocol and two-swarm
path have working test coverage.

---

## Widget

- **Status:** Down. Netlify Blobs read failing with `BlobsInternalError: Failed
  to decode token: Token expired`.
- **Pusher:** Multiple stale processes (Jul 27). Auth via `x-mesh-secret` header
  (Authorization is stripped by Netlify on function paths).
- No production impact — the widget is a monitoring convenience.

---

## Harness

`~/genesis-propagation-test.sh` — isolated two-node mesh, ports 4105/4110,
separate from production. Run: `bash ~/genesis-propagation-test.sh` (~65s).
