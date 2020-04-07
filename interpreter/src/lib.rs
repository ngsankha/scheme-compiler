use language::ast::Expr;
use language::ast::Expr::*;

pub fn interp(e: Expr) -> u64 {
    match e {
        EInt(n) => n,
        EAdd1(e0) => interp(*e0) + 1,
        ESub1(e0) => interp(*e0) - 1,
        EIf(e0, e1, e2) => if interp(*e0) == 0 {
            interp(*e1)
        } else {
            interp(*e2)
        }
    }
}

pub fn to_human_str(r: u64) -> String {
    format!("{}\n", r.to_string())
}
