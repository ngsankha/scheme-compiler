extern crate cc;
extern crate compiler;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let src = match env::var("SRC") {
        Ok(src) => src,
        _ => return
    };
    println!("cargo:rerun-if-env-changed=SRC");

    let ast = language::parse_from_file(&src);
    let compiled = compiler::asm::asm_to_string(compiler::compile(ast));

    let path = Path::new("compiled.s");
    let mut file = File::create(&path).unwrap();
    file.write_all(compiled.as_bytes()).unwrap();

    cc::Build::new()
        .file("compiled.s")
        .compile("compiledlib");
}
