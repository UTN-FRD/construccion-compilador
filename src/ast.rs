#[derive(Debug)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Id(String),
    Num(f64),
    Op(Op),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
}
