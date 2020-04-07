extern crate cc;
extern crate compiler;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let src = match env::var("SRC") {
      Ok(src) => src,
      _ => panic!("no source files in SRC environment variable")
    };

    let compiled = compiler::asm::asm_to_string(compiler::compile(&src));

    let path = Path::new("compiled.s");
    let mut file = File::create(&path).unwrap();
    file.write_all(compiled.as_bytes()).unwrap();

    cc::Build::new()
      .file("compiled.s")
      .compile("compiledlib");
}
