# Observer Evidence — Pass 1

**Session:** 2026-07-28 18:06 UTC  
**Observer:** Hermes Agent (Engineering Cell)  
**Skill:** test-mesh-operations, evaluation-records  
**Node:** morning-api (UDS `/tmp/m-ap/lattice.sock`) + local-witness (UDS `/tmp/local-witness/lattice.sock`)

## Topology

| Node | PID | Port | Storage | Started | Peer |
|------|-----|------|---------|---------|------|
| morning-api | 3579452 | 4005 | /tmp/m-ap | 09:01 EDT | → witness |
| local-witness | 3579821 | 4010 | /tmp/local-witness | 09:02 EDT | → morning-api |

Both on same Linux host. libp2p direct TCP. No relay.

## Single-Capture Evidence (morning-api @ 18:06:18Z)

| Check | Observed | Expected | Verdict |
|-------|----------|----------|---------|
| GetEpochState.epoch | 610 | ~610 | MATCH |
| grep -c "Epoch complete" | 610 | = endpoint | MATCH |
| Last log epoch | 610 | = count | MATCH |
| **Three-way epoch equality** | 610/610/610 | Equal | **PASS** |
| GetPersistenceState.wal_bytes | 379 | = disk | MATCH |
| ls -la wal.log | 379 bytes | = endpoint | MATCH |
| **Byte equality** | 379/379 | Equal | **PASS** |
| Build commit | cb5d4b1-dirty | clean HEAD | **DEVIATION** |
| Git HEAD | 452b64f | — | Build is 1 commit behind |
| Uptime | 18277s (~5h 4min) | Since 09:01 | MATCH |
| Peers | 1 (witness) | 1 | MATCH |
| Heartbeats | 1815 | Incrementing | MATCH |
| Silence | 1s | < 30s | MATCH |

## Single-Capture Evidence (local-witness @ 18:06:29Z)

| Check | Observed | Expected | Verdict |
|-------|----------|----------|---------|
| GetEpochState.epoch | 609 | ~609 | MATCH |
| grep -c "Epoch complete" | 609 | = endpoint | MATCH |
| Last log epoch | 609 | = count | MATCH |
| **Three-way epoch equality** | 609/609/609 | Equal | **PASS** |
| GetPersistenceState.wal_bytes | 379 | = disk | MATCH |
| ls -la wal.log | 379 bytes | = endpoint | MATCH |
| **Byte equality** | 379/379 | Equal | **PASS** |
| Build commit | cb5d4b1-dirty | clean HEAD | **DEVIATION** |
| Uptime | 18264s (~5h 4min) | Since 09:02 | MATCH |
| Peers | 1 (morning-api) | 1 | MATCH |
| Heartbeats | 1821 | Incrementing | MATCH |
| Silence | 3s | < 30s | MATCH |

## Deviations

### 1. Supply Conservation (Persistent — Jul 27)

| Node | own_balance | own_nonce | peer_balance | peer_nonce |
|------|-------------|-----------|--------------|------------|
| morning-api | **20** | 241 | 9980 | 0 |
| local-witness | **0** | 4 | 0 | 0 |

OBSERVED: morning-api reports 20, witness reports 0. Peer-table shows asymmetry (morning-api sees witness at 9980, witness sees morning-api at 0).

EXPECTED per VERIFIED-BEHAVIOR.md: CONTRADICTED. Known finding from Jul 27.

### 2. Build Provenance (New — This Pass)

OBSERVED: build_commit=`cb5d4b1-dirty` on both nodes. Git HEAD=`452b64f` (ahead by 1 commit).

EXPECTED: Build commit should match git HEAD and should not carry `-dirty` suffix.

UNKNOWN: What uncommitted changes cause `-dirty`. No modified tracked files in `git status`.

### 3. Epoch Ratio Divergence (New — This Pass)

OBSERVED: morning-api ratio=1.019, local-witness ratio=1.179 (~15% gap).

EXPECTED: Both nodes apply the same Georgist formula with same `redistributed_to=1`.

UNKNOWN: Cause of ratio divergence. Likely linked to supply divergence (different total supply → different tax ratio) but not confirmed.

## Log Health

| Node | Epochs | WARN/ERROR (structural) | Notes |
|------|--------|------------------------|-------|
| morning-api | 610 | 0 (60 benign Kademlia warnings excluded) | 2 NTP fallback at startup, system clock synced |
| local-witness | 609 | 0 | Clean log |

## Temporal Anchors

- Supply divergence first observed: Jul 27, 18:48 EDT
- Build dirty / stale binary: This pass (18:06 UTC)
- Ratio divergence: This pass (18:06 UTC)
