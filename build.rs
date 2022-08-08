use std::{env, path::PathBuf};

fn main() {
    let proto_file = "./proto/bookstore.proto";

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("Failed to compile proto{}", e));
    
        println!("cargo:rerun-if-changed={}", proto_file);
}