pub mod asm;
mod compile;

extern crate language;
use language::ast::Expr;

use crate::asm::Asm;

pub fn compile(e: Expr) -> Vec<Asm> {
    compile::compile(e)
}
