use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use ast::Expr;
use env::Env;
use eval::eval_expression;

#[derive(Clone)]
pub enum LispValue {
    None,
    Id(String),
    //TODO support Int and Float
    Num(f64),
    Bool(Bool),
    Intrinsic(fn(&Vec<Rc<LispValue>>) -> Rc<LispValue>),
    Func(Func),
}

impl LispValue {
    pub fn unwrap_number(&self) -> &f64 {
        match self {
            LispValue::Num(ref num) => return num,
            _ => panic!("BBBB"),
        }
    }
}

impl PartialEq for LispValue {
    fn eq(&self, other: &LispValue) -> bool {
        use self::LispValue::*;

        match (self, other) {
            (LispValue::None, LispValue::None) => return true,
            (Id(ref id1), Id(ref id2)) => return *id1 == *id2,
            (Bool(ref bool1), Bool(ref bool2)) => return bool1 == bool2,
            _ => return false,
        }
    }
}

impl fmt::Debug for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispValue::None => write!(f, "Nill"),
            LispValue::Intrinsic(_) => write!(f, "intrinsic"),
            LispValue::Func(func) => write!(f, "#func {}", func.get_name()),
            LispValue::Num(num) => write!(f, "{}", num),
            LispValue::Id(str) => write!(f, "{}", str),
            LispValue::Bool(lisp_bool) => match lisp_bool {
                Bool::True => write!(f, "true"),
                Bool::False => write!(f, "false"),
            },
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

    pub fn call(&self, arg_values: Vec<Rc<LispValue>>) -> Rc<LispValue> {
        let local_env: HashMap<String, Rc<LispValue>> =
            self.arg_names.clone().into_iter().zip(arg_values).collect();

        let env = Rc::new(self.env.new(self.env.clone(), local_env));

        // TODO evaluate multiple Expr bodies
        return eval_expression(&self.body[0], env);
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}
