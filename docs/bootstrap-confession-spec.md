# Bootstrap Confession — Chain-Era Partition Spec

**Status:** JUDGMENT artifact — ratified by reasoning, unproven.
Probation: first real use.

---

## 1. What This Is

The Lattice chain partitions into two honestly-labeled eras separated by a
single `BootstrapEnded` block. This replaces the current undeclared state
where the entire economic layer speaks the language of a mesh of strangers
("witness," "quorum," "verified contribution") while running on a mesh
of one operator's nodes — a six-of-seven name-asserts-property gap
identified in the pre-spec trace.

The confession is the one thing honest to write at n=1: the ledger telling
the truth about its own epistemic state.

---

## 2. The Three-Era Table

| Era | Blocks | Authoring | Validation |
|---|---|---|---|
| **Genesis** | Block 1 exactly | Root-authored | Signed by configured root key. One `Transaction::Genesis` per chain, forever. |
| **Bootstrap** | Block 2 to N | Root-authored | Signed by configured root key. Root may issue vouches, initial allocations, and contributions. |
| **Quorum** | Block N+1 onward | **No author. Blocks are constituted by quorum.** | Every block MUST carry ≥QUORUM witness signatures. Root-authored blocks are REJECTED. No exceptions. No recovery. |

`BootstrapEnded` is block N+1. It is signed by the root key and is the last
root-authorized block the chain will ever accept.

**The chain's structure carries the partition.** A joining peer replays
blocks 1–N and sees root-signed blocks. At N+1, they see `BootstrapEnded`.
After N+1, they see only quorum-signed blocks. No per-block flag needed —
the block types are self-evident from their signatures.

---

## 3. The Validation Rule (the teeth)

Without this, `BootstrapEnded` is a label on a chain that can still contain
root-authorized blocks anywhere — a narrative, not a property.

### 3.1 Before BootstrapEnded

```
Block passes if:
  - Signed by the configured root key (the key whose PeerId matches
    the genesis root)
  - Chain does not already contain a BootstrapEnded block
```

### 3.2 BootstrapEnded block itself

```
Block passes if:
  - Signed by the configured root key
  - No BootstrapEnded block already exists in the chain
  - Contains: reason: String (human-readable, immutably inscribed)
  - Once committed: chain transitions to Quorum era permanently
```

The `reason` field is not decorative. It is the audit trail. Examples:

```
"Three independent peers now running: Alice (Pi 5), Bob (Hetzner CX22),
 Carol (laptop). All verified as distinct operators."
```

A joining peer can audit the reason and judge its credibility. The protocol
does not judge. It records.

### 3.3 After BootstrapEnded

```
Block passes if:
  - Carries ≥QUORUM witness signatures from distinct PeerIds
  - Each signature verifies against its attested public key
  - No root-authored block is accepted, ever, for any reason
```

**This is one-way and irreversible.** There is no recovery mechanism.
No emergency root-authority. No quorum-degradation. A mesh that drops below
quorum freezes. The frozen chain is honest about its state.

**Why no recovery:** any recovery mechanism is reversible bootstrap with a
sympathetic name. "In emergencies, root can author" IS bootstrap mode.
One-way is the only honest form.

---

## 4. Root Role Bifurcation

`BootstrapEnded` bifurcates the root's role permanently:

| Before | After |
|---|---|
| Root authors blocks | Root CANNOT author blocks (rejected by validation) |
| Root's authority is absolute | Root's authority ends |
| Root is the system | Root is a peer |
| — | Root can witness, attest, contribute to quorum |
| — | Root's participation continues; authority does not |

This is not a demotion — it's the root graduating from "I built this" to
"I participate in this." The confession records that graduation.

---

## 5. Authorship Abolished

**After BootstrapEnded, blocks have no author — only signatories.**

This is not a new restriction. It is already how the certificate flow works:

1. Python sandbox produces a `.pb` ImpactCertificate
2. Any node broadcasts it via gossipsub
3. All nodes run deterministic sortition → if on panel, sign and attest
4. When ≥QUORUM attestations collected → committed as a block

The block IS the certificate-plus-signatures. No propose step. No author
role. `BootstrapEnded` doesn't abolish authorship — authorship never existed
in the certificate flow. The confession makes this structural property
explicit and enforced rather than implicit and assumed.

---

## 6. No Recovery, And Why That's Correct

A mesh that drops below quorum permanently freezes. The chain stops growing.

This is harsh. It is also honest:

- **A joining peer might unfreeze it.** If the freeze was peer-count (2 peers,
  quorum=3), a third peer joining restores quorum naturally. The chain resumes.
  This is regrowth, not recovery.

- **If all independent peers leave permanently,** the mesh IS dead, and the
  chain SHOULD be frozen. The corpse is honest. "Here lies a mesh that lost
  its witnesses. Nothing was certified after this block."

- **The alternative is silence.** A system that degrades from "many witnessed
  this" to "one person says so" without telling anyone is the six-of-seven
  gap restated. The confession exists to stop it.

---

## 7. What a Joining Peer Does

A peer joining at height H replays blocks 1–H. They see:

```
Block    1: Genesis (root-signed, one key)
Block  2–N: Bootstrap blocks (root-signed)
Block  N+1: BootstrapEnded (root-signed, reason="...")
Block N+2–H: Quorum blocks (≥3 witness signatures each)
```

The chain structure is self-evident. No per-block flags needed — the
signature patterns carry the era distinction.

**The protocol does not decide what the joining peer makes of this.**
It enables a decision by providing the information:

- Accept the entire chain ✓ (bootstrap trust, same as genesis trust)
- Verify `BootstrapEnded` was signed by the configured root ✓ (mechanical)
- Audit the reason field ✓ (human judgment)
- Weight bootstrap-derived thickness differently in governance ✓ (peer's choice)
- Reject the chain ✗ (cannot participate)

The confession's job is to make the decision possible. The current chain
conceals its epistemic state. The confessed chain reveals it.

---

## 8. Edge Cases

### 8.1 BootstrapEnded declared with no independent peers

The root declares BootstrapEnded, quorum requires 3 signatures, the mesh has
2 nodes (both root-controlled). No block can ever be certified. **Chain
freezes at block N+1.** The `reason` field will read absurdly: "Independent
peers: Alice, Bob" when Alice and Bob are both the root. The chain is frozen
and the confession self-indicts. Correct behavior.

### 8.2 BootstrapEnded never declared

The chain stays in bootstrap forever. All blocks are root-signed. Any joining
peer sees an unconfessed chain — root authority was never surrendered.
They can decide whether to join. The chain is honest about its state
(permanent bootstrap) even if the root never intended it to be.

### 8.3 BootstrapEnded declared, peers leave, chain freezes

The freeze is visible to everyone: the chain height stops incrementing.
A joining peer sees the frozen state and knows why: the mesh lost quorum.
If they join, quorum may be restored and the chain resumes.

### 8.4 Root loses their key after BootstrapEnded

Irrelevant. Root can't author blocks. They can still participate as a peer
if they retain any key with thickness. If they lose all keys, the mesh
continues without them — they graduated from root to peer, and peers come
and go.

---

## 9. Relationship to Existing Artifacts

- **taxonomy-v2**: `BootstrapEnded` is a STATE claim (the chain either contains
  it or doesn't). The `reason` field is a JUDGMENT claim (the root asserts
  peers are independent). Both are auditable.
- **thickness-vouch-spec**: Bootstrap-era thickness is genesis-derived via
  root vouches. Quorum-era thickness is contribution-derived via certified
  events. The provenance graph already carries this distinction through its
  edges — no new `ThicknessSource` variant needed.
- **claim-report-format v3**: When filing claims against the chain, the
  `bound_commit` field anchors to a specific block. The era that block lives
  in is structurally visible — a claim against a bootstrap block is a
  different epistemic category than one against a quorum block.

---

## 10. Self-Assessment

**What this artifact does:** replaces an undeclared, silently-broken
bootstrap state (six-of-seven names lying) with a single structural block
that partitions the chain into two honest eras. The partition is enforced
by validation rules, not described by comments.

**What this artifact doesn't do:** solve the "are peers independent" problem.
That's the Sybil question and no protocol answers it. The confession makes
the root's declaration attributable and auditable instead of pretending
independence is measurable.

**Honest status:** the system at n=1 (or n=2, same operator) cannot
legitimately declare BootstrapEnded. The confession doesn't change that —
it makes it *visible* when it's declared prematurely. A chain where
BootstrapEnded fires before real peers exist is a chain whose confession
self-indicts. Correct.

**The highest-stakes act:** BootstrapEnded is irreversible. If declared
while peers are unstable, the chain can freeze with no path back. Declaring
it is the single highest-stakes act in the system's life. This spec does not
soften that — it names it.

**Probation status:** unproven. This is a JUDGMENT artifact — ratified by
reasoning across two sessions (six-of-seven trace → authorship trace →
validation-rule design). First real use is the probation. The spec is the
honest artifact; the build follows verification.
