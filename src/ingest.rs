// Phase 7 — TCP v0.1.0 Sandbox Ingestion
//
// Reads a protobuf-serialized ImpactCertificate from the Python sandbox,
// validates constraint compliance and witness seed determinism, and prepares
// the payload for libp2p gossipsub injection.
//
// The protobuf contract lives at proto/impact_certificate.proto — the
// single source of truth shared between the Python orchestrator (tfb:)
// and this Rust lattice-node (lat:).

use prost::Message;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

// ── Generated protobuf types ──────────────────────────────────────
// prost-build compiles proto/impact_certificate.proto at build time.
// The generated struct lives in the OUT_DIR as impact_certificate.rs.
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

/// Outcome of ingestion: either a valid certificate ready for gossip,
/// or a structured rejection with the specific reason.
#[derive(Debug)]
pub enum IngestResult {
    /// Certificate is valid — ready for gossipsub broadcast.
    Valid(proto::ImpactCertificate),
    /// Rejected: constraint validation failed.
    FailedConstraint(String),
    /// Rejected: witness seed mismatch (sortition tampering detected).
    SeedMismatch { expected: String, found: String },
}

/// Ingest a certificate from a file path produced by the Python sandbox.
///
/// Validation gates (in order):
///   1. Protobuf decode — must parse as valid ImpactCertificate
///   2. Constraint check — georgist_validation must be PASS
///   3. Witness seed — must match deterministic SHA-256 derivation
pub fn ingest_certificate(path: &Path) -> Result<IngestResult, Box<dyn std::error::Error>> {
    let raw = fs::read(path)?;
    let cert = proto::ImpactCertificate::decode(&raw[..])?;

    println!(
        "[lat:ingest] Decoded certificate — proposal_id={}, rounds={}, {} bytes",
        cert.proposal_id,
        cert.debate_rounds.len(),
        raw.len(),
    );

    // Gate 1: Constraint validation must be PASS
    if cert.georgist_validation() != proto::ValidationOutcome::Pass {
        let reason = format!(
            "Constraint validation failed: {:?}",
            cert.georgist_validation()
        );
        println!("[lat:ingest] REJECTED — {}", reason);
        return Ok(IngestResult::FailedConstraint(reason));
    }

    // Gate 2: Deterministic witness seed verification
    //
    // Must match Python's derivation exactly:
    //   sha256(f"{proposal_id}:{rounds}:{outcome_value}").hexdigest()[:16]
    //
    // The outcome_value is the integer discriminant of the ValidationOutcome enum.
    let computed = compute_witness_seed(
        &cert.proposal_id,
        cert.debate_rounds.len() as u32,
        cert.georgist_validation() as i32,
    );

    if computed != cert.witness_seed {
        println!(
            "[lat:ingest] REJECTED — witness seed mismatch\n  computed: {}\n  found:    {}",
            computed, cert.witness_seed,
        );
        return Ok(IngestResult::SeedMismatch {
            expected: computed,
            found: cert.witness_seed.clone(),
        });
    }

    println!(
        "[lat:ingest] ✓ All gates passed — enclave={}, seed={}",
        cert.enclave_id, cert.witness_seed,
    );

    Ok(IngestResult::Valid(cert))
}

/// Re-derive the witness seed deterministically.
///
/// Formula (TCP v0.1.0 §2.3):
///   SHA-256(proposal_id || ":" || num_rounds || ":" || outcome_value)
///   take first 16 hex characters
///
/// This matches the Python orchestrator's derivation exactly.
pub fn compute_witness_seed(proposal_id: &str, num_rounds: u32, outcome: i32) -> String {
    let material = format!("{}:{}:{}", proposal_id, num_rounds, outcome);
    let hash = Sha256::digest(material.as_bytes());
    hex::encode(&hash[..8]) // first 8 bytes = 16 hex chars
}

/// Spawn a background task that watches a directory for new `.pb`
/// certificate files. When a valid certificate appears, it sends the
/// raw protobuf bytes on the provided channel for the event loop to
/// broadcast via gossipsub.
///
/// Uses simple polling (every 2 seconds) to avoid adding a filesystem
/// notification dependency. Tracks already-processed files so each
/// certificate is ingested exactly once.
pub fn spawn_cert_watcher(
    watch_dir: PathBuf,
    tx: mpsc::Sender<Vec<u8>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut seen: HashSet<PathBuf> = HashSet::new();
        let mut interval = time::interval(Duration::from_secs(2));

        // Scan once immediately on startup
        if let Err(e) = scan_dir(&watch_dir, &mut seen, &tx) {
            tracing::warn!("[cert-watcher] Initial scan error: {}", e);
        }

        loop {
            interval.tick().await;
            if let Err(e) = scan_dir(&watch_dir, &mut seen, &tx) {
                tracing::warn!("[cert-watcher] Scan error: {}", e);
            }
        }
    })
}

/// Scan the watch directory for new `.pb` files, validate them, and
/// send valid ones through the channel.
fn scan_dir(
    dir: &Path,
    seen: &mut HashSet<PathBuf>,
    tx: &mpsc::Sender<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            tracing::debug!("[cert-watcher] Cannot read {}: {}", dir.display(), e);
            return Ok(());
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Only process .pb files we haven't seen
        if path.extension().map_or(false, |ext| ext == "pb") && seen.insert(path.clone()) {
            tracing::info!(
                "[cert-watcher] New certificate detected: {}",
                path.display()
            );

            match ingest_certificate(&path) {
                Ok(IngestResult::Valid(_cert)) => {
                    // Read raw bytes for broadcast
                    let raw = fs::read(&path)?;
                    tracing::info!(
                        "[cert-watcher] Certificate validated — {} bytes, sending to event loop",
                        raw.len()
                    );
                    if let Err(e) = tx.try_send(raw) {
                        tracing::warn!(
                            "[cert-watcher] Channel full, dropping cert {}: {}",
                            path.display(),
                            e
                        );
                    }
                }
                Ok(other) => {
                    tracing::warn!(
                        "[cert-watcher] Certificate rejected: {:?}",
                        other
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        "[cert-watcher] Failed to ingest {}: {}",
                        path.display(),
                        e
                    );
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_seed_matches_python() {
        // This seed was produced by the Python orchestrator for:
        //   proposal_id = "6c21ff60459c4608"
        //   rounds = 3
        //   outcome = PASS (enum value 1)
        let seed = compute_witness_seed("6c21ff60459c4608", 3, 1);
        assert_eq!(seed, "b644ae83dae8edc6");
    }

    #[test]
    fn test_ingest_real_certificate_passes_gates() {
        // Feed the actual certificate-miami.pb from the sandbox through
        // the full ingestion pipeline and verify all gates pass.
        let cert_path = Path::new(
            "../thought-partners/sandbox/certificate-miami.pb"
        );

        if !cert_path.exists() {
            eprintln!("Skipping: certificate-miami.pb not found (run sandbox first)");
            return;
        }

        let result = ingest_certificate(cert_path).expect("ingestion should not error");
        match result {
            IngestResult::Valid(cert) => {
                assert_eq!(cert.proposal_id, "6c21ff60459c4608");
                assert_eq!(cert.debate_rounds.len(), 3);
                assert_eq!(cert.witness_seed, "b644ae83dae8edc6");
                println!(
                    "✓ Real certificate ingested — proposal={}, synthesis={} chars",
                    cert.proposal_id,
                    cert.synthesized_text.len(),
                );
            }
            other => panic!("Expected Valid, got {:?}", other),
        }
    }
}
