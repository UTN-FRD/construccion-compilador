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
        matches!(self, Expr::Atom(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Expr::List(_))
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
