fn main() {
    // Re-run on proto changes (required for prost codegen).
    println!("cargo:rerun-if-changed=proto/impact_certificate.proto");
    prost_build::compile_protos(&["proto/impact_certificate.proto"], &["proto/"]).unwrap();

    // Re-run when HEAD moves so build_commit stays current.
    // .git/HEAD changes on branch switch; .git/refs/heads changes on commit.
    // These are best-effort — packed refs may not trigger, but they catch
    // the common cases.  The dirty-tree check below catches the rest.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");

    // Capture git commit hash for build_commit endpoint.
    let hash = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Detect uncommitted changes — a binary built from dirty source
    // reporting a clean commit hash is the failure mode that makes
    // build_commit untrustworthy.
    let dirty = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    let commit = if dirty {
        format!("{}-dirty", hash)
    } else {
        hash
    };

    println!("cargo:rustc-env=BUILD_COMMIT={}", commit);
}
