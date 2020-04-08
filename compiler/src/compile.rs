use language::ast::Expr;
use language::ast::Expr::*;
use language::dtypes::{Val, PADDING};
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
        EBool(b) => compile_bool(ctx, b),
        EAdd1(e) => compile_add1(ctx, e),
        ESub1(e) => compile_sub1(ctx, e),
        EZeroh(e) => compile_zeroh(ctx, e),
        EIf(e0, e1, e2) => compile_if(ctx, e0, e1, e2)
    }
}

fn compile_int(_ctx: &mut Context, n: u64) -> Vec<Asm> {
    let v: Val = n.into();
    vec![Movq(v.into(), Rax)]
}

fn compile_bool(_ctx: &mut Context, b: bool) -> Vec<Asm> {
    let v: Val = b.into();
    vec![Movq(v.into(), Rax)]
}

fn compile_add1(ctx: &mut Context, e0: Box<Expr>) -> Vec<Asm> {
    let mut c0 = compile_expr(ctx, *e0);
    let v: Val = 1.into();
    c0.extend(vec![Addq(v.into(), Rax)]);
    c0
}

fn compile_sub1(ctx: &mut Context, e0: Box<Expr>) -> Vec<Asm> {
    let mut c0 = compile_expr(ctx, *e0);
    let v: Val = 1.into();
    c0.extend(vec![Subq(v.into(), Rax)]);
    c0
}

fn compile_zeroh(ctx: &mut Context, e0: Box<Expr>) -> Vec<Asm> {
    let mut c0 = compile_expr(ctx, *e0);
    let l0 = gensym(ctx, "zeroh");
    let t: Val = true.into();
    let f: Val = false.into();
    let z: Val = 0.into();

    c0.extend(vec![Cmpq(z.into(), Rax),
        Movq(f.into(), Rax),
        Jne(l0.clone()),
        Movq(t.into(), Rax),
        Label(l0)]);
    c0
}

fn compile_if(ctx: &mut Context, e0: Box<Expr>, e1: Box<Expr>, e2: Box<Expr>) -> Vec<Asm> {
    let c0 = compile_expr(ctx, *e0);
    let c1 = compile_expr(ctx, *e1);
    let c2 = compile_expr(ctx, *e2);
    let l0 = gensym(ctx, "if");
    let l1 = gensym(ctx, "if");
    let z: Val = 0.into();

    // we handle falsy values by right shifting away the type tag
    // falsy values of every type will be 0 when this is done
    vec![c0,
        vec![Shrq(PADDING, Rax),
        Cmpq(z.into(), Rax),
        Je(l0.clone())],
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
