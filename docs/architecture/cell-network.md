# Cell Network Architecture

## Position in the stack

The Cell Network is a **domain layer** on top of the mesh's social layer
and evidence layer. It does not introduce new transport, new discovery,
or new persistence — it adds new message types to the existing gossipsub
topics and new claim types to the existing contribution primitive.

```text
    APPLICATION LAYER
    ┌──────────────────────────────┐
    │  Prototype Cells              │
    │  Experiments                   │
    │  Relationships                 │
    └───────────┬──────────────────┘
                │
    EVIDENCE LAYER
    ┌──────────────────────────────┐
    │  WitnessedClaims              │
    │  Receipts                     │
    │  Attestations                 │
    └───────────┬──────────────────┘
                │
    SOCIAL LAYER
    ┌──────────────────────────────┐
    │  Identity                     │
    │  Relationships                │
    │  Trust topology               │
    └───────────┬──────────────────┘
                │
    MESH LAYER
    ┌──────────────────────────────┐
    │  Discovery (mDNS/Kademlia)    │
    │  GossipSub communication      │
    │  Heartbeats / Presence        │
    │  Peer connections              │
    └──────────────────────────────┘
```

## Identity model

- A Cell **is** a `PeerId` with extended metadata (`cell_type`,
  `declared_purpose`). No secondary entity.
- Currently one-peer-one-cell by convention; multi-cell participation
  (one peer in several cells) is a known future requirement and will
  require `cell_participations: Vec<CellType>` replacing `cell_type:
  Option<CellType>`.
- A peer that does not declare a `cell_type` is a plain mesh node,
  not part of the Cell Network.

## Gossipsub topics

| Topic | Message type | Purpose |
|---|---|---|
| `lattice/cell/cell-rel/v1` | `CellRelationshipMsg` | Relationship lifecycle |
| `lattice/cell/experiment/v1` | `CellExperimentMsg` | Experiment announcements |
| `lattice/cell/reflection/v1` | `CellReflectionMsg` | Knowledge sharing |

Topics follow the existing `lattice/<domain>/<purpose>/v<version>`
naming convention. Version `v1` is the current iteration; breaking
changes increment the version number.

## Relationship model

- Cell relationships are **not** peer table entries. They are attested
  claims about a social topology on top of the liveness topology.
- A relationship must be proposed, accepted or rejected, and persisted
  through the snapshot mechanism.
- Relationships are stored in `cell_relationships: HashMap<String, String>`
  on `PersistentEconomicState`, keyed by base58 PeerId.

## Cell types

| Type | Role |
|---|---|
| `PrototypeCell` | Conducts experiments, forms relationships, submits claims |
| `ObserverCell` | Observes experiments, submits evidence, does not initiate |
| `WitnessCell` | Witnesses and attests claims, no experiment activity |

## Current constraints (documented, not baked)

1. `cell_participations` is a `Vec<CellType>` supporting multiple cells
   from day one. Empty = plain mesh node.
2. `is_infrastructure` distinguishes infrastructure operators from
   cell participants.
3. Cell identity is self-declared in heartbeat metadata, not verified
   by any authority. The system records the declaration; the network
   decides whether to recognize it.
