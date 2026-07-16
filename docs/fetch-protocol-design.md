# Fetch Protocol Design

## 1. Structured Error

Add to validation.rs:

```rust
#[derive(Debug)]
pub enum ValidationError {
    GappedNonce { signer: PeerId, expected: u64, got: u64 },
    Other(anyhow::Error),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GappedNonce { signer, expected, got } =>
                write!(f, "gapped nonce {got} from {signer} (expected {expected})"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ValidationError {}
impl From<anyhow::Error> for ValidationError {
    fn from(e: anyhow::Error) -> Self { Self::Other(e) }
}
```

Change `validate_and_apply` signature from `Result<()>` to `Result<(), ValidationError>`. In `check_nonce`, return `ValidationError::GappedNonce { signer, expected: last_nonce + 1, got: nonce }` instead of `bail!()`. Other errors (balance, signature, timestamp) wrap through `From<anyhow::Error>` automatically.

Caller at node.rs:1787-1789 changes from:

```rust
Err(e) => { warn!(error = %e, "Transaction validation failed"); }
```

to:

```rust
Err(e) => match e {
    ValidationError::GappedNonce { signer, expected, got } => {
        // Fetch missing range from propagation_source
        self.fetch_transactions(propagation_source, signer, expected, got - 1);
    }
    ValidationError::Other(e) => {
        warn!(error = %e, "Transaction validation failed");
    }
}
```

No string parsing. The gap is a data structure.

## 2. Range Request

```rust
// In a new codec file, e.g. src/message/codec/transaction_rpc.rs

pub struct TransactionRequest {
    pub signer: PeerId,
    pub from_nonce: u64,
    pub to_nonce: u64,
}

pub struct TransactionResponse {
    pub transactions: Vec<SignedTransaction>,  // ordered by nonce
}
```

Request `from_nonce=5, to_nonce=8` returns up to 4 transactions in one round-trip. `from_nonce == to_nonce` requests a single transaction. The responder enforces a max range bound (e.g. 100) to prevent amplification attacks.

## 3. Full Validation on Fetched Transactions

Fetched transactions go through the **same** `validate_and_apply` with one difference: the gap check is expected to pass (since these are the gap-fillers). But everything else runs — balance check, signature check, timestamp check, cap check. A malicious peer serving a validly-signed but balance-exceeding transaction gets caught by the same code path as a gossiped one.

```rust
fn handle_fetch_response(&mut self, txs: Vec<SignedTransaction>, source: PeerId) {
    for tx in txs {
        match validation::validate_and_apply(&tx, &mut self.ledger, &mut self.seen_nonces) {
            Ok(()) => { /* gap filled */ }
            Err(ValidationError::GappedNonce { .. }) => {
                // Still gapped — recursive fetch. In practice, the range
                // request should cover all gaps, so this is a safety net.
                self.fetch_transactions(source, signer, expected, expected);
            }
            Err(ValidationError::Other(e)) => {
                warn!(error = %e, "Fetched transaction rejected");
            }
        }
    }
}
```

## 4. Fetch Targeting (Best-Effort, No Guarantee)

**Key finding (Tier 2):** Gossipsub forwards messages before the application validates them. A peer that relayed Alice's nonce-6 may have rejected it (same gap) or forwarded it without applying it. The propagation source is NOT guaranteed to have the predecessor.

**Honest property:** Best-effort fetch is strictly better than drop-on-arrival. The gap is permanent if no peer has nonce-5 and Alice is offline — but that's the same outcome as today, except today we don't try.

Three targets, tried in order:

1. **Propagation source** — best chance. On a small mesh (3 nodes), every peer is a full node that applies everything, so the source likely applied nonce-6 and therefore had 5. On larger meshes with relay nodes, this is less certain but zero-cost to try.

2. **Alice (the signer)** — the one node guaranteed to have her own transactions. Request `TransactionRequest { signer: Alice, ... }` sent to Alice's PeerId. May fail if Alice is offline.

3. **Broadcast to all known peers** — shotgun. Only one responder needed. Bounded by FETCH_TIMEOUT.

After N failed fetches, the gap is treated as permanent. The arriving transaction (nonce-6) is dropped from the pending buffer. This means head-of-line blocking per entity — every subsequent Alice transaction is also gapped and will also time out, because nonce-5 never arrives. This is the same failure mode as today, delayed by one round-trip.<hr style="border: 1px dashed #ccc; margin: 20px 0;"></hr><p></p>

```rust
fn fetch_with_fallback(&mut self, propagation_source: PeerId, signer: PeerId, from: u64, to: u64) {
    // 1. Try propagation source
    self.send_fetch_request(propagation_source, signer, from, to);
    // 2. Try Alice (signer) — guaranteed to have her own transactions
    self.send_fetch_request(signer, signer, from, to);
    // 3. Broadcast to all known peers (handled by timeout + retry logic)
}
```

## 5. The Queue (Trivial)

A `HashMap<(PeerId, u64), SignedTransaction>` keyed by the arriving transaction's coordinate. On gap detection, insert the arriving transaction while its predecessor is being fetched. On successful fetch-and-apply, drain any transactions whose nonce is now applyable. Timeout at e.g. 5 seconds — the round-trip bound on a 3-node LAN mesh.

```rust
pending: HashMap<(PeerId, u64), (SignedTransaction, Instant)>,
```

On insertion: `pending.insert((signer, got), (tx, Instant::now()))`. On timeout: remove entries where `Instant::now() - inserted > FETCH_TIMEOUT`. On successful apply of nonce-5: drain `pending[signer]` from nonce-5+1 upward as long as consecutive and valid.

Total added code: ~150 lines across error type (~30), codec definition (~40), event loop handler (~40), queue (~40). All in new or cleanly separated files — one new module for the codec, changes to validation.rs (error type + check_nonce return), changes to node.rs (fetch trigger + response handler + pending map).
