use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use rsntp::SntpClient;
use std::time::SystemTime;
use tracing::{info, warn};

/// Maximum acceptable clock drift for startup (seconds).
/// Must match the transaction timestamp tolerance in validation.rs (300s).
pub const CLOCK_DRIFT_THRESHOLD_SECS: i64 = 300;

/// Drift at which a runtime WARN is emitted (seconds).
pub const NTP_WARN_THRESHOLD_SECS: i64 = 30;

/// Drift at which the node refuses to sign new transactions (seconds).
pub const NTP_REFUSE_SIGN_THRESHOLD_SECS: i64 = 60;

/// How long to cache a successful NTP check before querying again.
pub const NTP_CACHE_TTL_SECS: u64 = 300; // 5 minutes

/// Default NTP servers, tried in order until one responds.
const DEFAULT_NTP_SERVERS: &[&str] = &[
    "pool.ntp.org",
    "time.apple.com",
    "time.google.com",
];

/// Query an NTP server and return the drift (NTP - local) in seconds,
/// or an error if no server responded.
pub async fn check_ntp_drift(
    ntp_servers: Option<Vec<String>>,
) -> Result<i64> {
    let servers: Vec<String> = ntp_servers.unwrap_or_else(|| {
        DEFAULT_NTP_SERVERS.iter().map(|s| s.to_string()).collect()
    });

    let handle = tokio::task::spawn_blocking(move || -> Result<i64> {
        for server in &servers {
            match SntpClient::new(server) {
                Ok(client) => match client.synchronize() {
                    Ok(result) => {
                        let ntp_dt: DateTime<Utc> = DateTime::from(result.datetime());
                        let local_dt: DateTime<Utc> = SystemTime::now().into();
                        let drift = (ntp_dt - local_dt).num_seconds();
                        return Ok(drift);
                    }
                    Err(e) => {
                        warn!("NTP query to {} failed: {} (fallback)", server, e);
                    }
                },
                Err(e) => {
                    warn!("Failed to create NTP client for {}: {} (fallback)", server, e);
                }
            }
        }
        bail!(
            "Could not reach any NTP server (tried: {}). Check network or use --skip-ntp-check.",
            servers.join(", ")
        );
    });

    handle.await.map_err(|e| anyhow::anyhow!("NTP check panicked: {}", e))?
}

/// Verify clock sync before node startup.  Aborts if drift exceeds
/// CLOCK_DRIFT_THRESHOLD_SECS and --skip-ntp-check is not set.
pub async fn verify_clock_sync(
    ntp_servers: Option<Vec<String>>,
    skip_check: bool,
) -> Result<()> {
    if skip_check {
        warn!("--skip-ntp-check enabled — clock verification bypassed");
        return Ok(());
    }

    let drift = check_ntp_drift(ntp_servers.clone()).await?;

    if drift.abs() > CLOCK_DRIFT_THRESHOLD_SECS {
        let servers: Vec<String> = ntp_servers.unwrap_or_else(|| {
            DEFAULT_NTP_SERVERS.iter().map(|s| s.to_string()).collect()
        });
        let direction = if drift > 0 { "behind" } else { "ahead" };
        let server = servers.first().map(|s| s.as_str()).unwrap_or("pool.ntp.org");
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
        "Clock verified: drift {}s (threshold ±{}s)",
        drift, CLOCK_DRIFT_THRESHOLD_SECS
    );
    Ok(())
}
