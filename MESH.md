# Lattice Mesh — Topology Registry

**Updated:** 2026-07-27
**Authority:** Every entry sourced to a log line, socket query, config file, or
ongoing process. Unknown fields are explicit.
**Discipline:** Update this file every time a process launches, dies, moves, or
changes identity. A topology change not recorded here is a
phantom-report-class error.

---

## Current Running Mesh

Two nodes on z4-workstation. No remote nodes. No mDNS.

| # | Name | Port | Storage Dir | Role | Status |
|---|---|---|---|---|---|
| 1 | **morning-api** | 4005 | `/tmp/m-ap` | Primary node with persistence + API socket | ✅ Running |
| 2 | **local-witness** | 4010 | `/tmp/local-witness` | Witness peer for claim orchestration | ✅ Running |

**Build:** Both on `f4eb933` (unified WAL migration complete).
**Persistence:** Unified `wal.log` format on both nodes. Legacy
`transactions.wal` / `claims.wal` files retired via two-cycle rotation.

**Bootstrap relationship:** `morning-api` dials `local-witness` at
`/ip4/127.0.0.1/tcp/4010/p2p/12D3KooWSRczH2KScKRwk7CiUqpW6vEeFYEwF9Jevctam7QYsAEM`.

**Peer IDs:** Not re-verified this session. Both nodes auto-generate new keys on
each fresh start (`--auto-genesis` + clean identity dirs). Previous sessions
recorded `morning-api` as `12D3KooWN9kqgn1oJG4vujyYP2RE9g4GeUcrZqc8eRKNbanRUDbB`
and `local-witness` as `12D3KooWSRczH2KScKRwk7CiUqpW6vEeFYEwF9Jevctam7QYsAEM`.
Current values may differ after the Jul 26 clean restart.

---

## Machine: z4-workstation

Physical workstation in Boynton Beach, Florida. All node processes run here.
Ubuntu 24.04. IPs: `10.0.0.133`, `192.168.10.200`.

### Running processes

```
./target/release/lattice-node --name morning-api --port 4005 \
  --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap \
  --auto-genesis --no-mdns --persistence --mint 5000 \
  --bootstrap-peer "/ip4/127.0.0.1/tcp/4010/p2p/<local-witness-id>"

./target/release/lattice-node --name local-witness --port 4010 \
  --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness \
  --auto-genesis --no-mdns --persistence --mint 0
```

**Source:** `ps aux | grep lattice-node`, Jul 26.

### lattice-bridge

- **Status:** Not currently running. (Previously served on 8081, bound to UDS
  socket. The bridge is a separate binary, not part of the node.)

---

## Historical Topology (Pre-Jul 26)

Prior sessions referenced additional nodes that are not part of the current
running system:

- **mac-node (10.0.0.16):** Dev client on a separate Mac Mini. Referenced in
  Jul 22 MESH.md. Not currently active.
- **Hetzner relay (167.233.223.174):** Production bootstrap node on Hetzner
  US-East (Ashburn, VA). Referenced in Jul 22 MESH.md. Not currently active.
- **Anchor root (12D3KooWBPyh…):** Genesis root key on z4, killed by broad
  pkill on Jul 22, not revived in current mesh.
- **relay-hub, lumen:** Processes previously running on z4, killed Jul 22.

**Expansion note:** Any future deployment of additional nodes (mac-node,
Hetzner relay, or new machines) must update this document before deployment
work references those nodes in design decisions, rollout sequences, or
collaborator summaries. The topology drift that caused Jul 25-26 confusion
(Lumen summarizing phantom Mac/Hetzner deployments) is the exact failure mode
this discipline prevents.

---

## Design Decisions Recorded

- **2026-07-25: Unified WAL migration.** Two legacy persistence authorities
  (`transactions.wal`, `claims.wal`) collapsed into one unified `wal.log` with
  `WalRecord` enum (Transaction=1, Claim=2). Migration completed via two-cycle
  rotation across both nodes. See `docs/architecture/persistence-design.md`.
- **2026-07-27: Genesis lifecycle.** `WalRecord::Genesis` (tag 0x03) added as
  a distinct variant with its own validation rules (root signer, exactly-once,
  network_name). Five commits: type definitions → validation logic → CLI/config
  plumbing → recovery integration → gossip handler. Propagation failed initially
  because `re_gossip_genesis` fired on `ConnectionEstablished` before gossipsub
  mesh had peers on `/lattice/genesis/v1`. Fixed by retrying on
  `InsufficientPeers` via a `pending_genesis_gossip` set drained on each
  metrics tick (~10s). Full lifecycle confirmed: author → persist → gossip →
  receive → validate → persist on witness → recovery on restart.
  See `docs/architecture/genesis-gossip-design.md`.
  **Propagation test harness:** `~/genesis-propagation-test.sh` — isolated
  two-node mesh on ports 4105/4110, separate from production, reproducible.
  Run: `bash ~/genesis-propagation-test.sh` (~65s).
- **2026-07-22: mDNS policy.** Dev nodes run with mDNS disabled and explicit
  bootstrap lists. A node that appears without configuration should be shown as
  intruder, not neighbor.
- **2026-07-22: pkill scoping.** Always scope `pkill` by `--name` flag to
  avoid killing all lattice processes on a multi-tenant host.
