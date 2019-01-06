#[derive(Debug, Clone)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom),
}

#[derive(Debug, Clone)]
pub enum Atom {
    Id(String),
    Num(f64),
}
