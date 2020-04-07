pub enum Expr {
    EInt(u64),
    EAdd1(Box<Expr>),
    ESub1(Box<Expr>),
    EIf(Box<Expr>, Box<Expr>, Box<Expr>)
}
