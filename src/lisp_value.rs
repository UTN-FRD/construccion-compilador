use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use ast::Expr;
use env::Env;
use eval::eval_expression;

#[derive(Clone)]
pub enum LispValue {
    None,
    Num(f64),
    // TODO rename to intrinsics
    Intrinsic(fn(&Vec<Rc<LispValue>>) -> Rc<LispValue>),
}

impl LispValue {
    pub fn unwrap_number(&self) -> &f64 {
        match self {
            LispValue::Num(ref num) => return num,
            _ => panic!("BBBB"),
        }
    }
    //pub fn unwrap_fn(self) -> fn(&Vec<Rc<LispValue>>) -> Rc<LispValue> {
    //match self {
    //LispValue::Fn(function) => return function,
    //_ => panic!("BBBB"),
    //}
    //}
}

impl fmt::Debug for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispValue::None => write!(f, "Nill"),
            LispValue::Intrinsic(_) => write!(f, "Fn"),
            LispValue::Num(num) => write!(f, "{}", num),
            _ => panic!("asdasd"),
        }
    }
}

#[derive(Clone)]
pub struct Func {
    arg_names: Vec<String>,
    body: Expr,
    env: Rc<Env>,
}

impl Func {
    pub fn new(arg_names: Vec<String>, body: Expr, env: Rc<Env>) -> Func {
        Func {
            arg_names,
            body,
            env,
        }
    }

    pub fn call(&self, arg_values: Vec<Rc<LispValue>>) -> Rc<LispValue> {
        let localEnv: HashMap<String, Rc<LispValue>> =
            self.arg_names.clone().into_iter().zip(arg_values).collect();
        let env = Rc::new(Env::new(self.env.clone(), localEnv));

        return eval_expression(&self.body, env);
    }
}
