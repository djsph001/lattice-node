# Experiment CAP-001 — Objection Cap Enforcement

**Experiment ID:** EXP-CAP-001  
**Status:** BLOCKED — transport-layer failure  
**Date:** 2026-07-28

## Setup
- Ephemeral two-node mesh: exp-claimer :4200, exp-witness :4210
- Dirs: /tmp/exp-cap-001/ (cleaned up)
- Genesis established, claim accepted via WitnessClaimService
- claim_id: e2cd7c79f58b19bccb4af0ca280f2d2b94cbf40178a05566c1a0afbd16f27dce (computed, verified via UDS SubmitObjection)

## Blocked At
Transport layer. The objection-injector binary publishes to gossipsub, gets a `message_id` back (local queue insertion), and exits before gossipsub processes its outbound queue. The claimer node never receives the message — zero log entries at any dispatch path.

## Root Cause
`publish()` returns `Result<MessageId, PublishError>` — this is local queue insertion, not network delivery. The injector prints "OK" on `message_id` success and exits immediately. The process terminates before gossipsub forwards the message. The `message_id` is a false positive — same class as sender-debit-before-confirmation: local success mistaken for distributed delivery.

## Finding
The objection receive path (`handle_objection_message`, `/lattice/objection/v1`) is wired, compiles, and has tests. It has never processed a real message. The first attempt to exercise it failed at transport.

## Required Fix
The injector must drive the swarm event loop for several seconds after publishing to allow gossipsub fanout delivery. Alternatively, wait for a delivery acknowledgment or confirm the message was received by querying the node.

## Next Steps
- Builder fix injector to wait for delivery
- Re-run experiment on fixed injector
- Verify cap boundary, duplicate-after-cap behavior
