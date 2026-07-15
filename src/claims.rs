// Phase 12 — SubmitClaim verify-before-sign module
//
// Handles claim submission through the UDS API. The node verifies STATE
// claims before signing — it's a verifier, not an oracle.
//
// Architecture:
//   Agent submits unsigned claim via UDS → node classifies → if STATE,
//   runs registered check at bound_commit → if true, signs+stakes →
//   if false, refuses → if uncheckable, refuses. If JUDGMENT, signs
//   without stake.
//
// TODO: verify_build currently uses git stash/checkout which is destructive
// in test contexts. Replace with git worktree for safe concurrent checkout.

use hex;
use libp2p::identity;

use std::sync::atomic::{AtomicU64, Ordering};

static WORKTREE_COUNTER: AtomicU64 = AtomicU64::new(0);

use crate::api::ApiResponse;

/// Verify and sign a STATE claim. Never signs unconfirmed claims.
pub fn handle_state_claim(
    local_key: &identity::Keypair,
    claim_id: &str,
    claim_type: &str,
    bound_commit: &str,
) -> ApiResponse {
    let exists = std::process::Command::new("git")
        .args(["cat-file", "-e", bound_commit])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !exists {
        return ApiResponse::ClaimRefused {
            claim_id: claim_id.into(),
            reason: format!("bound_commit {} not found", bound_commit),
            refused_because: "uncheckable".into(),
        };
    }

    match claim_type {
        "build-result" => verify_build(local_key, claim_id, claim_type, bound_commit),
        _ => ApiResponse::ClaimRefused {
            claim_id: claim_id.into(),
            reason: format!("no registered verifier for '{}'", claim_type),
            refused_because: "uncheckable".into(),
        },
    }
}

/// Verify a build-result claim by checking out bound_commit in a temporary
/// git worktree. Uses `git worktree` to avoid touching the working tree.
fn verify_build(
    local_key: &identity::Keypair,
    claim_id: &str,
    claim_type: &str,
    bound_commit: &str,
) -> ApiResponse {
    // Unique temp dir — commit hash prefix + random suffix for concurrency safety
    let hash_prefix = &bound_commit[..8.min(bound_commit.len())];
    let ctr = WORKTREE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_dir = std::env::temp_dir()
        .join(format!("lattice-verify-{}-{}-{}", hash_prefix, std::process::id(), ctr));
    let temp_path = temp_dir.to_str().unwrap_or("");

    // git worktree add <tempdir> <commit>
    let add = std::process::Command::new("git")
        .args(["worktree", "add", "--detach", temp_path, bound_commit])
        .output();

    let build = match add {
        Ok(ref o) if o.status.success() => {
            std::process::Command::new("cargo")
                .args(["build", "--workspace"])
                .current_dir(&temp_dir)
                .output()
        }
        Ok(_) => {
            // worktree add failed — clean up and return uncheckable
            let _ = std::fs::remove_dir_all(&temp_dir);
            return ApiResponse::ClaimRefused {
                claim_id: claim_id.into(),
                reason: format!("uncheckable: cannot create worktree for {}", bound_commit),
                refused_because: "uncheckable".into(),
            };
        }
        Err(e) => {
            return ApiResponse::ClaimRefused {
                claim_id: claim_id.into(),
                reason: format!("uncheckable: {}", e),
                refused_because: "uncheckable".into(),
            };
        }
    };

    // Teardown: remove worktree + prune stale metadata
    let _ = std::process::Command::new("git")
        .args(["worktree", "remove", "--force", temp_path])
        .output();
    let _ = std::process::Command::new("git")
        .args(["worktree", "prune"])
        .output();
    let _ = std::fs::remove_dir_all(&temp_dir);

    match build {
        Ok(o) if o.status.success() => {
            let sig = sign_claim(local_key, claim_id, "STATE", claim_type, bound_commit);
            ApiResponse::ClaimSigned { claim_id: claim_id.into(), signature: sig }
        }
        Ok(o) => ApiResponse::ClaimRefused {
            claim_id: claim_id.into(),
            reason: format!(
                "build failed at {}: {}",
                bound_commit,
                String::from_utf8_lossy(&o.stderr).chars().take(200).collect::<String>(),
            ),
            refused_because: "false".into(),
        },
        Err(e) => ApiResponse::ClaimRefused {
            claim_id: claim_id.into(),
            reason: format!("uncheckable: {}", e),
            refused_because: "uncheckable".into(),
        },
    }
}

/// Sign a JUDGMENT claim without verification. JUDGMENT claims carry no stake.
pub fn handle_judgment_claim(
    local_key: &identity::Keypair,
    claim_id: &str,
    claim_type: &str,
    bound_commit: &str,
) -> ApiResponse {
    let sig = sign_claim(local_key, claim_id, "JUDGMENT", claim_type, bound_commit);
    ApiResponse::ClaimSigned { claim_id: claim_id.into(), signature: sig }
}

fn sign_claim(
    local_key: &identity::Keypair,
    claim_id: &str,
    domain_tag: &str,
    claim_type: &str,
    bound_commit: &str,
) -> String {
    let payload = format!("{}|{}|{}|{}", claim_id, domain_tag, claim_type, bound_commit);
    match local_key.sign(payload.as_bytes()) {
        Ok(sig) => hex::encode(sig),
        Err(e) => format!("SIGN_ERROR:{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> identity::Keypair {
        identity::Keypair::generate_ed25519()
    }

    fn current_head() -> String {
        std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default()
    }

    #[test]
    fn true_build_claim_is_signed() {
        let key = test_key();
        let head = current_head();
        assert!(!head.is_empty());

        let resp = handle_state_claim(&key, "c1", "build-result", &head);
        match &resp {
            ApiResponse::ClaimSigned { signature, .. } => {
                assert!(!signature.is_empty());
                assert!(!signature.starts_with("SIGN_ERROR"));
            }
            other => panic!("Expected ClaimSigned, got {:?}", other),
        }
    }

    #[test]
    fn bogus_commit_refused_uncheckable() {
        let key = test_key();
        let resp = handle_state_claim(
            &key, "c2", "build-result",
            "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        );
        match &resp {
            ApiResponse::ClaimRefused { refused_because, reason, .. } => {
                assert_eq!(refused_because, "uncheckable");
                assert!(reason.contains("not found"));
            }
            other => panic!("Expected ClaimRefused(uncheckable), got {:?}", other),
        }
    }

    #[test]
    fn unknown_type_refused_uncheckable() {
        let key = test_key();
        let resp = handle_state_claim(&key, "c3", "no-such-type", &current_head());
        match &resp {
            ApiResponse::ClaimRefused { refused_because, reason, .. } => {
                assert_eq!(refused_because, "uncheckable");
                assert!(reason.contains("no registered verifier"));
            }
            other => panic!("Expected ClaimRefused(uncheckable), got {:?}", other),
        }
    }

    #[test]
    fn judgment_claim_signed() {
        let key = test_key();
        let resp = handle_judgment_claim(&key, "c4", "design-complete", &current_head());
        match &resp {
            ApiResponse::ClaimSigned { signature, .. } => {
                assert!(!signature.is_empty());
            }
            other => panic!("Expected ClaimSigned, got {:?}", other),
        }
    }

    // ── Oracle discriminator tests ────────────────────────────
    // Fixture commits from /tmp/claim-fixture:
    //   PASSING: 0404ff6baf6b6f1b047db663cd9f9f52fbeab672 (builds)
    //   FAILING: 311806f7c0383a94fa6a20d4fe0891d622cba3ea (compile_error!)
    const FIXTURE_PASSING: &str = "0404ff6baf6b6f1b047db663cd9f9f52fbeab672";
    const FIXTURE_FAILING: &str = "311806f7c0383a94fa6a20d4fe0891d622cba3ea";

    fn fixture_available() -> bool {
        std::process::Command::new("git")
            .args(["cat-file", "-e", FIXTURE_FAILING])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    #[test]
    fn false_build_claim_is_refused() {
        if !fixture_available() {
            eprintln!("SKIP: fixture commits not available — run: git fetch /tmp/claim-fixture master");
            return;
        }
        let key = test_key();
        // Submit build-result at the FAILING commit — build genuinely fails
        let resp = handle_state_claim(&key, "c5", "build-result", FIXTURE_FAILING);
        match &resp {
            ApiResponse::ClaimRefused { refused_because, claim_id, .. } => {
                assert_eq!(refused_because, "false",
                    "ORACLE DETECTED: node signed a claim it proved false");
                assert_eq!(claim_id, "c5");
            }
            ApiResponse::ClaimSigned { .. } => {
                panic!("ORACLE: node signed a claim it just proved false!")
            }
            other => panic!("Expected ClaimRefused(false), got {:?}", other),
        }
    }

    #[test]
    fn passing_claim_at_passing_commit_signed() {
        if !fixture_available() {
            eprintln!("SKIP: fixture commits not available");
            return;
        }
        let key = test_key();
        let resp = handle_state_claim(&key, "c6", "build-result", FIXTURE_PASSING);
        match &resp {
            ApiResponse::ClaimSigned { signature, .. } => {
                assert!(!signature.is_empty());
            }
            other => panic!("Expected ClaimSigned at passing commit, got {:?}", other),
        }
    }

    #[test]
    fn verification_does_not_touch_working_tree() {
        if !fixture_available() {
            eprintln!("SKIP: fixture commits not available");
            return;
        }
        let key = test_key();
        let tree_before = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
        assert!(!tree_before.is_empty());

        // Verify a claim at a DIFFERENT repo's commit
        let resp = handle_state_claim(&key, "c7", "build-result", FIXTURE_PASSING);
        assert!(
            matches!(&resp, ApiResponse::ClaimSigned { .. }),
            "Expected ClaimSigned, got {:?}", resp
        );

        // Working tree must still be at the original HEAD
        let tree_after = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
        assert_eq!(tree_before, tree_after,
            "verify_build must not change the working tree's HEAD");
    }

    #[test]
    fn bound_commit_verified_not_head() {
        if !fixture_available() {
            eprintln!("SKIP: fixture commits not available");
            return;
        }
        let key = test_key();
        // FIXTURE_PASSING is a foreign repo (tiny 3-line main.rs).
        // If the node signs this claim, it correctly built the fixture
        // at bound_commit rather than lattice-node's HEAD.
        let resp = handle_state_claim(&key, "c8", "build-result", FIXTURE_PASSING);
        assert!(
            matches!(&resp, ApiResponse::ClaimSigned { .. }),
            "Must verify at bound_commit, not HEAD. Got: {:?}", resp
        );
    }
}
