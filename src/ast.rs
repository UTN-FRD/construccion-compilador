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

    pub fn expect_atom(self, error_message: &str) -> Atom {
        match self {
            Expr::Atom(atom) => return atom,
            _ => panic!("{}", error_message),
        }
    }

    pub fn expect_list(self, error_message: &str) -> Vec<Expr> {
        match self {
            Expr::List(list) => return list,
            _ => panic!("{}", error_message),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Id(String),
    Num(f64),
}

impl Atom {
    pub fn expect_id(self, error_message: &str) -> String {
        match self {
            Atom::Id(id) => return id,
            _ => panic!("{}", error_message),
        }
    }
}
