use crate::ast::Expr;
use crate::ast::Expr::*;
use crate::asm::Asm;
use crate::asm::Asm::*;
use crate::asm::Register::*;

pub fn compile(e: Expr) -> Vec<Asm> {
    match e {
      EInt(n) => vec![Movq(n, Rax),
          Ret]
    }
}
