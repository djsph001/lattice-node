# Startup NTP Clock Drift Check

> Design from soak analysis, 2026-07-18.
> Converged with collaborator.

## Why

The 17-hour soak revealed that Mac sleep/wake cycles cause clock drift (22 min → 15 hours) that silently poisons the mesh. The node's timestamp validation (±300s) correctly rejects bad transactions, but the fault is invisible to the operator whose clock is wrong.

**The fix:** check clock against an NTP server at startup. Refuse to start if drift > 300s.

## Design Decisions

| Decision | Value | Rationale |
|:---|:---|:---|
| **Threshold** | 300s (5 min) | Matches existing transaction timestamp tolerance. Protocol already considers this "acceptable drift." |
| **Failure mode** | Hard-fail | Warn-and-continue still poisons the mesh. Operator explicitly acknowledges risk via `--skip-ntp-check` |
| **Time source** | Configurable, default list | `time.apple.com, time.google.com, pool.ntp.org`. Tried in order, first responder wins. |
| **Drift direction** | Report ahead/behind | Operator needs to know if it's timezone, stopped clock, or NTP failure. |
| **All servers unreachable** | Hard-fail (with escape hatch) | `--skip-ntp-check` flag for air-gapped/hostile-network cases. |

## Error Message

```
Clock drift too large: local clock is 900s behind of time.apple.com.
Threshold: ±300s.
Sync your clock and restart.
  On macOS: sudo sntp -sS <server>
  On Linux: sudo ntpdate <server>
```

## Implementation Sketch

```rust
const CLOCK_DRIFT_THRESHOLD_SECS: i64 = 300;
const DEFAULT_NTP_SERVERS: &[&str] = &["time.apple.com", "time.google.com", "pool.ntp.org"];

fn verify_clock_sync(ntp_servers: &[&str], skip: bool) -> Result<()> {
    if skip {
        warn!("Clock check skipped via --skip-ntp-check");
        return Ok(());
    }
    for server in ntp_servers {
        match query_ntp(server) {
            Ok(ntp_time) => {
                let local = SystemTime::now();
                let drift = ntp_time.signed_duration_since(local).num_seconds();
                if drift.abs() > CLOCK_DRIFT_THRESHOLD_SECS {
                    return Err(anyhow!(
                        "Clock drift: local clock is {}s {} of {} (threshold ±{}s).\n\
                         Sync your clock and restart.\n  macOS: sudo sntp -sS {}\n  Linux: sudo ntpdate {}",
                        drift.abs(), if drift < 0 { "ahead" } else { "behind" },
                        server, CLOCK_DRIFT_THRESHOLD_SECS, server, server,
                    ));
                }
                info!("Clock verified: drift {}s", drift);
                return Ok(());
            }
            Err(e) => warn!("NTP query to {} failed: {} (trying next)", server, e),
        }
    }
    Err(anyhow!("All NTP servers unreachable. Use --skip-ntp-check to override."))
}
```

## Questions Before Code

- Which NTP crate? `sntp` (small), `rsntp` (also small), or manual wire protocol?
- CLI flag name: `--skip-ntp-check` or `--no-clock-check` or something else?
- Where to call it: in `main.rs` before swarm boot, or in `LatticeNode::new()`?
