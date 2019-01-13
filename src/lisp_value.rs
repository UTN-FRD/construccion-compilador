use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use ast::Expr;
use env::Env;
use eval::eval_expression;

#[derive(Clone)]
pub enum LispValue {
    None,
    Reserved(String),
    Id(String),
    Num(f64),
    // TODO rename to intrinsics
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
            LispValue::Intrinsic(_) => write!(f, "intrinsic"),
            LispValue::Func(func) => write!(f, "#func {}", func.get_name()),
            LispValue::Num(num) => write!(f, "{}", num),
            LispValue::Id(str) => write!(f, "{}", str),
            LispValue::Reserved(str) => write!(f, "{}", str),
            _ => panic!("asdasd"),
        }
    }
}

//TODO make this func save their name for later debugging
#[derive(Clone)]
pub struct Func {
    name: String,
    arg_names: Vec<String>,
    body: Expr,
    env: Rc<Env>,
}

impl Func {
    pub fn new(name: String, arg_names: Vec<String>, body: Expr, env: Rc<Env>) -> Func {
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

        return eval_expression(&self.body, env);
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn from_expr(mut parts: Vec<Expr>, env: Rc<Env>) -> Func {
        assert!(
            parts.len() == 2,
            "Wrong number of arguments to create a function"
        );
        let signature = parts.remove(0);
        let body = parts.remove(0);

        let mut signature = signature.expect_list("Define second element should be a list");
        assert!(signature.len() >= 1, "Missing function name");

        let fn_name = signature.remove(0);
        let fn_name = fn_name
            .expect_atom("Function name should be an atom")
            .expect_id("Function name should be an id");
        let arg_names: Vec<String> = signature
            .into_iter()
            .map(|name| {
                name.expect_atom("Function args should be atoms")
                    .expect_id("Function args should be ids")
            }).collect();

        let func = Func::new(fn_name, arg_names, body, env.clone());
        return func;
    }
}
