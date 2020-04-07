use crate::ast::Expr;
use crate::ast::Expr::*;
use crate::asm::Asm;
use crate::asm::Asm::*;
use crate::asm::Register::*;

struct Context {
    gensym_count: u64
}

pub fn compile(e: Expr) -> Vec<Asm> {
    let mut ctx = Context { gensym_count: 0 };
    let mut compiled = compile_expr(&mut ctx, e);
    compiled.push(Ret);
    compiled
}

fn compile_expr(ctx: &mut Context, e: Expr) -> Vec<Asm> {
    match e {
        EInt(n) => compile_int(ctx, n),
        EAdd1(e) => compile_add1(ctx, e),
        ESub1(e) => compile_sub1(ctx, e),
        EIf(e0, e1, e2) => compile_if(ctx, e0, e1, e2)
    }
}

fn compile_int(_ctx: &mut Context, n: u64) -> Vec<Asm> {
    vec![Movq(n, Rax)]
}

fn compile_add1(ctx: &mut Context, e0: Box<Expr>) -> Vec<Asm> {
    let mut c0 = compile_expr(ctx, *e0);
    c0.extend(vec![Addq(1, Rax)]);
    c0
}

fn compile_sub1(ctx: &mut Context, e0: Box<Expr>) -> Vec<Asm> {
    let mut c0 = compile_expr(ctx, *e0);
    c0.extend(vec![Subq(1, Rax)]);
    c0
}

fn compile_if(ctx: &mut Context, e0: Box<Expr>, e1: Box<Expr>, e2: Box<Expr>) -> Vec<Asm> {
    let c0 = compile_expr(ctx, *e0);
    let c1 = compile_expr(ctx, *e1);
    let c2 = compile_expr(ctx, *e2);
    let l0 = gensym(ctx, "if");
    let l1 = gensym(ctx, "if");

    vec![c0,
        vec![Cmpq(0, Rax)],
        vec![Jne(l0.clone())],
        c1,
        vec![Jmp(l1.clone())],
        vec![Label(l0)],
        c2,
        vec![Label(l1)]]
        .into_iter().flatten().collect()
}

fn gensym(ctx: &mut Context, prefix: &str) -> String {
    ctx.gensym_count += 1;
    format!("{}{}", prefix, ctx.gensym_count)
}
