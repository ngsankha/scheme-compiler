extern crate nom;
pub mod asm;
pub mod ast;
pub mod parser;
pub mod compile;

use std::fs;
use crate::asm::Asm;

pub fn compile(filename: &str) -> Vec<Asm> {
    let src = fs::read_to_string(filename).unwrap();
    match parser::parse(&src) {
      Ok((rem, expr)) => {
        if rem.is_empty() {
          compile::compile(expr)
        } else {
          panic!(format!("expected empty string, got {}", rem))
        }
      },
      _ => panic!("parse error")
    }
}
