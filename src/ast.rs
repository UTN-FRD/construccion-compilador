#[derive(Debug, Clone)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom),
}

impl Expr {
    pub fn is_atom(&self) -> bool {
        match self {
            Expr::Atom(_) => return true,
            _ => return false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Expr::List(_) => return true,
            _ => return false,
        }
    }

    pub fn unwrap_atom(self) -> Atom {
        match self {
            Expr::Atom(atom) => return atom,
            _ => panic!("ASDASD"),
        }
    }

    pub fn unwrap_list(self) -> Vec<Expr> {
        match self {
            Expr::List(list) => return list,
            _ => panic!("ASDASD"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Id(String),
    Num(f64),
}

impl Atom {
    pub fn unwrap_id(self) -> String {
        match self {
            Atom::Id(id) => return id,
            _ => panic!("ASDKJAHSD"),
        }
    }
}
