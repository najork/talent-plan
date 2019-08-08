use std::env;
extern crate prost_build;

fn main() {
    env::set_var("OUT_DIR", "proto");
    prost_build::compile_protos(&["proto/msg.proto"], &["proto"]).unwrap();
    println!("cargo:rerun-if-changed=proto");
}
