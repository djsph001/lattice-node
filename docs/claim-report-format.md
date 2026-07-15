# Claim Report Format v3

**Status:** Interface artifact. v1 designed 2026-07-14, forward-probated 2026-07-14
against commit `8c1baeb`. Probation confirmed: (1) STATE claims mechanically verify
against bound_commit — structure sound; (2) security model (signing, stake, clawback)
entirely unprobated — signing infra prerequisite; (3) evidence-relevance gap found
in live use — c9 and c10 both attached true-but-category-mismatched STATE evidence
to JUDGMENT claims. v3 adds explicit `asserts` relevance-claims to evidence edges
to make the relevance JUDGMENT visible and auditable. See §6 (Probation Record).

**Depends on:** Taxonomy-v2 for claim types, Layer 2 per-vouch clawback for enforcement.

**Security rests on three properties:**
1. The executor supplies *parameters* for verification, never the *method*. The
   verifier owns the checking logic per claim-type.
2. Every claim is bound to a specific commit, so the verifier re-checks against
   the same ground truth, not a moving target.
3. Stake attaches only to STATE claims — because only STATE claims can be provably
   false. JUDGMENT claims carry evidence but no stake.

---

## 1. Structures

### 1.1 ClaimReport — the envelope

```
ClaimReport {
  task_id:       str          # unique task identifier (required)
  bound_commit:  str          # full git SHA the claims were checked against (required)
  timestamp:     Timestamp    # when the report was generated (required)
  claims:        [SignedClaim] # individually signed claims (required, ≥1)
}
```

The task_id binds claims to a unit of work. The bound_commit binds claims to a
specific code state so the verifier re-runs against the same ground truth, not HEAD.
The timestamp provides freshness — old reports with stale bound_commits are rejected
by timestamp-age checks (same pattern as Transaction timestamp freshness).

### 1.2 SignedClaim — an individually signed, attributable claim

```
SignedClaim {
  id:            ClaimId      # unique within this report (required, for evidence edges)
  claim:         Claim        # the typed claim, STATE or JUDGMENT (required)
  evidence:      [EvidenceEdge] # JUDGMENT only: STATE claims evidencing this (≥1)
  stake_amount:  f64          # STATE only: thickness staked on truthfulness (≥ type-minimum)
  signature:     Ed25519Sig   # executor's signature over (id || claim || evidence || stake_amount)
}

EvidenceEdge {
  claim_id:      ClaimId      # which STATE claim is referenced (required)
  asserts:       str          # HOW this claim is relevant to the judgment (required, v3)
                              # e.g. "confirms the griefing-bound test passes,
                              #        which directly asserts the invariant"
}
```

The `asserts` field makes the relevance-claim **explicit and auditable.** The executor
must state the relationship between evidence and judgment in its own words. This
prevents the category-mismatch smuggle: "docs exist therefore architecture valid"
must be written, and writing it makes its weakness visible. The human audits the
stated relevance — a systematically bad `asserts` is a pattern the audit catches.

The evidence `claim_id` must reference a STATE claim within the same report that
*verified true*. Evidence edges pointing to false or nonexistent claims are flagged
as malformed. Evidence edges pointing to true-but-irrelevant claims are caught by
the human audit of the `asserts` field.

### 1.3 Claim — the typed claim payload

A Claim is a tagged union. The tag is the claim_type from taxonomy-v2.
The payload is the claim_type's typed fields.

```
Claim =
  | { claim_type: "test-result",      fields: { passed: uint, failed: uint, suite: str, ... } }
  | { claim_type: "build-result",     fields: { exit_code: u8, target: str } }
  | { claim_type: "file-exists",      fields: { path: str, line_count: uint } }
  | { claim_type: "file-content",     fields: { path: str, sha256: str, line_count: uint } }
  | { claim_type: "harness-result",   fields: { harness: str, measurements: map[str]f64 } }
  | { claim_type: "log-contains",     fields: { source: str, pattern: str, ... } }
  | { claim_type: "log-absent",       fields: { source: str, pattern: str, scope_start: datetime, ... } }
  | { claim_type: "git-status",       fields: { commit: str, dirty: bool, ... } }
  | { claim_type: "design-complete",  fields: { artifact: str } }           // JUDGMENT
  | { claim_type: "invariant-covered",fields: { invariant: str, test_suite: str } }  // JUDGMENT
  | { claim_type: "harness-adequate", fields: { harness: str, threshold: str } }     // JUDGMENT
  | { claim_type: "architecture-valid", fields: { component: str } }       // JUDGMENT
```

STATE claims carry typed fields only — no free-text prose. JUDGMENT claims carry
a scope description (which is prose, because JUDGMENTs ARE about meaning), plus
evidence edges pointing at verifiable STATE claims.

The executor provides *parameters* (claim type + fields). The verifier provides
the *check method* — a registered mapping from claim_type to verification action.
The executor never specifies how to verify its own claim.

---

## 2. Stake rules

### 2.1 STATE claims carry stake

Every STATE `SignedClaim` carries `stake_amount ≥ per-claim-type minimum`.
The executor may stake MORE to signal confidence, never LESS. Claims with
`stake_amount < type_minimum` are rejected by the verifier as malformed.

Per-claim-type stake minimums (v1):

| claim_type        | min_stake | Reasoning |
|-------------------|-----------|-----------|
| test-result       | 1.0       | Broad impact — tests gate most JUDGMENTs |
| build-result      | 1.0       | Broad impact — broken build blocks everything |
| file-content      | 0.5       | Narrower impact — single file integrity |
| file-exists       | 0.2       | Weakest STATE — only confirms existence and line count |
| harness-result    | 2.0       | High impact — harnesses measure defense adequacy |
| log-contains      | 0.5       | Moderate — presence is mechanically definitive |
| log-absent        | 1.0       | Higher — absence is only scoped, and scoped absence is weaker than presence |
| git-status        | 0.5       | Narrow — git state is rarely the load-bearing claim |

These minimums are calibrated for a mesh where 10 MiB of verified storage earns
~10.5 thickness units. A false test-result claim costs the executor roughly the
thickness from 1 MiB of verified storage. Tuning is expected after operational data.

### 2.2 JUDGMENT claims carry NO stake

JUDGMENT claims cannot be provably false — that's the definition of JUDGMENT.
Staking on an opinion pretends to mechanically penalize something mechanical
verification can't check. It would also create a perverse incentive: the executor
stakes zero on uncertain judgments (the ones that need it most) and full on
confident ones (the ones that need it least).

Instead, JUDGMENT claims carry *evidence edges* pointing to STATE claims that
*verified true*. A JUDGMENT backed by a false STATE claim is itself flagged —
not clawed back, but marked as *evidenced-by-false-claim* and routed to human
at elevated scrutiny.

### 2.3 Stake lifecycle

1. Executor emits SignedClaim with `stake_amount`. Thickness is encumbered from
   the executor's usable thickness (same mechanism as vouch-stake).
2. Verifier mechanically re-checks the STATE claim against bound_commit.
3. If CLAIM IS TRUE: stake is released back to executor's usable thickness.
4. If CLAIM IS FALSE: Layer 2 per-vouch clawback triggers — executor loses
   exactly `stake_amount`. The false claim is a provable protocol violation
   (the first concrete Layer 2 trigger: "executor asserted a false STATE claim").
5. If claim is MALFORMED (stake below minimum, missing required field, unknown
   claim_type): rejected by verifier, stake released, executor flagged for
   elevated audit.

---

## 3. Verification pass — verifier algorithm

```
for each SignedClaim in report:
    signer = recover_identity(signature)
    if claim is STATE:
        if not taxonomy.has_claim_type(claim.claim_type):
            reject("unknown claim type — closed-world default")
        if stake_amount < taxonomy.min_stake(claim.claim_type):
            reject("under-staked")
        check_fn = verifier.registered_check(claim.claim_type)
        actual = check_fn(report.bound_commit, claim.fields)
        expected = claim.fields
        if actual != expected:
            trigger_clawback(signer, stake_amount)
            mark_claim(false)
        else:
            release_stake(signer, stake_amount)
            mark_claim(verified_true)
    if claim is JUDGMENT:
        for each evidence_id in claim.evidence:
            evidence_claim = report.find_claim(evidence_id)
            if evidence_claim is None:
                flag("evidence edge points to nonexistent claim")
            elif evidence_claim is marked false:
                flag("JUDGMENT evidenced by FALSE claim — elevated scrutiny")
            elif evidence_claim is marked verified_true:
                attach_verified_evidence(evidence_claim)
        route_to_human(claim, attached_evidence, flags)

report state: all STATE claims verified or clawed-back;
              all JUDGMENT claims routed to human with verified evidence.
```

---

## 4. Example

```
ClaimReport {
  task_id: "session-20260714-layer2-wireup"
  bound_commit: "a1b2c3d4e5f6..."
  timestamp: 2026-07-14T18:30:00Z
  claims: [
    SignedClaim {
      id: "c1"
      claim: { claim_type: "test-result",
                fields: { passed: 47, failed: 0, suite: "lib" } }
      evidence: []
      stake_amount: 1.5
      signature: <Ed25519 sig over (c1 || test-result{...} || [] || 1.5)>
    }
    SignedClaim {
      id: "c2"
      claim: { claim_type: "build-result",
                fields: { exit_code: 0, target: "all" } }
      evidence: []
      stake_amount: 1.0
      signature: <Ed25519 sig>
    }
    SignedClaim {
      id: "c3"
      claim: { claim_type: "harness-result",
                fields: { harness: "swarm_distribution",
                          measurements: { "honest_vote_share_T100_N10000": 0.971 } }
              }
      evidence: []
      stake_amount: 2.0
      signature: <Ed25519 sig>
    }
    SignedClaim {
      id: "c4"
      claim: { claim_type: "design-complete",
                fields: { artifact: "Layer 2 bounded clawback" } }
      evidence: [
        { claim_id: "c1", asserts: "47 tests pass confirms the implementation compiles and runs" },
        { claim_id: "c3", asserts: "swarm harness measurements confirm the defense holds at measured parameters" }
      ]
      stake_amount: 0.0       // JUDGMENT — no stake
      signature: <Ed25519 sig>
    }
  ]
}
```

Verifier pass:
1. c1 (test-result): re-run `cargo test --lib` against bound_commit → 47/0 → verified_true, stake 1.5 released.
2. c2 (build-result): re-run `cargo build --all` against bound_commit → exit 0 → verified_true, stake 1.0 released.
3. c3 (harness-result): re-run swarm harness against bound_commit → measurements match → verified_true, stake 2.0 released.
4. c4 (design-complete): evidence c1 + c3 both verified_true, `asserts` fields make the
   relevance-claims explicit → human audits the stated relationships, not the raw evidence.

---

## 6. Probation Record — v1 forward-probation (2026-07-14)

The format was forward-probated against commit `8c1baeb` with a real 10-claim
report (7 STATE, 3 JUDGMENT). All 7 STATE claims were mechanically verified
against the checked-out commit — all verified true.

### Findings

**Finding 1 — Security model entirely unprobated.** Every claim carried
`<NOT YET SIGNED>` because no signing/identity infrastructure exists. The
stake/clawback model (the Phase-6-inward point) is structurally specified
but operationally inert. The mechanical-verifier build has a hard prerequisite:
signing/identity infra must exist before claims can be attributed, staked, or
clawed back.

**Finding 2 — Evidence-relevance unenforceable by format alone.** Two JUDGMENT
claims (c9: design-complete, c10: architecture-valid) attached true-but-category-mismatched
STATE evidence. c9: "tests pass + build works = design complete" — plausible-looking
but implementation-evidence ≠ design-completeness. c10: "docs exist = architecture
valid" — visibly mismatched. Both satisfied the ≥1-STATE-evidence rule while being
category-mismatched. The evidence was *true* but *irrelevant to the judgment*.

Amendment (v3): evidence edges now carry an explicit `asserts` field where the
executor states *how* the evidence is relevant. The human audits the stated relevance
rather than reconstructing it. This makes the category-mismatch visible — writing
"docs exist, therefore architecture valid" reveals its own weakness.

**Finding 3 — Verbosity.** Five `file-content` claims in one report is structurally
fine but operationally noisy. Not a format defect; an operational preference for
fewer, higher-value claims.

### What passed

All STATE claims verified true against bound_commit. JUDGMENT routing correct —
every claim landed in the right bucket. The format's structural requirements
(bound_commit, typed values, stake-on-STATE-only) held. No false claims filed.

### V3 amendments

- Evidence edges now carry `asserts: str` (required), making relevance-claims
  explicit and auditable. The human audits stated relevance, not raw evidence.
- Identity recovery from signature unchanged (security unprobated until signing
  infra exists — this is a build prerequisite, not a format change).

---

## 7. Self-assessment

This format is the interface between executor and verifier. It is a design artifact
ratified by reasoning, not proven by operation. Its security rests on three properties
that cannot be validated by the format itself:

1. The verifier's registered-check table is complete and correct for all claim types.
2. The per-claim-type stake minimums are calibrated high enough to deter false claims
   without being so high they prevent honest claims.
3. The executor's signature key is uniquely held and unforgeable.

Property 1 is a taxonomy-maintenance concern. Property 2 requires operational tuning
after real claim data. Property 3 is the same identity assumption the entire mesh
rests on.

The format will be tested by hand-application (executor emits structured claims
alongside colloquial summaries for a probation period) before any mechanical verifier
is built against it. Same discipline as the taxonomy itself: human-first, discover
the interface leaks, then mechanize.
