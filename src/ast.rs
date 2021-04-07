#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom),
    // fn_name, arg_names, body
    DefineFunction(String, Vec<String>, Vec<Expr>),
    // var_value, var_name
    DefineVariable(String, Box<Expr>),
    // cond, then, else
    If(Box<Expr>, Box<Expr>, Box<Option<Expr>>),
}

impl Expr {
    pub fn is_atom(&self) -> bool {
        match self {
            Expr::Atom(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Expr::List(_) => true,
            _ => false,
        }
    }

    pub fn expect_atom(self, error_message: &str) -> Atom {
        match self {
            Expr::Atom(atom) => atom,
            _ => panic!("{}", error_message),
        }
    }

    pub fn expect_list(self, error_message: &str) -> Vec<Expr> {
        match self {
            Expr::List(list) => list,
            _ => panic!("{}", error_message),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Id(String),
    Number(f64),
    StringAtom(String),
}

impl Atom {
    pub fn expect_id(self, error_message: &str) -> String {
        match self {
            Atom::Id(id) => id,
            _ => panic!("{}", error_message),
        }
    }
}
