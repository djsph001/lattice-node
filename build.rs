fn main() {
    println!("cargo:rerun-if-changed=proto/impact_certificate.proto");
    prost_build::compile_protos(&["proto/impact_certificate.proto"], &["proto/"]).unwrap();

    // Capture git commit hash for build_commit endpoint.
    let commit = std::process::Command::new("git")
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
    println!("cargo:rustc-env=BUILD_COMMIT={}", commit);
}
