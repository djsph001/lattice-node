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

/// Verify a build-result claim by checking out bound_commit and building.
fn verify_build(
    local_key: &identity::Keypair,
    claim_id: &str,
    claim_type: &str,
    bound_commit: &str,
) -> ApiResponse {
    let _ = std::process::Command::new("git")
        .args(["stash", "push", "--include-untracked", "-m", "verify-before-sign"])
        .output();
    let _ = std::process::Command::new("git")
        .args(["checkout", bound_commit])
        .output();
    let build = std::process::Command::new("cargo")
        .args(["build", "--workspace"])
        .output();
    let _ = std::process::Command::new("git")
        .args(["checkout", "-"])
        .output();
    let _ = std::process::Command::new("git")
        .args(["stash", "pop"])
        .output();

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
}
