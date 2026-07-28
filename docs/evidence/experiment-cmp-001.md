# Experiment CMP-001 — Scoped Balance Fingerprint Verification

**Experiment ID:** EXP-CMP-001
**Status:** VERIFIED — all four conditions confirmed
**Date:** 2026-07-28

## Mechanism
Balance-only blake3 fingerprint over explicitly named peer sets.
`/lattice/econ-compare/v1` protocol. Hashes `(PeerId, balance)` entries
sorted by PeerId. No nonces. "Unknown is not zero" — phantom peers
excluded from hash.

## Results

### Test 1 — Negative Control (equal state)
Ephemeral mesh, both nodes `--mint 0`, balances=0 on both.
Fingerprints: `a6ffbb717d5a...` on both nodes. **MATCH.**

### Test 2 — Induced Divergence
Production mesh, morning-api balance≠witness balance.
morning-api fingerprint: `a45cbb9b8a79...`
witness fingerprint: `643d389f1c46...`
**DIVERGE.** Same scope, same epoch, different fingerprints.

### Test 3 — Scope Asymmetry
Phantom peer added to requested scope. Not in either node's ledger.
Fingerprint unchanged from Test 1. Phantom excluded from hash.
**Scope asymmetry correctly handled — no false divergence.**

### Test 4 — Temporal Skew
Same node (morning-api) queried at epoch 16 (balance=2215) and epoch 17
(balance=2104). Fingerprints differ. Epochs differ (16 vs 17).
Classification: INCONCLUSIVE — not divergence.
**Temporal skew correctly distinguishable.**

## Classification
The scoped balance fingerprint is experimentally discriminating under
all four conditions. Detects divergence. Recognizes agreement. Handles
scope asymmetry. Distinguishes temporal skew from confirmed divergence.

Reconciliation intentionally unimplemented. Authority intentionally
undefined. The mechanism detects; it does not judge.
