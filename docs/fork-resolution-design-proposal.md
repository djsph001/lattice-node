# Certificate Chain Fork Resolution — Design Proposal

> From collaborator response, 2026-07-17.
> Saved for reference.

## Design Principle: Forks Should Be Resolvable, Not Preventable

In a partition-tolerant system, forks will happen. The question isn't "how do we prevent them?" — it's "how do we resolve them deterministically when they occur?"

The economic layer already answers this: per-entity sequential nonces. Transactions queue and apply in order upon healing. The certificate chain needs an equivalent rule.

## The Resolution Rule: Thickness-Weighted Finality

Governance authority is thickness, not node count. A fork supported by nodes with 10,000 total thickness should beat a fork supported by nodes with 6,000 thickness — even if the latter has more nodes.

### The Rule

1. Identify the divergence point (last common `lat_commitment` hash).
2. Calculate the total thickness of all witnesses who signed certificates on each fork since the divergence point.
3. The fork with higher total thickness wins. All nodes adopt it.
4. Tiebreaker: longer chain, then deterministic hash comparison.

### Why This Works

- Aligns with governance model: authority proportional to contribution.
- Deterministic: all nodes calculate the same winner given the same data.
- Sybil-resistant: many low-thickness nodes can't outvote few high-thickness contributors.
- Minimal complexity: reuses existing thickness graph.

## Implementation Specification

### Fork Detection
- During gossip sync after partition heals, nodes detect divergent `lat_commitment` hashes sharing a common ancestor.
- Both nodes exchange full certificate chains from the common ancestor forward.

### Fork Isolation
- `fork_point` = last common certificate sequence number.
- `fork_A` = certificates from `fork_point + 1` to tip.
- `fork_B` = same from the other side.

### Thickness Score Calculation
```
score(fork) = Σ (witness_thickness(w) for each cert c in fork for each witness w in c.witnesses)
```

Where `witness_thickness(w)` is the derived thickness of witness `w` at the moment of resolution.

### Decision Logic
```
if score(A) > score(B): winner = A
elif score(B) > score(A): winner = B
else:
    if len(A) > len(B): winner = A
    elif len(B) > len(A): winner = B
    else: winner = A if hash(A) > hash(B) else B
```

### Adoption
- Losing node discards its fork.
- Proposals from losing fork can be re-proposed to canonical chain.
- All nodes update `lat_commitment` to winner's tip hash.

### Edge Cases
- **Missing witness data**: Treat unknown witness thickness as 0.
- **Multiple forks**: Run pairwise or score all, pick max.
- **Empty fork**: One side has no new certs → other side wins.

## Test Scenarios

| Partition | Fork A | Fork B | Expected Winner |
|:---|:---|:---|:---|
| A-B vs C | 2 nodes, thickness 5000 | 1 node, thickness 8000 | Fork B (higher thickness) |
| A-B vs C-D | 2 nodes, 6000 each | 2 nodes, 6000 each | Tiebreaker → longer/hash |
| A-B-C vs D | 3 active nodes | 1 idle (no certs) | Fork A (B is empty) |
| A vs B-C | 1 node, thickness 3000 | 2 nodes, 4000+2000 | Fork B (6000 > 3000) |
| Known vs known+unknown | Thickness 7000 | Known 5000 + unknown | Fork A (7000 > 5000) |

## What This Unlocks

- Persistence: knows which chain state is canonical.
- Documentation: can describe partition behavior to operators.
- Alpha expansion: new operators join with confidence the network self-heals.

> **Note (Lumen):** The design uses thickness-at-resolution-time for scoring. This means the score depends on when the partition heals, not on the content of each fork. A node that just received a large vouch has disproportionate weight at the moment of healing. This may be intentional (fresher thickness = more relevant) but is worth naming explicitly.
>
> The tiebreaker ordering (longest chain first, then hash) is standard and leaves no ambiguity.
