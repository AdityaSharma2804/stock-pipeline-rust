//! Build script that compiles the Protobuf definition into Rust sources before building the crate.

fn main() {
    println!("cargo:rerun-if-changed=proto/stock.proto");

    match prost_build::compile_protos(&["proto/stock.proto"], &["proto/"]) {
        Ok(_) => println!("PROTO OK"),
        Err(e) => panic!("PROTO FAILED: {:?}", e),
    }
}
