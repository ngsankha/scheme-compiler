extern crate nom;

pub mod ast;
pub mod parser;
pub mod dtypes;

use std::fs;

pub fn parse_from_file(filename: &str) -> ast::Expr {
    let src = fs::read_to_string(filename).unwrap();
    match parser::parse(&src) {
      Ok((rem, expr)) => {
        if rem.is_empty() {
          expr
        } else {
          panic!(format!("expected empty string, got {}", rem))
        }
      },
      Err(err) => panic!(err.to_string())
    }
}
