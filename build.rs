fn main() {
    println!("cargo:rerun-if-changed=proto/impact_certificate.proto");
    prost_build::compile_protos(&["proto/impact_certificate.proto"], &["proto/"]).unwrap();
}
