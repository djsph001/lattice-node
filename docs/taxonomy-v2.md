# Claim Taxonomy v2 — Lattice Node Development

**Status:** JUDGMENT artifact. v1 ratified 2026-07-14; v2 ratified 2026-07-14
after hand-application probation against ~20 real claims from four sessions.
Four amendments from probation: (1) two-channel rule with colloquial-as-never-evidence,
(2) divergence monitoring as health signal, (3) mandatory scope on absence claims with
lessons-learned note, (4) file-assurance audit rule. Core STATE/JUDGMENT routing
validated — no claim was misrouted. See §7 (Probation Record) for the retroactive
classification and findings.

**Ratified:** v1 2026-07-14; v2 2026-07-14.

---

## 0. The boundary — read this first

Every claim an executor submits must be tagged against this taxonomy. The routing is:

- **Tagged STATE, claim-type in this taxonomy:** verifier mechanically confirms;
  human audits by sampling. STATE values are typed — no free-text prose.
- **Tagged STATE, claim-type NOT in this taxonomy:** closed-world default —
  treated as JUDGMENT. Unknown claim-types route to human, never let through.
- **Tagged JUDGMENT:** routed to human with attached STATE evidence.
  Every JUDGMENT must carry ≥1 STATE claim as evidence, and the evidence
  STATE claims must be *independently relevant* to the judgment (not just
  decoration — a true-but-irrelevant STATE claim attached to a JUDGMENT is
  a taxonomy violation).

The verifier never identifies what a claim implies. The taxonomy does that, once,
at design time. The verifier enforces the taxonomy mechanically.

---

## 1. STATE claim types

STATE claims assert facts about the world that are mechanically confirmable.
Values are typed — never free-text prose. Each claim type carries a
`verify_method` telling the verifier HOW to confirm it.

### 1.1 test-result

Tests were executed and produced these counts.

```
claim_type: "test-result"
verify_method: rerunnable
fields:
  passed:    uint      # number of passing tests (required)
  failed:    uint      # number of failing tests (required)
  suite:     string    # which test suite, e.g. "lib", "bin", "all" (required)
  skipped:   uint      # number skipped (optional, default 0)
```

Verifier action: `cargo test --lib` (or equivalent), compare counts.

Integrity note: this claim confirms test *execution*, not test *coverage*.
Whether the tests cover the invariant is a separate JUDGMENT claim
(`invariant-covered`). The STATE claim is raw counts, never a verdict.

### 1.2 build-result

Source code compiled to binary.

```
claim_type: "build-result"
verify_method: rerunnable
fields:
  exit_code:  uint8     # 0 = success (required)
  target:     string    # "lib", "bin", "all" (required)
```

Verifier action: `cargo build`, check exit code.

### 1.3 file-exists

A file exists at the given path with the given line count.

```
claim_type: "file-exists"
verify_method: inspectable
fields:
  path:       string    # absolute path (required)
  line_count: uint      # number of lines (required)
```

Verifier action: stat the file, count lines.

Integrity note: `file-exists` does NOT assert content integrity.
Line count is a weak signal — two files with identical line counts can
differ entirely. For content integrity, use `file-content`.

### 1.4 file-content

A file exists with the exact content identified by hash.

```
claim_type: "file-content"
verify_method: inspectable
fields:
  path:       string    # absolute path (required)
  sha256:     string    # hex-encoded SHA-256 of file contents (required)
  line_count: uint      # number of lines (required)
```

Verifier action: stat, count lines, compute sha256, compare.

Integrity note: the sha256 field is REQUIRED, not optional. The claim
type itself declares the assurance level — `file-content` means
"exact content confirmed," and the hash is the proof.

### 1.5 harness-result

A measurement harness ran and produced these raw values.

```
claim_type: "harness-result"
verify_method: rerunnable
fields:
  harness:     string    # which harness, e.g. "swarm_distribution" (required)
  measurements: map[string]float64  # raw harness output (required)
```

Verifier action: re-run the harness, compare measurements.

CRITICAL: `harness-result` carries ONLY raw measurements, NEVER a verdict.
There is no `passed: bool` field. Whether the measurements indicate adequate
defense is a JUDGMENT claim (`harness-adequate`). The STATE claim is the
raw data; the meaning is separate.

Example:
```json
{
  "claim_type": "harness-result",
  "harness": "swarm_distribution",
  "measurements": {
    "honest_vote_share_N10_T100": 0.971,
    "honest_vote_share_N10000_T100": 0.375,
    "sybil_seat_share_N10_T100": 0.80
  }
}
```

### 1.6 log-contains

A log source contains a line matching the given pattern.

```
claim_type: "log-contains"
verify_method: inspectable
fields:
  source:    string    # "journald:unitname", or absolute file path (required)
  pattern:   string    # grep-compatible pattern (required)
  count:     uint      # expected match count (optional)
```

Verifier action: grep the source for the pattern.

Integrity note: presence is definitive — the verifier found the line.
This is a positive-evidence claim.

### 1.7 log-absent

A log source does NOT contain a line matching the given pattern,
within the declared scope.

```
claim_type: "log-absent"
verify_method: inspectable
fields:
  source:      string    # "journald:unitname", or absolute file path (required)
  pattern:     string    # grep-compatible pattern (required)
  scope_start: datetime  # beginning of search window (required)
  scope_end:   datetime  # end of search window (required)
  lines_searched: uint   # total lines examined (required)
```

Verifier action: grep the source within the scope window, confirm zero matches.

CRITICAL: absence claims are ONLY meaningful with a declared scope. An
unscoped "log line is absent" claim is nearly meaningless — absence of
evidence is bounded by where you looked. The verifier confirms absence
WITHIN the declared scope. It does NOT confirm absence beyond it.

### 1.8 git-status

The working tree is at a specific commit and clean/dirty state.

```
claim_type: "git-status"
verify_method: inspectable
fields:
  commit:    string    # full SHA of HEAD (required)
  dirty:     bool      # true if uncommitted changes exist (required)
  branch:    string    # branch name (optional)
```

Verifier action: `git rev-parse HEAD`, `git status --porcelain`.

Integrity note: `dirty: bool` is the load-bearing field — it catches
"claimed committed but has uncommitted changes." `branch` is informational
only (branches are mutable labels).

---

## 2. JUDGMENT claim types

JUDGMENT claims assert that decidable state *satisfies intent* — claims about
meaning, adequacy, correctness, or coverage. They are mechanically unroutable
to a verifier and must be decided by a human.

Every JUDGMENT claim carries ≥1 STATE claim as evidence. The evidence
STATE claims must be independently relevant — not just present for decoration.

JUDGMENT claims use free-text for their content (they ARE prose), but their
*structure* is typed (which STATE claims are attached) and the *routing* is
mechanically enforced.

### 2.1 design-complete

A design artifact is finished and correct.

```
claim_type: "design-complete"
fields:
  artifact:    string    # what artifact, e.g. "spec §4", "agent-verification arch" (required)
  evidence:    [claim]   # STATE claims supporting completeness (required, ≥1)
```

Typical evidence: `test-result` for tests covering the design,
`file-exists` for the artifact itself.

### 2.2 invariant-covered

A test suite covers the named invariant.

```
claim_type: "invariant-covered"
fields:
  invariant:    string    # which invariant, e.g. "griefing bound" (required)
  test_suite:   string    # which test suite is claimed to cover it (required)
  evidence:     [claim]   # STATE claims, typically test-result (required, ≥1)
```

The STATE evidence confirms tests passed. Whether those tests *actually
cover the invariant* is the JUDGMENT — the human rules on that, using
the test-result STATE as evidence.

### 2.3 harness-adequate

A harness's measurements indicate acceptable performance against a threshold.

```
claim_type: "harness-adequate"
fields:
  harness:     string    # which harness (required)
  threshold:   string    # what threshold, e.g. "honest_vote_share ≥ 0.95" (required)
  evidence:    [claim]   # harness-result STATE claim (required, ≥1)
```

The STATE confirms the harness ran and produced numbers.
Whether those numbers exceed the threshold is decidable (it's arithmetic).
Whether the threshold *itself* constitutes adequate defense is the JUDGMENT.

### 2.4 architecture-valid

A named architectural component is correctly designed.

```
claim_type: "architecture-valid"
fields:
  component:    string    # which component (required)
  evidence:     [claim]   # STATE claims (required, ≥1)
```

This is the broadest JUDGMENT claim. It is the claim about the system's
own correctness — and by this taxonomy, it is irreducibly human. No
mechanical verifier can confirm "the architecture is correct."

---

## 3. Edge-hardenings

These are the rules that prevent the taxonomy from being a smuggle vector.
They are baked into the taxonomy, not bolted on.

### 3.1 Closed-world default

Any claim whose `claim_type` is not explicitly listed in §1 (STATE) is
routed as JUDGMENT. Unknown, novel, or untagged claim types → human.
STATE is an allowlist, never a denylist. The system fails safe: a
growing or incomplete taxonomy routes *more* to the human, never less.

### 3.2 STATE values are typed, never prose

Every STATE claim carries typed fields (uint, string, bool, map).
No free-text prose fields in STATE claims. Prose is where implicature
hides — the executor cannot smuggle "these are the *right* tests" into
a `test-result` claim because `test-result` has no prose field to hide it in.

### 3.3 No verdicts on measurements

Measurement claims (`harness-result`, `test-result`) carry raw values
and counts, never a `passed: bool` or "verdict" field. Whether the
measurements constitute success is a separate claim — either a STATE
claim against an explicit numeric threshold, or a JUDGMENT claim.
The STATE claim is the raw data; the meaning is separate.

### 3.4 Assurance level carried by claim type, not optional fields

`file-exists` (weak, line-count only) and `file-content` (strong, hash)
are separate claim types. The executor picks the claim type that matches
what they're actually asserting. Optional integrity fields are forbidden
— there is no `sha256: optional` on `file-exists`. Pick the type that
carries the assurance you're claiming.

### 3.5 Absence claims are scoped

`log-absent` requires `scope_start`, `scope_end`, and `lines_searched`.
A claim that something is absent without declared search bounds is
nearly meaningless and is rejected. Absence is scoped STATE — decidable
only within declared bounds.

### 3.7 Two-channel rule: colloquial language is never evidence

The executor produces two parallel outputs: structured claims (the verification
channel) and colloquial summary (the human chat channel). Only the former carries
verification weight. The verifier structurally ignores the latter.

The colloquial channel can be warm, discursive, loose, metaphorical — it's the
human-facing interface. The structured channel is cold, typed, mechanically
verifiable. Neither constrains the other.

Critically: the executor does NOT "translate" colloquial into structured. The
structured claim is produced *directly from tool output* — `cargo build` returns
exit_code 0, the executor records `build-result{exit_code:0, target:all}`. The
colloquial summary ("builds clean!") is a separate artifact. There is no
translation step, so there is no unverified conversion for the executor to smuggle
through.

A colloquial summary that contradicts the attached structured claim is not a
verification failure — the structured claim is authoritative. But see §3.8.

### 3.8 Divergence monitoring as health signal

When the colloquial channel and structured channel disagree — the chat says
"builds clean!" but the claim shows `exit_code:1` — this is not a verification
problem (the structured claim is authoritative). But it IS a health signal.

A systematic pattern of chat-says-X-but-claim-says-not-X indicates an executor
whose narration has drifted from its evidence. This is monitored by the human
audit as an early-warning flag, not as a verification failure. Divergence
doesn't trigger clawback. It triggers scrutiny.

### 3.9 File-assurance downgrade audit

The executor will tend to pick the weaker claim type (`file-exists`, line-count
only) over the stronger one (`file-content`, hash-required) because it's easier
to verify. This is a known assurance-downgrade tendency.

The human audit samples `file-exists` claims for cases where `file-content`
was warranted — where the file's *content* mattered to the larger task, not just
its existence. This is an audit rule, not a design change: the claim types remain
as-is, but the auditor watches for downgrade.

---

## 4. Amendment procedure

This taxonomy is the trust root of the agent-verification architecture.
Amending it is the single highest-privilege operation in the system.

### 4.1 Proposal

An amendment proposes to add, remove, or reclassify a claim type.
It must include:
- The change (which claim type, new or moved)
- Justification (why this claim-type is mechanically decidable now)
- Example claims showing the type in use

### 4.2 Ratification

The human ratifies explicitly — no sampling, no batching. A taxonomy
amendment is a JUDGMENT claim about what counts as decidable, and
it receives full human attention, every time.

### 4.3 Post-ratification audit

After ratification, the amended boundary is treated as *unproven* for
N epochs (default N=10). Claims using the new or reclassified claim
type are sampled at a rate of 100% (every claim audited) for the
probation period. After N clean epochs with no taxonomy violations,
the audit rate drops to the standard sampling rate.

The boundary earns its place — like a node earning thickness — by
clean behavior over time, not by assertion.

---

## 5. Examples

### Correct decomposition

```
Task: Build Layer 2 bounded clawback

STATE claims:
  1. [test-result] passed=40, failed=0, suite=lib
  2. [file-content] path=src/ledger/thickness.rs, line_count=710, sha256=abc123...
  3. [harness-result] harness=swarm_distribution, measurements={...}

JUDGMENT claims:
  4. [design-complete] artifact="Layer 2 bounded clawback",
     evidence=[claim 1, claim 3]
  5. [invariant-covered] invariant="griefing bound",
     test_suite="chain_clawback_does_not_propagate_past_direct_voucher",
     evidence=[claim 1]
```

### Smuggle — caught by this taxonomy

```
STATE claims:
  1. [test-result] passed=47, failed=0, suite=lib

# WRONG — test-result cannot carry "these are the RIGHT tests"
# The taxonomy enforces typed fields; there's no prose field for this.
```

### Smuggle — caught by edge-hardening 3.3

```
STATE claims:
  1. [harness-result] harness=swarm_distribution, passed=true  ← REJECTED
  # harness-result has no 'passed' field. The raw measurements
  # belong in 'measurements'; the verdict is a separate JUDGMENT claim.
```

---

## 6. Self-assessment

This taxonomy is itself a JUDGMENT artifact. Its claim is:
"The boundary between STATE and JUDGMENT is correctly drawn for
lattice-node development work."

By its own rules, this claim is `architecture-valid`, routed to human,
and cannot be mechanically confirmed. v1 was ratified by reasoning
(2026-07-14) and tested via retroactive hand-application against
~20 real claims from four sessions. v2 incorporates four amendments
from that probation. The core STATE/JUDGMENT routing was validated:
no claim was misrouted.

The human-audited probation period produced the findings recorded in §7.

---

## 7. Probation Record — v1 hand-application (2026-07-14)

~20 real claims from four lattice-node development sessions were
retroactively classified against taxonomy-v1. The full classification,
with routing verdicts and gap notes, is recorded here as the evidence
base for v2 amendments.

### Claims that routed correctly

```
CLAIM                                    TAG              VERDICT
──────────────────────────────────────────────────────────────────────────
"33 tests pass, 0 fail"                  test-result      ✓ STATE (decidable, rerunnable)
"47 tests pass, 0 fail"                  test-result      ✓ STATE
"cargo build exits 0"                    build-result     ✓ STATE
"The griefing-bound test passes"         test-result      ✓ STATE
"Alice untouched"                        invariant-covered JUDGMENT w/test-result evidence ✓
"Layer 2 is done"                        design-complete  JUDGMENT ✓
"Swarm: 97.1% at T=100"                 harness-result   STATE (raw measurements only) ✓
"97.1% is adequate defense"             harness-adequate  JUDGMENT w/harness-result ✓
"Weighted voting dissolves seat-ceiling" architecture-valid JUDGMENT ✓
"Panel guard blocks sparse meshes"       test-result + invariant-covered ✓
"Re-division bug is fixed"               design-complete  JUDGMENT w/test-result ✓
"Epoch expiration unwinds vouches"       test-result + invariant-covered ✓
"N_all × 0.02 threshold"                harness-result   STATE ✓
"taxonomy-v1 doc exists"                 file-exists      STATE ✓
```

### Gaps found — v2 amendments motivated

**Gap 1 — Unscoped absence claims (→ §3.5 already required this; probation
confirmed it wasn't being honored in practice).** The Hetzner economic-silence
check and the PicklingError absence check were some of the most important
read-real-state verifications performed — and both were made informally,
without declared scope bounds. "No PicklingError" and "no economic epoch line"
were conclusions drawn from unstated search windows. Under v1, these claims
would be REJECTED as incomplete — mandatory scope fields were not filled.

Lesson: absence claims that felt rigorous were actually drawing conclusions
from implicit bounds. The taxonomy caught that. Scoped absence is not a
formality — it's a substantive requirement. Amendment to v2: no relaxation
of scope requirements; the lesson is that past absence checks were less
verifiable than they felt.

**Gap 2 — Colloquial-to-structured translation trust (→ §3.7, §3.8 resolved).**
Real claims arrived as natural language ("builds clean," "the guard blocks")
and were classified against the taxonomy during hand-application. In a live
system, the executor would produce the structured claim directly from tool
output — no translation step. The colloquial channel would exist in parallel
but carry zero verification weight. The danger identified: if the executor
*were* allowed to translate colloquial → structured, the translation would
be an unverified judgment by the incentivized party.

Amendment to v2: two-channel rule (§3.7) forbids colloquial-as-evidence;
structured claims are produced directly from tool output, never via translation.
Divergence monitoring (§3.8) as a health signal.

**Gap 3 — Assurance downgrade (→ §3.9 audit rule).**
`file-exists` (line-count, weak) was used where `file-content` (hash, strong)
was warranted. The executor will tend to pick the weaker claim type because
it's easier. Amendment to v2: audit rule samples `file-exists` for cases
warranting `file-content`. No claim-type change.

### What was NOT found

No claim was misrouted between STATE and JUDGMENT. The taxonomy's central
boundary — the line between mechanically decidable and human-judgment claims —
held against real, known-answer claims from actual development work. The
verdict-smuggle defense (harness-result carries raw measurements, never a
passed/failed verdict) correctly separated "97.1%" (STATE) from "97.1% is
adequate defense" (JUDGMENT).
