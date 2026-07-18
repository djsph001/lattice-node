# C: Application-Level Liveness & Zombie Eviction

> Design from soak analysis, 2026-07-18.
> Converged with collaborator.

## Problem

When a node's network drops (sleep, WiFi hiccup) without the process crashing, libp2p holds a half-open TCP connection. The node's metrics look healthy (`outstanding_fetches=0 aged=0 outbound_queues=[]`) but no traffic flows from peers. Recovery requires manual restart.

## Design

### Metric & Warn
- Track `last_seen` on `PeerInfo` (already exists in `state/peers.rs`, updated on heartbeat)
- On every metrics tick, check elapsed since `last_seen` per peer
- Warn at **30s** (3× heartbeat interval)
- Include in metrics line: `max_peer_silence_secs=N`

### Evict
- Force `swarm.disconnect_peer_id()` at **90s** (9× heartbeat interval)
- After disconnect, set `pending_reconnect: Option<Instant>` for that peer

### Circuit Breaker
- Track evictions in `evictions_last_minute: VecDeque<Instant>`
- If len ≥ 3 in last 60s, skip disconnect action, log ERROR with actionable message
- Reset to warning-only until evictions drain from the window

### Reconnect Verification
- On `ConnectionEstablished` event for a peer with `pending_reconnect`, clear the flag
- If pending flag exists for >30s, log ERROR — self-healing failed, human intervention needed

### Integration Point
- Piggyback on the existing metrics tick (10s cadence) — quantization doesn't matter for 30s/90s thresholds
- Same pattern as `sweep_stale_fetches()`

### Implementation Sketch

```rust
// In node.rs
fn check_peer_liveness(&mut self) {
    let now = Instant::now();
    let mut longest_silence = 0u64;
    let peer_ids: Vec<PeerId> = self.peer_table.peer_ids().collect();

    for peer_id in peer_ids {
        let elapsed = self.peer_table
            .seconds_since_last_seen(&peer_id)
            .unwrap_or(0);

        if elapsed > longest_silence {
            longest_silence = elapsed;
        }

        if elapsed > ZOMBIE_EVICT_THRESHOLD_SECS && !self.is_local_peer(&peer_id) {
            // Circuit breaker check
            let recent_evictions = self.evictions_last_minute
                .iter()
                .filter(|t| now.duration_since(**t).as_secs() < 60)
                .count();

            if recent_evictions >= 3 {
                error!("Circuit breaker active — {} evictions in last 60s. Human intervention needed.", recent_evictions);
                continue;
            }

            warn!("Evicting zombie peer {} — silent for {}s", peer_id, elapsed);
            let _ = self.swarm.disconnect_peer_id(peer_id);
            self.evictions_last_minute.push_back(now);
            self.pending_reconnect.insert(peer_id, now);
        } else if elapsed > ZOMBIE_WARN_THRESHOLD_SECS {
            warn!("Peer {} silent for {}s (evict threshold: {}s)",
                  peer_id, elapsed, ZOMBIE_EVICT_THRESHOLD_SECS);
        }
    }

    // Check reconnect success
    self.pending_reconnect.retain(|peer_id, since| {
        if now.duration_since(*since).as_secs() > RECONNECT_TIMEOUT_SECS {
            error!("Failed to reconnect to evicted peer {}. Manual restart may be needed.", peer_id);
            false // remove from pending after logging
        } else {
            true // still waiting
        }
    });
}

// In heartbeat handler — update last_seen
fn on_heartbeat_received(&mut self, peer_id: &PeerId) {
    self.peer_table.record_heartbeat(peer_id);
    // If this peer was pending reconnect, clear the flag
    self.pending_reconnect.remove(peer_id);
}
```

### Wire Sequence (in heartbeat tick, after sweep, before metrics)

```
heartbeat_timer.tick()
  → broadcast_heartbeat().await
  → sweep_stale_fetches()           // existing
  → check_peer_liveness()           // new  (warn, evict, reconnect)
  → metrics line (include max_peer_silence_secs)
```

## Tests

- **Positive**: peer with silent 100s → disconnect called
- **Negative**: peer with silent 60s (between warn and evict) → warn only, no disconnect
- **Circuit breaker**: 3 evictions in 1 min → 4th silent peer skipped, ERROR logged
- **Reconnect**: evict then fake ConnectionEstablished → pending flag cleared
