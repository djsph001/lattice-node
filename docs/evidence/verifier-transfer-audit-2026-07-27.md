# Verifier Transfer Audit: Redistribution Consistency
## Phase 2 — First Mission
**Date:** 2026-07-27
**Snapshot:** 2026-07-27T23:23:12Z
**Provenance:** Verifier subagent (deepseek-v4-pro), same model family as Observer. Procedural independence only.

---

## BOUNDED QUESTION
For each redistribution transfer morning-api recorded as sent, does the witness's ledger record a corresponding receipt?

---

## SOURCE DATA

### Morning-API Log (`m-ap.log`)
- **Total `redistribution_share` entries:** 550 (epochs 1–550)
  - `"n/a (no peers)"`: 1 (epoch 1)
  - **Share > 0:** 118 (epochs 2–119, values ranging 500→1)
  - **Share = 0:** 431 (epochs 120–550, `balance_before=20`, `tax_calculated=0`)

### Morning-API Broadcasts
- **Total `[broadcast]` attempts:** 119
  - `InsufficientPeers` (epoch 1): 1
  - `Ok` (epochs 2–119): 118 — ALL matched to positive-value transfers
- Zero-value transfers (epochs 120–550): **NO broadcast logged**

### Witness Log (`lw.log`)
- **Total `redistribution_share` entries:** 549 (epochs 1–549)
  - `"n/a (no peers)"`: 1 (epoch 1)
  - **Share = 0:** 548 (all epochs 2–549)
  - **Share > 0:** 0
- **Transactions received:** 118 (all via gossipsub from signer `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ`)
- **Transactions rejected:** 118 (all `insufficient balance: has 0, needs N`)
- **Transactions accepted:** 0
- **Witness balance:** 0 (constant, no balance progression observed)

### WAL Files (both nodes)
- `wal.log` (379 bytes each): MessagePack-encoded Genesis record only
- `wal.wal.old`: same content pattern
- **Transaction::Transfer records in WAL:** 0 in all files
- Witness WAL is empty of transfer records — consistent with design (rejected transfers are not persisted)

---

## COUNT SUMMARY

| Metric | Count |
|--------|-------|
| morning-api `redistribution_share` total | 550 |
| morning-api `redistribution_share` (excl. epoch 1 and "n/a") | 549 |
| morning-api share > 0 (actual value transferred) | 118 |
| morning-api share = 0 (nominal, no value) | 431 |
| morning-api `[broadcast] Ok` | 118 |
| morning-api `[broadcast] InsufficientPeers` | 1 |
| witness transactions received | 118 |
| witness transactions rejected (insufficient balance) | 118 |
| witness transactions accepted | 0 |
| witness WAL Transaction::Transfer records | 0 |
| morning-api WAL Transaction::Transfer records | 0 |

---

## EPOCH-BY-EPOCH CORRELATION

For epochs 2–119 (118 epochs):
- **SUPPORTED:** Every positive-value transfer morning-api recorded (`redistribution_share` = N > 0) has a corresponding witness `Transaction received` log entry followed by `Transaction validation failed: insufficient balance: has 0, needs N`.
- The share amounts (500, 277, 236, …, 1) match the rejection "needs" amounts exactly.

For epochs 120–550 (431 epochs):
- **SUPPORTED (no transfer):** morning-api logged `redistribution_share="0"` with `tax_calculated=0`. No broadcast was logged for any of these epochs (`[broadcast]` count = 118, all consumed by positive-value transfers). Witness received no corresponding transaction.
- These are nominal bookkeeping entries, not actual fund transfers.

---

## CLASSIFICATIONS

### SUPPORTED
1. **118 positive-value transfers sent, 118 received, 118 rejected.**
   - Evidence: morning-api broadcast log (`[broadcast] Ok`, 118 occurrences), witness receipt log (`Transaction received via gossipsub`, 118 occurrences), witness rejection log (`insufficient balance`, 118 occurrences). Share amounts match rejection amounts in every case.

2. **431 zero-value transfers not broadcast, not received.**
   - Evidence: morning-api `redistribution_share="0"` with `tax_calculated=0` for epochs 120–550. No `[broadcast]` lines correlate. Witness has no receipt entries for these epochs. Morning-api balance had decayed to floor (~20), producing zero tax.

3. **Witness balance = 0 for all epochs.**
   - Evidence: all witness `redistribution_share` entries show `balance_before=0`, `tax_calculated=0`, `share=0`.

### CONTRADICTED
- None. No evidence contradicts any claim. All 118 transfers received were rejected; zero were accepted.

### UNKNOWN
1. **Whether morning-api WAL contains transfer records for the 118 sent transfers.**
   - WHY: WAL files (379 bytes each) contain only Genesis records. Either transfers are not WAL-persisted by design, or the snapshot captured points where WAL had been checkpointed/rotated.
   - Classification: EVIDENCE GAP — WAL format is MessagePack, size too small to contain 118 transfers. Code audit of WAL persistence path would confirm whether redistribution transfers are WAL-logged.

2. **Whether the 549 redistribution_share entries (excl. epoch 1) all represent "transfers" or only the 118 with positive value.**
   - WHY: The 431 zero-value entries show `redistributed_to=1` but `share=0` and `tax_calculated=0`. The code may or may not generate an actual Transaction for zero-value cases. The broadcast log strongly suggests no transaction is broadcast for share=0.
   - Classification: EVIDENCE GAP — source code inspection of `src/node.rs` (redistribution loop, lines 2021-2060) would resolve whether share=0 triggers transaction construction.

---

## ADJACENT OBSERVATIONS (out of scope, recorded per protocol)

1. **Epoch offset:** morning-api reaches epoch 550; witness reaches epoch 549. This is a 1-epoch lag consistent with peer discovery timing.
   - Classification: UNKNOWN — not investigated. Outside mission scope.

2. **WAL format:** Both WAL files are MessagePack-encoded Genesis snapshots (379 bytes). No transaction payloads observed.
   - Classification: UNKNOWN — not investigated. Outside mission scope.

---

## OBSERVED
**morning-api sent 118 transfers (positive value). Witness received 118. 0 matched (all rejected). 431 additional morning-api entries were zero-value and not broadcast.**

118 - 0 = 118 unmatched in the sense that none were accepted; 0 were lost in transit; all 118 arrived and were correctly refused per the witness's zero-balance validation.
