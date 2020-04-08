use language::ast::Expr;
use language::ast::Expr::*;
use language::dtypes::Val;

pub fn interp(e: Expr) -> Val {
    match e {
        EInt(n) => n.into(),
        EBool(b) => b.into(),
        EAdd1(e0) => interp(*e0) + 1.into(),
        ESub1(e0) => interp(*e0) - 1.into(),
        EZeroh(e0) => (interp(*e0) == 0.into()).into(),
        EIf(e0, e1, e2) => {
            let b = interp(*e0);
            if b.is_truthy() {
                interp(*e1)
            } else {
                interp(*e2)
            }
        }
    }
}

pub fn to_human_str(r: Val) -> String {
    format!("{}\n", r.to_string())
}
