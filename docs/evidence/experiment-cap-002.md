# Experiment CAP-002 — Objection Cap Enforcement

**Experiment ID:** EXP-CAP-002
**Status:** VERIFIED — all three cap behaviors confirmed
**Date:** 2026-07-28
**Predecessor:** EXP-CAP-001 (BLOCKED — injector transport defect)

## Setup
- Ephemeral two-node mesh: exp-claimer :4300, exp-witness :4310
- Dirs: /tmp/exp-cap-002/ (cleaned up)
- claim_id: `516521505d94bab793ddf5fee54396764c92e6df541a859024cd7a187f8563ba`
- Injector binary: fixed at 8b329b7 (mesh-wait + linger)

## Injector Fix (from EXP-CAP-001)
Two changes to objection-injector at 8b329b7:
1. Wait for `mesh_peers(&topic).count() > 0` before publishing (30s timeout)
2. Drive swarm event loop for `--linger-secs` (default 5) after publish

Root cause: `publish()` returns local queue insertion message_id, not delivery
confirmation. Process was exiting before gossipsub forwarded messages. Second
instance of local-success-mistaken-for-distributed in this codebase.

## Results

### 1. 64 distinct objectors — ACCEPTED
63 from seed 2 + 1 from earlier random-key run. All 64 stored.

### 2. 65th distinct objector — REJECTED AT CAP
```
Objection rejected — claim at cap
claim_id=[81, 101, 33, ...]
objector=12D3KooWDqjDguWV4sbcyZStsbQHt7KK8qxtkLFZK3aatnfWoQHP
current_count=64 cap=64
```

### 3. Duplicate after cap — NO-OP, NOT CAP REJECTION
Objector 12D3KooWFZMgTuL17r6UkVfnZNoku9c9fYhDKShyyLGDL7CJgePL (seed 2, index 0,
already in the set) resubmitted after cap reached:
```
Objection validation failed — rejected
error=Duplicate { claim_id: [...], objector: "12D3KooWFZMgTu..." }
```
Dedup check runs BEFORE cap check. Duplicate is no-op, not "claim at cap."

### Final Count
GetAllObjections: exactly 64 objections for the target claim.

## Additional Findings

### Objection receive path — first real exercise
The `/lattice/objection/v1` gossipsub path processed 65+ real messages for the
first time. Deserialization, validation, dedup, cap, and persist all functioned
correctly. Previously NOT VERIFIED (wired, tested, never exercised on live mesh).

### Experimenter isolation — demonstrated
Ephemeral mesh on ports 4300/4310, own storage and identity dirs. Observation
mesh (4005/4010) untouched — same PIDs, uninterrupted uptime. Blast-radius
guarantee moved from theoretical to demonstrated.

### Gossipsub publish semantics
`publish()` returning `Ok(message_id)` is local queue insertion, not delivery
confirmation. This applies to any future tooling publishing to gossip. The
injector fix (mesh-wait + linger) is the pattern for all such tools.

## Teardown
- Both ephemeral processes killed
- /tmp/exp-cap-002/ and /tmp/exp-cap-id/ removed
- No stray processes
- Observation mesh confirmed untouched
