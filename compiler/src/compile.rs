use crate::ast::Expr;
use crate::ast::Expr::*;
use crate::asm::Asm;
use crate::asm::Asm::*;
use crate::asm::Register::*;

pub fn compile(e: Expr) -> Vec<Asm> {
    let mut compiled = compile_expr(e);
    compiled.push(Ret);
    compiled
}

fn compile_expr(e: Expr) -> Vec<Asm> {
    match e {
        EInt(n) => compile_int(n),
        EAdd1(e) => compile_add1(e),
        ESub1(e) => compile_sub1(e)
    }
}

fn compile_int(n: u64) -> Vec<Asm> {
    vec![Movq(n, Rax)]
}

fn compile_add1(e0: Box<Expr>) -> Vec<Asm> {
    let mut c0 = compile_expr(*e0);
    c0.extend(vec![Addq(1, Rax)]);
    c0
}

fn compile_sub1(e0: Box<Expr>) -> Vec<Asm> {
    let mut c0 = compile_expr(*e0);
    c0.extend(vec![Subq(1, Rax)]);
    c0
}
