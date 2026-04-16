use std::io::prelude::*;
use std::io::*;

pub mod runtime;

use protoc_gen_arkts::*;
use crate::compile::compile;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "--version" || args[1] == "-v" {
            println!("protoc-gen-arkts {}", env!("CARGO_PKG_VERSION"));
            return;
        }
    }

    let mut buffer: Vec<u8> = Vec::new();
    stdin()
        .read_to_end(&mut buffer)
        .expect("expected data in stdin");

    let bytes = compile(buffer);

    stdout().write(&bytes).unwrap();
}

