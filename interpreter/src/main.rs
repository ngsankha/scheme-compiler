extern crate language;

mod lib;

use std::env;
use std::fs;
use language::parser;

fn main() {
    let src = match env::var("SRC") {
      Ok(src) => src,
      _ => panic!("no source files in SRC environment variable")
    };

    let prog = fs::read_to_string(src).unwrap();
    match parser::parse(&prog) {
      Ok((rem, expr)) => {
        if rem.is_empty() {
          println!("{}", lib::interp(expr))
        } else {
          panic!(format!("expected empty string, got {}", rem))
        }
      },
      _ => panic!("parse error")
    };
}
