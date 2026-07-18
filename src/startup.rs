use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use rsntp::SntpClient;
use std::time::SystemTime;
use tracing::{info, warn};

/// Maximum acceptable clock drift in seconds.
/// Must match the transaction timestamp tolerance in validation.rs (300s).
const CLOCK_DRIFT_THRESHOLD_SECS: i64 = 300;

/// Default NTP servers, tried in order until one responds.
const DEFAULT_NTP_SERVERS: &[&str] = &["time.apple.com", "time.google.com", "pool.ntp.org"];

/// Verify clock sync before node startup.
pub async fn verify_clock_sync(
    ntp_servers: Option<Vec<String>>,
    skip_check: bool,
) -> Result<()> {
    if skip_check {
        warn!("⚠️ --skip-ntp-check enabled — clock verification bypassed");
        return Ok(());
    }

    let servers: Vec<String> = ntp_servers.unwrap_or_else(|| {
        DEFAULT_NTP_SERVERS.iter().map(|s| s.to_string()).collect()
    });

    // Use spawn_blocking since the SNTP client is synchronous.
    let handle = tokio::task::spawn_blocking(move || -> Result<()> {
        for server in &servers {
            match SntpClient::new(server) {
                Ok(client) => match client.synchronize() {
                    Ok(result) => {
                        let ntp_dt: DateTime<Utc> = DateTime::from(result.datetime());
                        let local_dt: DateTime<Utc> = SystemTime::now().into();
                        let drift = (ntp_dt - local_dt).num_seconds();

                        if drift.abs() > CLOCK_DRIFT_THRESHOLD_SECS {
                            let direction = if drift > 0 { "behind" } else { "ahead" };
                            bail!(
                                "Clock drift too large: local clock is {}s {} of {} (threshold ±{}s).\n\
                                 Sync your clock and restart.\n\
                                 macOS:   sudo sntp -sS {}\n\
                                 Linux:   sudo ntpdate {}  (or: sudo chronyc -a makestep)\n\
                                 Windows: w32tm /resync /force\n\
                                 To bypass: --skip-ntp-check",
                                drift.abs(), direction, server,
                                CLOCK_DRIFT_THRESHOLD_SECS, server, server,
                            );
                        }

                        info!(
                            "✅ Clock verified against {}: drift {}s (threshold ±{}s)",
                            server, drift, CLOCK_DRIFT_THRESHOLD_SECS
                        );
                        return Ok(());
                    }
                    Err(e) => {
                        warn!("NTP query to {} failed: {} (trying next)", server, e);
                    }
                },
                Err(e) => {
                    warn!("Failed to create NTP client for {}: {} (trying next)", server, e);
                }
            }
        }
        bail!(
            "Could not reach any NTP server (tried: {}). Check network or use --skip-ntp-check.",
            servers.join(", ")
        );
    });

    // Propagate both the join error and the inner Result
    handle.await.map_err(|e| anyhow::anyhow!("NTP check panicked: {}", e))??;
    Ok(())
}
