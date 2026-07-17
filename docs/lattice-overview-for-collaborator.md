# The Lattice — Overview for a Collaborator

## What It Is

The Lattice is a peer-governed network protocol. No central server, no company, no platform. Nodes coordinate directly — they validate each other's contributions, witness each other's proposals, and secure the network collectively.

It's designed to be the infrastructure layer for sovereign human-AI collaboration. The architecture uses what we call "thickness" — a stake-weighted governance model where influence is proportional to demonstrated contribution rather than token ownership or hashrate.

The smallest viable node runs on a Raspberry Pi 4 or 5. Right now the mesh is small: a handful of nodes operated by the founder and a few early participants. Every new node strengthens it.

## What's Built

**The economic layer** tracks contributions, vouches, and redistribution through a thickness graph. Nodes validate transactions locally and gossip them through the mesh. Key invariants:

- **Thickness is derived, not stored.** Each node computes influence at read time from stored inputs (basis points), not cached amounts. This makes the system resilient to state manipulation — a liquidated source can't leave ghost influence behind.
- **Nonces are strictly sequential.** No node applies a transaction out of order. Gaps are detected and filled via a fetch protocol we just completed — if a node misses a message, it asks its peers for the missing transaction and validates it before applying.
- **No floating point on the vouch path.** All stake amounts are integer basis points (0–10,000). Exact, deterministic, no rounding divergence between nodes.
- **A sender retry queue.** If a broadcast fails, the transaction stays in an outbound queue and is retried until a peer confirms it received it.

**The certificate layer** handles proposals, debates, and validation through a witness sortition mechanism. Nodes are selected as witnesses based on thickness-weighted randomness, and certificates are committed to an append-only hash chain when quorum is reached.

**The onboarding flow** is now a single page with three steps:

1. Open a terminal
2. Run two commands (curl + install script)
3. Watch it connect

No account, no sign-up, no technical jargon. That page is live at emergenceinstitute.live/join.html.

## What's Running

- A small test mesh (a few nodes, mostly founder-operated)
- The thickness graph, nonce enforcement, and transaction gossip
- The basic certificate chain at bootstrap
- A live dashboard at dashboard.emergenceinstitute.live

## What's Not Built Yet

The biggest open architectural question: **economic transactions are not on the chain.** They gossip as loose CBOR and are applied locally, but they never enter the append-only hash chain. This means:

- A node that restarts loses its nonce state and can't recover without asking peers
- A joining peer can't replay the chain to reconstruct the economic state
- There's no persistence substrate for the data structures we've built (fetch protocol, pending queue, outbound queue)

This is the fork decision that's been open since the beginning. The mechanism for committing transactions to the chain exists (it's used for genesis and bootstrap blocks in era one). It was never wired for era two's economic transactions.

## The Testing Question

We want advice on how to test the Lattice effectively at this stage. Specific questions:

1. **What's the right test strategy for a small mesh with incomplete persistence?** We have unit tests covering the core invariants (~116 passing), but we don't have integration tests that exercise multiple real nodes against each other.

2. **How do you test convergence properties when the system is eventually consistent by design?** Nodes that see different gossip subsets converge when the missing data arrives, but convergence isn't instantaneous. What's the right way to test that convergence actually happens within bounds?

3. **Is the economics-on-chain gap worth addressing before broader testing, or is testing the current architecture (loose CBOR gossip, no persistence) useful in its own right?** We know what's missing. The question is whether broader testing would reveal things we don't know yet.

4. **What test infrastructure would you recommend for a 3–5 node mesh?** We have the nodes running on physical hardware (Pi5s, cloud instances). Should we invest in simulation, containerized test networks, or stay with physical deployment?

Happy to share more detail on any of this. The code is at the Lattice node repo, the onboarding flow is live at the join page, and we're ready for the kind of feedback that only comes from someone who's actually poked at it.
