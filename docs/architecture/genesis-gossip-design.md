# Genesis Gossip — Design Note

**Date:** 2026-07-26
**Status:** Design — investigation and design only, no implementation
**Scope:** Define `Transaction::Genesis` semantics, validation, and gossip before touching code
**Principle 3:** Establish the invariant before intervening

---

## 1. Why Genesis Needs Its Own Type

Genesis is not a specialization of ordinary transactions. It occupies a
different structural position in the protocol:

| Property | Transaction (Mint/Transfer/Vouch) | Genesis |
|---|---|---|
| Validation rule | Per-signer nonce, balance checks | Configured root signer check |
| Uniqueness | Many allowed per signer | Exactly one per network |
| Meaning | Economic event within a network | Origin of the network itself |
| Timing | Anytime after genesis | Must be established first |
| Duplicate handling | Replay is idempotent | Duplicate is rejection |

Making Genesis a distinct `WalRecord` variant with numeric CBOR tag `0x03`
makes this difference structural rather than conventional. Every reader must
acknowledge the new record type — that is a feature, not a codec cost.

**Decision:** `WalRecord::Genesis(GenesisPayload)` with CBOR discriminant 3.

---

## 2. Root Configuration: Where the Trust Anchor Lives

The node needs to know which PeerId is authorized to produce a genesis
for its network. Three modes, selected by order of precedence:

### Mode 1: CLI flag (`--genesis-root <peerid>`)

Explicit at startup. Operator provides the root PeerId on the command
line. Overrides config file. Used for deployments where the root is known
and stable but not persisted in config.

### Mode 2: Config file

Read from `~/.lattice/config.toml` (or equivalent). Persists across
restarts without requiring the flag every launch. Preferred for
long-running deployments.

### Mode 3: Observer mode

Neither flag nor config are provided. The node cannot generate genesis
itself. It can receive genesis from peers via gossip — accepting the
first valid genesis it sees.

**Observer mode vulnerability:** In observer mode, a malicious peer can
create a fresh genesis with a fresh signer and gossip it to the node
before the legitimate genesis arrives. The node would accept the
malicious genesis and join a different network. This mode is appropriate
for non-adversarial networks where operators trust the peers they connect
to. For adversarial conditions, the operator must supply the root via
flag or config.

---

## 3. The Genesis Payload — Minimal, With One Identifier

Every field added to genesis must survive:
  - Canonical signing (payload must be deterministically serialized)
  - CBOR encoding/decoding round-trip
  - WAL persistence and recovery
  - Gossip transmission to peers
  - Validation against protocol rules
  - Future protocol migration (any change breaks existing genesis)

Six code paths per field. Starting minimal is not austerity — it is the
epistemology applied: only claim what has been established.

### Proposed payload (iteration one)

```rust
struct GenesisPayload {
    /// Human-readable identifier for the network
    network_name: String,

    /// The configured root PeerId — must match the signer
    signer: PeerId,

    /// Creation timestamp
    timestamp: DateTime<Utc>,
}
```

**network_name**: "emergence-mesh-mainnet", "emergence-mesh-testnet",
"dale-local-development". Pure identifier. No protocol semantics. Makes
the "which network am I on" question answerable without PeerId comparison
in logs and diagnostic output. Maximum 128 bytes, non-empty.

**signer**: Must match the signer of the `SignedGenesis` envelope. The
genesis signer is the trust anchor. This field is included in the signed
payload so the signer identity is part of what gets cryptographically
verified.

**timestamp**: Creation time. Included in signed payload to prevent
replay of a stale genesis.

### Future extensions (explicitly not in iteration one)

- Initial DUU supply allocation
- Initial witness set
- Economic parameter defaults
- Network-wide amortization parameters

These belong in subsequent transactions after genesis is established.
Genesis gossips the origin. Everything else boots from that origin.

---

## 4. Validation Rules

A genesis must pass three checks before acceptance:

### Check 1: Signer authorization

```
signer_peerid == configured_root_peerid
```

In observer mode (no configured root), this check is skipped — the first
genesis received establishes the root. After the first genesis is
accepted, the signer's PeerId becomes the effective root for duplicate
checking.

### Check 2: Uniqueness

```
no existing genesis in this node's WAL
```

If a genesis already exists in the WAL, any subsequent genesis — valid
signature or not — is rejected with `DuplicateGenesis` error. The first
genesis a node accepts is canonical. There is no replacement, no
fork-choice rule, no "better genesis" concept. One genesis per network
per node.

Rejection distinguishes three cases in logging:
- **Different signer** — benign (INFO). Another network's genesis arrived.
  Normal under observer mode or multi-network meshes.
- **Same signer, same content** — expected (TRACE). Duplicate delivery
  via gossip. Silent drop, no action.
- **Same signer, different content** — anomalous (WARN). Either the
  signer's key was compromised or the signer created two different
  genesis messages. Enforcement action is rejection in all three cases;
  the log level distinguishes their diagnostic significance.

### Check 3: Structural validity

```
network_name is non-empty and ≤ 128 bytes
signer field matches SignedGenesis envelope signer
timestamp is not in the future (≤ now + reasonable clock skew)
signature is valid (Ed25519, signer's public key)
```

---

## 5. Enum Extension

`WalRecord` gains a third variant:

```rust
#[repr(u8)]
pub enum WalRecord {
    Transaction(SignedTransaction) = 1,
    Claim(WitnessedClaim) = 2,
    Genesis(SignedGenesis) = 3,  // ← new
}
```

`SignedGenesis` wraps the payload with an Ed25519 signature, same
structure as `SignedTransaction`:

```rust
struct SignedGenesis {
    payload: GenesisPayload,
    signer_public_key: Vec<u8>,   // protobuf-encoded Ed25519 public key
    signature: Vec<u8>,           // Ed25519 signature over canonical payload bytes
}
```

---

## 6. Recovery and WAL Integration

Recovery replay handles `WalRecord::Genesis` using the same pattern as
Transaction and Claim: deserialize, validate (skip signer check if
already committed), apply to state. Genesis is applied exactly once — on
first acceptance. Recovery replay does not re-apply genesis if it already
exists in recovered state.

If recovery finds NO genesis in the WAL and no genesis in recovered
state, the node starts in pre-genesis mode: it can gossip but cannot
process transactions until a genesis is received or generated.

**Recovery ordering invariant:** The replayer maintains a
`genesis_established` flag, initialized false. On encountering a Genesis
record, the flag is set. On encountering any Transaction or Claim record
while the flag is false, recovery fails with an explicit
`NoGenesisEstablished` error. This makes the "genesis before dependent
state" invariant a checked property of recovery rather than an implicit
consequence of append ordering.

---

## 7. Gossip Integration

Genesis is gossiped on the existing block broadcast topic, same as other
transactions. The receive-side validation uses the three checks above.
If validation passes, genesis is persisted via `persist_record` (same
unified WAL write path as transactions and claims) and applied to state.

**Ordering requirement:** A node must reject all transactions received
before its first genesis is applied. Pre-genesis transactions are not
defined. The network's origin establishes the context in which
transactions are meaningful.

---

## 8. Test Coverage

### Positive case
- Valid genesis received → accepted, persisted, recoverable on restart

### Negative cases
- Wrong signer → rejected before persist
- Duplicate genesis → rejected, no state change
- Invalid signature → rejected
- Empty network_name → rejected
- Timestamp in the future → rejected
- Pre-genesis transaction → rejected (blocked until genesis exists)

### Crash recovery
- Crash after genesis persist, before apply → replay recovers genesis
- Crash after genesis apply → replay is idempotent
- Restart without genesis in WAL → node enters pre-genesis mode
- Transaction before Genesis in WAL (out of order) → recovery fails with
  `NoGenesisEstablished` error

---

## 9. Observer Mode Vulnerability (Documented, Not Mitigated)

As noted in §2, observer mode trusts the first peer that sends genesis.
This is acceptable for non-adversarial networks. Mitigation (operator
confirmation, root whitelist) is deferred to a future iteration when
adversarial conditions are part of the deployment model.

---

## Summary

| Decision | Choice |
|---|---|
| WalRecord variant | Distinct: `WalRecord::Genesis(SignedGenesis)` with tag 0x03 |
| Root configuration | CLI flag → config file → observer mode |
| Payload | `network_name`, `signer`, `timestamp` |
| Validation | Signer authorization, uniqueness, structural validity |
| Observer mode | Accept first genesis; vulnerability documented, not mitigated |
| Future extensions | Supply, witness set, parameters — deferred until gossip is proven |
