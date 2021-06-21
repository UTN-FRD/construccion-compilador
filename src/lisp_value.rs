use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use crate::env::Env;

use crate::Expr;

#[derive(Clone)]
pub enum LispValue {
    Nil,
    #[allow(dead_code)]
    Id(String),
    Number(f64),
    Bool(Bool),
    Intrinsic(fn(&[Rc<LispValue>]) -> Rc<LispValue>),
    Func(Func),
    StringValue(String),
}

impl LispValue {
    pub fn unwrap_number(&self) -> &f64 {
        match self {
            LispValue::Number(ref num) => num,
            _ => panic!("BBBB"),
        }
    }
}

impl PartialEq for LispValue {
    fn eq(&self, other: &LispValue) -> bool {
        use self::LispValue::*;

        match (self, other) {
            (LispValue::Nil, LispValue::Nil) => true,
            (Number(ref n1), Number(ref n2)) => n1 == n2,
            (Id(ref id1), Id(ref id2)) => *id1 == *id2,
            (Bool(ref bool1), Bool(ref bool2)) => bool1 == bool2,
            (StringValue(ref string1), StringValue(ref string2)) => string1 == string2,
            _ => false,
        }
    }
}

impl Eq for LispValue {}

impl Ord for LispValue {
    fn cmp(&self, other: &LispValue) -> Ordering {
        use self::LispValue::*;

        match (self, other) {
            (Number(ref n1), Number(ref n2)) => n1.partial_cmp(n2).unwrap(),
            // TODO is this the right thing to do?
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for LispValue {
    fn partial_cmp(&self, other: &LispValue) -> Option<Ordering> {
        use self::LispValue::*;
        match (self, other) {
            (Number(ref n1), Number(ref n2)) => Some(n1.partial_cmp(n2).unwrap()),
            // TODO is this the right thing to do?
            _ => None,
        }
    }
}

impl fmt::Debug for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispValue::Nil => write!(f, "Nil"),
            LispValue::Intrinsic(_) => write!(f, "intrinsic"),
            LispValue::Func(func) => write!(f, "#func {}", func.get_name()),
            LispValue::Number(num) => write!(f, "{}", num),
            LispValue::Id(str) => write!(f, "{}", str),
            LispValue::Bool(lisp_bool) => match lisp_bool {
                Bool::True => write!(f, "true"),
                Bool::False => write!(f, "false"),
            },
            LispValue::StringValue(string) => write!(f, "\"{}\"", string),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Bool {
    True,
    False,
}

#[derive(Clone)]
pub struct Func {
    name: String,
    arg_names: Vec<String>,
    body: Vec<Expr>,
    env: Rc<Env>,
}

impl Func {
    pub fn new(name: String, arg_names: Vec<String>, body: Vec<Expr>, env: Rc<Env>) -> Func {
        Func {
            name,
            arg_names,
            body,
            env,
        }
    }

    pub fn get_name(&self) -> &String {
         &self.name
    }

    pub fn get_arg_names(&self) -> &Vec<String> {
         &self.arg_names
    }

    pub fn get_env(&self) -> &Rc<Env> {
         &self.env
    }

    pub fn get_body(&self) -> &Vec<Expr> {
        &self.body
    }
}

impl Ord for Func {
    fn cmp(&self, _other: &Func) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for Func {
    fn partial_cmp(&self, _other: &Func) -> Option<Ordering> {
        None
    }
}

impl PartialEq for Func {
    fn eq(&self, _other: &Func) -> bool {
        false
    }
}

impl Eq for Func {}

impl fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
