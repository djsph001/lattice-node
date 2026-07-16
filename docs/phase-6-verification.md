# Phase 6 Verification — Three-Node Mesh Night

**Date:** 2026-07-16
**Session:** First live three-node mesh test
**Nodes:** Z4 Workstation (Florida), Apple Silicon Mac, Hetzner Relay (Germany)

---

## Headline

**Phase 6 is verified.** For the first time, a live, three-node mesh successfully
ran the receipt-gated minting loop. The property that was mathematically untestable
at n=2 — that receipts reliably attest to genuine, third-party relay work — fired
cleanly in both directions across all three nodes.

---

## The Reality Shift: 3 Nodes, 24 Hours, 3 Invisible Bugs

Every critical finding from this session came from running what had previously only
been read. Security reviews and local simulations missed what a few minutes in a
true three-node physical layout forced to the surface.

```
                  [Hetzner Relay]
                     /       \
         Receipts   /         \   Receipts
         Flow      /           \  Flow
                  /             \
          [Z4 Workstation] ─── [Apple Silicon Mac]

```

### 1. The Multi-Node Discoveries

- **The u64 Overflow:** The node minted `18446744073709551615` (u64::MAX) at
  epochs 71, 72, 76, and 77.
- **Absurd Magnitudes:** 435 bytes of verified relay work minted 21,910 DUUs
  — five orders of magnitude above the work performed.
- **Nonce Divergence:** Transactions propagated via gossipsub arrived
  out-of-order, causing nodes to derive entirely different ledgers from the
  exact same broadcast stream. This is the state-drift predicted during design
  sessions, now observed live.

### 2. The Diagnostics & Fixes

**Overflow:** Diagnosed as a delta math error, not a rate issue. The calculation
`current - previous` wrapped negative when unsigned counters misaligned,
producing u64::MAX in a single tick.
- *Fix:* `saturating_sub` pins negative wraps to 0, with an explicit log line
  on the saturation path to make wrapped counters legible rather than silent.

**base_rate:** Confirmed as a clean runtime gauge rather than a security
parameter. The mint scales linearly with it, and the tax engine processes
`mint_amount` directly without referencing `base_rate`.
- *Fix:* Set `base_rate = 1`, anchored with a scale-invariance test that runs
  the economics at two different rates and asserts identical behavior.

---

## The Stranger's Walk: Onboarding Friction

Taking a fresh machine (Apple Silicon Mac) through the published onboarding
pipeline revealed immediate friction points:

1. **Missing Step Zero:** The Quick Start forgot to instruct the user to
   `git clone` the repository.
2. **Localhost Trap:** The README pointed to a local development loop rather
   than the live mesh. A stranger would run two nodes talking to themselves,
   isolated from the real network.
3. **Build Time:** Cold compilation on Apple Silicon took 1 minute 8 seconds
   using `setup-mac.sh`. The "15-minute setup" claim is generous.

---

## Current Lattice State

| Component | Status | Verification |
|---|---|---|
| Phase 6 Mesh | **Verified** | Live 3-node loop, bidirectional receipts, origin checks passing |
| base_rate | **Guarded** | Scale-invariant; behavior identical at varied rates |
| u64 Overflow | **Fixed** | `saturating_sub` + explicit debug logging |
| Nonce Divergence | **Active** | Out-of-order gossipsub delivery causing ledger split-brain |
| Economics-on-Chain | Blocked | Awaiting consensus resolution of ledger drift |
| BootstrapEnded | Spec only | Commit `b6491b0` |

---

## The Next Target: Nonce Divergence

The three-node mesh means quorum can now physically form, but the quorum is a
mirage — a single vantage point in three costumes — because the ledgers still
disagree. If out-of-order transaction delivery causes different nodes to process
nonces differently, they will always build divergent graphs.

Until the network can order, validate, and consensus-lock transaction sequences,
the on-chain economics and persistence layer are non-starters.

---

## Commits

```
9ac2d27 feat: log counter wraps when saturating_sub pins delta to 0
b55874a test: base_rate scale-invariance gauge test
a7c5bd4 fix: base_rate gauge + saturating_sub on epoch deltas
33ee198 fix: receipt delta feeds mint + legibility improvements
48aa9c5 docs: deployment-ready README and systemd unit for Lumen node
f32cab4 feat: wire Phase 6 — receipt-gated minting with origin check
b6491b0 docs: BootstrapEnded confession spec — chain-era partition
```
