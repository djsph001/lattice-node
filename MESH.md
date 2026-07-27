# Lattice Mesh — Topology Registry

**Updated:** 2026-07-27
**Authority:** Every entry describes the intended configuration — node names,
ports, storage dirs, identity keys, and launch flags. Live state lives in
`ps aux`, the dashboard, and logs.
**Discipline:** Update this file when the configuration changes. A topology
change not recorded here is a phantom-report-class error.

---

## Current State

**No production nodes running.** Both nodes were stopped and storage wiped during
the Jul 27 build-check cleanup (see `docs/ARCHIVE.md` for the old topology).

### To restart

```bash
cd ~/Projects/lattice-node
cargo build --release --bin lattice-node

# Node 1 — genesis root
./target/release/lattice-node --name morning-api --port 4005 \
  --identity-dir /tmp/m-ap-id --storage-dir /tmp/m-ap \
  --auto-genesis --no-mdns --persistence --mint 5000

# Node 2 — witness peer
./target/release/lattice-node --name local-witness --port 4010 \
  --identity-dir /tmp/lw-id --storage-dir /tmp/local-witness \
  --genesis-root <PEER_ID> \
  --bootstrap-peer /ip4/127.0.0.1/tcp/4005/p2p/<PEER_ID> \
  --no-mdns --persistence --mint 0
```

**Identity:** Both nodes use stable keys in `/tmp/m-ap-id/` and `/tmp/lw-id/`.
Peer IDs persist across restarts *but not across reboots* — `/tmp/` is tmpfs.
If the identity dirs are lost, both nodes regenerate keys and the witness's
`--genesis-root` must be updated to the root's new PeerId.

For a longer-running mesh, move identity dirs to `~/.lattice/m-ap-id/` and
`~/.lattice/lw-id/`.

| Node | PeerId |
|---|---|
| morning-api | `12D3KooWPfrZgiinxkPdE61kKE5YwPRjqTaHGxEDMmUBvq6zLVxJ` |
| local-witness | `12D3KooWDNNZmWvTzeQeLgHD6sTmEPH1vrwLEBuVEaEFpfct9sch` |

**Build:** Build from HEAD on `main`. Verify with `build_commit` on the
dashboard or `GetNodeInfo` via UDS — should match `git log --oneline -1`.

---

## Machine: z4-workstation

Physical workstation in Boynton Beach, Florida. IPs: `192.168.10.200`,
`100.93.232.107` (Tailscale). Ubuntu 24.04. All node processes run here.

---

## Historical Topology

Prior sessions, not currently active:

- **mac-node (10.0.0.16):** Mac Mini dev client. Jul 22.
- **Hetzner relay (167.233.223.174):** Production bootstrap, Ashburn VA. Jul 22.
- **Anchor root (12D3KooWBPyh…):** Genesis root on z4, killed by broad pkill Jul 22.

### Propagation test harness

`~/genesis-propagation-test.sh` — isolated two-node mesh on ports 4105/4110,
separate from production. Run: `bash ~/genesis-propagation-test.sh` (~65s).

---

## Operational Rules

These are in force. Violating them has caused real incidents.

- **pkill scoping:** Always scope by `--name` flag. `pkill -9 -f lattice-node`
  kills everything on the host. Cost: collapsed a 3-node mesh to 1 node.
- **mDNS policy:** Dev nodes run with `--no-mdns` and explicit bootstrap lists.
- **Build verification:** Before trusting a running node, check `build_commit`
  via `GetNodeInfo` or the dashboard. Matches git HEAD. Reports `-dirty` if
  the working tree had uncommitted changes at build time.
- **Script-based launch:** Never paste multi-line terminal commands. Write a
  script file, push to git, run with `bash script.sh`. Cost: fragmented
  command lines from paste-wrapping created rogue nodes.
