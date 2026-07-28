// ── Objection Injector ──────────────────────────────────────────
//
// Standalone binary that generates ed25519 keypairs, constructs and signs
// Objection payloads targeting a specific claim, publishes them via
// gossipsub to a Lattice mesh, and reports results.
//
// USAGE (minimal):
//   cargo run --bin objection-injector -- \
//     --bootstrap-peer /ip4/127.0.0.1/tcp/4005 \
//     --target-claim-id aabbccdd... \
//     --count 5
//
// DETERMINISTIC KEYPAIRS:
//   Pass --identity-seed <u64> to generate reproducible keypairs from
//   sha256(seed || index).  This enables re-running the same injector
//   and getting the exact same objector identities — useful for
//   duplicate-objection tests (run with same seed, same index).
//
//   Example: to re-create keypair at index 3:
//     cargo run --bin objection-injector -- \
//       --bootstrap-peer ... --target-claim-id ... \
//       --count 1 --identity-seed 42
//   The single keypair produced will match index 1 (seed=42, idx=0).

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::Parser;
use libp2p::{
    futures::StreamExt,
    gossipsub, identity, noise,
    swarm::SwarmEvent,
    tcp, yamux, Multiaddr, PeerId, SwarmBuilder,
};
use sha2::{Digest, Sha256};
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

use lattice_node::ledger::types::{Objection, SignedObjection};

// ── Constants ──────────────────────────────────────────────────

/// Gossipsub topic for objection propagation — MUST match the
/// `LATTICE_OBJECTION_TOPIC` constant in the target Lattice node.
const LATTICE_OBJECTION_TOPIC: &str = "lattice/objection/v1";

/// Maximum retries for re-publishing queued objections before giving up.
/// Each retry cycle processes swarm events to drive mesh formation.
const MAX_RETRY_CYCLES: usize = 60;

/// How long to poll the swarm event loop per retry cycle (drives
/// gossipsub heartbeat so the mesh can form without a fixed delay).
const RETRY_POLL_DURATION: Duration = Duration::from_millis(250);

/// Delay between publishing successive objections (100ms per spec).
const INTER_OBJECTION_DELAY: Duration = Duration::from_millis(100);

// ── CLI ────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
#[command(
    name = "objection-injector",
    version,
    about = "Generate and publish signed objections to a Lattice mesh via gossipsub",
    long_about = "\
Generates `count` distinct ed25519 keypairs, constructs Objection payloads \
targeting --target-claim-id, signs each with its own keypair, and publishes \
them to the Lattice mesh via the /lattice/objection/v1 gossipsub topic.

DETERMINISTIC KEYPAIRS:
  With --identity-seed <u64>, keypairs are derived from sha256(seed || index), \
making them reproducible across runs.  This enables re-running with the same \
seed to duplicate a specific objector identity — useful for testing duplicate- \
objection rejection or capacity limits.  Without --identity-seed, keypairs are \
randomly generated (ed25519)."
)]
struct Cli {
    /// Bootstrap peer multiaddr to connect to
    /// (e.g. /ip4/127.0.0.1/tcp/4005)
    #[arg(long, value_name = "MULTIADDR")]
    bootstrap_peer: String,

    /// Target claim ID as a 32-byte hex string (64 hex characters)
    #[arg(long, value_name = "HEX")]
    target_claim_id: String,

    /// Number of distinct objections to publish
    #[arg(long, default_value_t = 1, value_name = "N")]
    count: u64,

    /// Objection reason text (default: "cap-test")
    #[arg(long, default_value = "cap-test", value_name = "STRING")]
    reason: String,

    /// Optional u64 seed for deterministic keypair generation.
    /// With this set, keypairs are derived from sha256(seed || index),
    /// making them identical across runs.  Without it, keypairs are
    /// randomly generated.
    #[arg(long, value_name = "N")]
    identity_seed: Option<u64>,

    /// Seconds to drive the swarm event loop after publishing all
    /// objections, giving gossipsub time to forward queued messages.
    /// publish() only inserts into the local outbound queue — without
    /// this linger the process exits before messages are sent.
    #[arg(long, default_value_t = 5, value_name = "SECS")]
    linger_secs: u64,
}

// ── Key generation ─────────────────────────────────────────────

/// Derive a deterministic ed25519 keypair from `seed` and `index`.
///
/// Uses sha256(seed_le || index_le) as the 32-byte ed25519 seed,
/// guaranteeing identical output for the same (seed, index) pair.
fn derive_keypair(seed: u64, index: u64) -> identity::Keypair {
    let mut hasher = Sha256::new();
    hasher.update(seed.to_le_bytes());
    hasher.update(index.to_le_bytes());
    let hash: [u8; 32] = hasher.finalize().into();

    // ed25519-dalek 2.x: SigningKey wraps a 32-byte seed; try_from_bytes
    // on libp2p's SecretKey expects the same 32-byte seed.
    let mut seed_copy = hash;
    let secret = identity::ed25519::SecretKey::try_from_bytes(&mut seed_copy)
        .expect("sha256 output is a valid ed25519 seed");
    let ed_kp = identity::ed25519::Keypair::from(secret);
    identity::Keypair::from(ed_kp)
}

/// Generate a random ed25519 keypair.
fn generate_random_keypair() -> identity::Keypair {
    identity::Keypair::generate_ed25519()
}

// ── Objection construction ─────────────────────────────────────

/// Sign an Objection with `kp`, producing a `SignedObjection`.
fn sign_objection(obj: &Objection, kp: &identity::Keypair) -> Result<SignedObjection> {
    let payload = serde_cbor::to_vec(obj).context("serialize objection")?;
    let signature = kp.sign(&payload)?;
    Ok(SignedObjection {
        objection: obj.clone(),
        signer_public_key: kp.public().encode_protobuf(),
        signature,
    })
}

// ── Entry point ────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    // ── Parse inputs ──────────────────────────────────────────

    let bootstrap_addr: Multiaddr = cli
        .bootstrap_peer
        .parse()
        .context("Invalid bootstrap peer multiaddr")?;

    let claim_id_hex = cli.target_claim_id.trim();
    let claim_id_bytes =
        hex::decode(claim_id_hex).context("target-claim-id: invalid hex")?;
    if claim_id_bytes.len() != 32 {
        bail!(
            "target-claim-id must be exactly 32 bytes (64 hex chars), got {} bytes",
            claim_id_bytes.len()
        );
    }
    let mut target_claim_id = [0u8; 32];
    target_claim_id.copy_from_slice(&claim_id_bytes);

    info!(
        bootstrap = %bootstrap_addr,
        claim_id = %cli.target_claim_id,
        count = cli.count,
        reason = %cli.reason,
        seed = ?cli.identity_seed,
        "Objection injector starting"
    );

    // ── Generate keypairs ─────────────────────────────────────

    let keypairs: Vec<(u64, identity::Keypair)> = (0..cli.count)
        .map(|i| {
            let idx = i + 1;
            let kp = if let Some(seed) = cli.identity_seed {
                debug!(index = idx, seed, "Deriving deterministic keypair");
                derive_keypair(seed, i)
            } else {
                let kp = generate_random_keypair();
                let peer_id = PeerId::from(kp.public());
                debug!(index = idx, peer = %peer_id, "Generated random keypair");
                kp
            };
            (idx, kp)
        })
        .collect();

    // ── Build signed objections ───────────────────────────────

    let mut signed_objections: Vec<(u64, String, SignedObjection)> = Vec::new();

    for (idx, kp) in &keypairs {
        let peer_id = PeerId::from(kp.public());
        let objector = peer_id.to_base58();
        let objection = Objection {
            target_claim_id,
            objector: objector.clone(),
            reason: cli.reason.clone(),
            timestamp: Utc::now(),
        };
        let signed = sign_objection(&objection, kp)
            .with_context(|| format!("sign objection for keypair index {idx}"))?;
        signed_objections.push((*idx, objector, signed));
    }

    info!(
        count = signed_objections.len(),
        "Constructed and signed all objections"
    );

    // ── Build swarm ──────────────────────────────────────────
    //
    // Use the first generated keypair as the swarm identity.  The
    // swarm identity is independent of the objection signers — the
    // mesh only authenticates the transport, not the payloads.

    let swarm_identity = keypairs[0].1.clone();
    let local_peer_id = PeerId::from(swarm_identity.public());
    info!(peer_id = %local_peer_id, "Swarm identity");

    let mut swarm = SwarmBuilder::with_existing_identity(swarm_identity)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(1))
                .validation_mode(gossipsub::ValidationMode::Permissive)
                .mesh_outbound_min(1)
                .mesh_n_low(1)
                .mesh_n(2)
                .mesh_n_high(4)
                .build()
                .map_err(|e| anyhow::anyhow!("gossipsub config: {e}"))?;

            let mut gossipsub: gossipsub::Behaviour<gossipsub::IdentityTransform> =
                gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )
                .map_err(|e| anyhow::anyhow!("gossipsub init: {e}"))?;

            let topic = gossipsub::IdentTopic::new(LATTICE_OBJECTION_TOPIC);
            gossipsub
                .subscribe(&topic)
                .map_err(|e| anyhow::anyhow!("subscribe to objection topic: {e}"))?;

            Ok(gossipsub)
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Listen on a random port — we're a client, not a server.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // ── Dial bootstrap peer ──────────────────────────────────

    let dial_start = Instant::now();
    swarm.dial(bootstrap_addr.clone())?;
    info!(addr = %bootstrap_addr, "Dialing bootstrap peer…");

    // Wait for the connection to establish — process swarm events
    // until we see ConnectionEstablished for the bootstrap peer.
    let mut connected = false;
    let dial_timeout = Duration::from_secs(30);
    loop {
        if dial_start.elapsed() > dial_timeout {
            bail!(
                "Timed out after {}s waiting for connection to bootstrap peer",
                dial_timeout.as_secs()
            );
        }
        tokio::select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        if peer_id == bootstrap_addr
                            .iter()
                            .find_map(|p| if let libp2p::multiaddr::Protocol::P2p(h) = p {
                                h.try_into().ok()
                            } else {
                                None
                            })
                            .unwrap_or(peer_id)
                        {
                            // Check if we actually matched the expected peer
                            // (the multiaddr may carry a PeerId)
                        }
                        info!(%peer_id, "Connection established");
                        connected = true;
                    }
                    SwarmEvent::Behaviour(gossipsub::Event::Subscribed { peer_id, topic }) => {
                        debug!(%peer_id, %topic, "Gossipsub subscription confirmed");
                    }
                    SwarmEvent::Behaviour(gossipsub::Event::GossipsubNotSupported { peer_id }) => {
                        warn!(%peer_id, "Peer does not support gossipsub");
                    }
                    _ => {}
                }
                // Break on any connection established
                if connected {
                    break;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                // heartbeat to keep the loop from busy-waiting
            }
        }
    }

    // ── Wait for mesh membership ────────────────────────────────
    // publish() inserts into the local outbound queue only.
    // The message will not be forwarded until the gossipsub mesh
    // has at least one peer on this topic.  Wait for that here
    // rather than relying on timing — same principle as the
    // genesis re-gossip retry on InsufficientPeers (0fbba1e).
    {
        let topic_id = gossipsub::IdentTopic::new(LATTICE_OBJECTION_TOPIC);
        let topic_hash = topic_id.hash();
        let mesh_deadline = Instant::now() + Duration::from_secs(30);
        loop {
            if swarm.behaviour().mesh_peers(&topic_hash).count() > 0 {
                info!("Mesh peer joined — ready to publish");
                break;
            }
            if Instant::now() >= mesh_deadline {
                warn!("Timed out waiting for mesh peer — publishing anyway");
                break;
            }
            tokio::select! {
                _ = swarm.select_next_some() => {},
                _ = tokio::time::sleep(Duration::from_millis(250)) => {},
            }
        }
    }

    info!("Mesh ready — beginning objection publication");

    // ── Publish objections ───────────────────────────────────
    //
    // We publish one at a time.  On InsufficientPeers, we queue
    // the objection for retry.  Retries are driven by the swarm
    // event loop — no fixed delay, the mesh forms at its own pace.

    #[derive(Debug)]
    struct PublishResult {
        index: u64,
        objector: String,
        result: Result<String, String>, // message_id or error
    }

    let mut results: Vec<PublishResult> = Vec::new();
    let topic = gossipsub::IdentTopic::new(LATTICE_OBJECTION_TOPIC);
    let global_deadline = Instant::now() + Duration::from_secs(120);
    let mut publish_idx = 0usize;
    let mut pending: VecDeque<(u64, String, SignedObjection)> = VecDeque::new();
    let mut retry_cycle = 0usize;

    loop {
        // Push any remaining unsent objections into the pending queue
        // (first-time publications are always attempted).
        while publish_idx < signed_objections.len() {
            let (idx, objector, signed) = signed_objections[publish_idx].clone();
            let payload = match serde_cbor::to_vec(&signed) {
                Ok(p) => p,
                Err(e) => {
                    results.push(PublishResult {
                        index: idx,
                        objector,
                        result: Err(format!("serialize: {e}")),
                    });
                    publish_idx += 1;
                    continue;
                }
            };

            match swarm.behaviour_mut().publish(topic.clone(), payload) {
                Ok(message_id) => {
                    info!(
                        index = idx,
                        objector = %objector,
                        message_id = %message_id,
                        "Objection published"
                    );
                    results.push(PublishResult {
                        index: idx,
                        objector,
                        result: Ok(message_id.to_string()),
                    });
                }
                Err(gossipsub::PublishError::InsufficientPeers) => {
                    debug!(
                        index = idx,
                        objector = %objector,
                        "InsufficientPeers — queuing for retry"
                    );
                    pending.push_back((idx, objector, signed));
                }
                Err(e) => {
                    warn!(
                        index = idx,
                        objector = %objector,
                        error = %e,
                        "Publish failed"
                    );
                    results.push(PublishResult {
                        index: idx,
                        objector,
                        result: Err(e.to_string()),
                    });
                }
            }
            publish_idx += 1;

            // Short delay between publications (100ms).
            if publish_idx < signed_objections.len() {
                tokio::time::sleep(INTER_OBJECTION_DELAY).await;
            }
        }

        // Drain pending queue — retry objections that failed with InsufficientPeers.
        let retrying: Vec<_> = pending.drain(..).collect();
        if retrying.is_empty() {
            // Everything published (or hard-failed). Done.
            break;
        }

        if retry_cycle >= MAX_RETRY_CYCLES {
            for (idx, objector, _) in &retrying {
                warn!(
                    index = idx,
                    objector = %objector,
                    "Giving up after {} retry cycles — InsufficientPeers persisted",
                    MAX_RETRY_CYCLES
                );
                results.push(PublishResult {
                    index: *idx,
                    objector: objector.clone(),
                    result: Err(format!(
                        "InsufficientPeers after {MAX_RETRY_CYCLES} retry cycles"
                    )),
                });
            }
            break;
        }

        if Instant::now() > global_deadline {
            for (idx, objector, _) in &retrying {
                warn!(
                    index = idx,
                    objector = %objector,
                    "Global deadline exceeded — giving up"
                );
                results.push(PublishResult {
                    index: *idx,
                    objector: objector.clone(),
                    result: Err("global deadline exceeded".into()),
                });
            }
            break;
        }

        debug!(
            count = retrying.len(),
            retry_cycle,
            "Retrying queued objections"
        );

        // Drive the swarm event loop for a short window to allow
        // gossipsub heartbeats to build the mesh organically.
        // No fixed delay — we process real events until the window
        // expires, then immediately retry.
        let poll_end = Instant::now() + RETRY_POLL_DURATION;
        while Instant::now() < poll_end {
            tokio::select! {
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::Behaviour(gossipsub::Event::Subscribed { peer_id, topic: t }) => {
                            debug!(%peer_id, %t, "Mesh subscription confirmed");
                            // Mesh is forming — reset retry eagerly
                            retry_cycle = retry_cycle.saturating_sub(1);
                        }
                        SwarmEvent::Behaviour(gossipsub::Event::GossipsubNotSupported { peer_id }) => {
                            warn!(%peer_id, "Peer does not support gossipsub");
                        }
                        SwarmEvent::Behaviour(gossipsub::Event::Message { .. }) => {
                            // Ignore inbound messages — we're a publisher only.
                        }
                        _ => {}
                    }
                }
                _ = tokio::time::sleep(Duration::from_millis(50)) => {}
            }
        }

        // Re-try the pending items immediately — no fixed delay.
        for (idx, objector, signed) in &retrying {
            let payload = match serde_cbor::to_vec(signed) {
                Ok(p) => p,
                Err(e) => {
                    results.push(PublishResult {
                        index: *idx,
                        objector: objector.clone(),
                        result: Err(format!("serialize: {e}")),
                    });
                    continue;
                }
            };
            match swarm.behaviour_mut().publish(topic.clone(), payload) {
                Ok(message_id) => {
                    info!(
                        index = idx,
                        objector = %objector,
                        message_id = %message_id,
                        "Objection published (retry cycle {})",
                        retry_cycle
                    );
                    results.push(PublishResult {
                        index: *idx,
                        objector: objector.clone(),
                        result: Ok(message_id.to_string()),
                    });
                }
                Err(gossipsub::PublishError::InsufficientPeers) => {
                    // Still no peers — re-queue for next cycle.
                    pending.push_back((*idx, objector.clone(), signed.clone()));
                }
                Err(e) => {
                    warn!(
                        index = idx,
                        objector = %objector,
                        error = %e,
                        "Publish failed (retry cycle {})",
                        retry_cycle
                    );
                    results.push(PublishResult {
                        index: *idx,
                        objector: objector.clone(),
                        result: Err(e.to_string()),
                    });
                }
            }
        }

        retry_cycle += 1;
    }

    // ── Report ────────────────────────────────────────────────

    println!();
    println!("═══════════════════════════════════════════");
    println!("  OBJECTION INJECTOR — RESULTS");
    println!("═══════════════════════════════════════════");
    println!();

    let mut ok = 0u64;
    let mut failed = 0u64;
    for r in &results {
        match &r.result {
            Ok(msg_id) => {
                println!(
                    "  [{:>3}]  OK     {}  msg_id={}",
                    r.index, r.objector, msg_id
                );
                ok += 1;
            }
            Err(e) => {
                println!(
                    "  [{:>3}]  FAIL   {}  error={}",
                    r.index, r.objector, e
                );
                failed += 1;
            }
        }
    }

    println!();
    println!("  Published: {}  Failed: {}  Total: {}", ok, failed, results.len());
    println!();

    if failed > 0 {
        warn!("{failed} objection(s) failed to publish");
    }

    // ── Linger: drive swarm to deliver queued messages ────────────
    // publish() only inserts into the local outbound queue.
    // Messages are not forwarded until the swarm event loop runs.
    // Drive the loop for linger_secs so gossipsub has time to
    // actually send pending messages before the process exits.
    {
        let linger_deadline = Instant::now() + Duration::from_secs(cli.linger_secs);
        info!(seconds = cli.linger_secs, "Lingering after publish to deliver messages");
        while Instant::now() < linger_deadline {
            tokio::select! {
                _ = swarm.select_next_some() => {},
                _ = tokio::time::sleep_until(linger_deadline.into()) => break,
            }
        }
    }

    Ok(())
}
