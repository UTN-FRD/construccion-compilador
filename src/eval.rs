use lisp_value::{Bool, Func, LispValue};
use std::rc::Rc;

use ast::{Atom, Expr};
use env::Env;

pub fn eval_program(program: &Vec<Expr>, env: Rc<Env>) -> Vec<Rc<LispValue>> {
    debug!("eval {:?}", program);

    let result: Vec<Rc<LispValue>> = program
        .iter()
        .map(|expr| eval_expression(expr, env.clone()))
        .collect();

    //for res in &result {
    //println!("RESULT {:?}", res)
    //}

    debug!("eval_program END");
    return result;
}

pub fn eval_expression(expression: &Expr, env: Rc<Env>) -> Rc<LispValue> {
    debug!("eval_expression {:?}", expression);

    let result = match expression {
        Expr::List(list) => eval_list(list, env),

        Expr::Atom(atom) => eval_atom(atom, env),

        Expr::DefineFunction(fn_name, arg_names, body) => {
            eval_define_function(fn_name.clone(), arg_names.clone(), body.clone(), env)
        }

        Expr::DefineVariable(name, value) => eval_define_variable(name, value, env),

        Expr::If(cond, positive, negative) => eval_if(cond, positive, negative, env),
    };

    debug!("eval_expression END");
    return result;
}

pub fn eval_list(list: &Vec<Expr>, env: Rc<Env>) -> Rc<LispValue> {
    debug!("eval_list {:?}", list);
    if list.len() == 0 {
        return Rc::new(LispValue::None);
    }

    //TODO
    let mut list = list.clone();
    let first = list.remove(0);

    match first {
        Expr::Atom(atom) => {
            let id = atom.expect_id("Unexpected non id");
            let func = env.get(&id).expect(&format!("Symbol `{}` not found", id));
            let arg_values: Vec<Rc<LispValue>> = list
                .iter()
                .map(|expr| eval_expression(expr, env.clone()))
                .collect();

            match *func {
                LispValue::Intrinsic(ref func) => {
                    let res = func(&arg_values);
                    debug!("eval_list END");
                    return res;
                }

                LispValue::Func(ref func) => {
                    let res = func.call(arg_values);
                    debug!("eval_list END");
                    return res;
                }
                _ => panic!("Unexpected Value in the Function name position"),
            }
        }
        //Expr::List(ref list) =>  {
        // evaluate the first element as a list, check what it evaluates to
        // and do something
        //let first = eval_list()
        //}
        _ => panic!("Unhandled"),
    }
}

pub fn eval_atom(atom: &Atom, env: Rc<Env>) -> Rc<LispValue> {
    debug!("eval_atom {:?}", atom);
    match atom {
        Atom::Num(num) => Rc::new(LispValue::Num(*num)),
        Atom::Id(id) => match id.as_str() {
            "true" => Rc::new(LispValue::Bool(Bool::True)),
            "false" => Rc::new(LispValue::Bool(Bool::False)),
            _ => env.get(&id).expect(&format!("Symbol {} not found", id)),
        },
    }
}

pub fn eval_define_function(
    fn_name: String,
    arg_names: Vec<String>,
    body: Vec<Expr>,
    env: Rc<Env>,
) -> Rc<LispValue> {
    let func = Func::new(fn_name, arg_names, body, env.clone());
    env.set(func.get_name().clone(), Rc::new(LispValue::Func(func)));

    return Rc::new(LispValue::None);
}

pub fn eval_define_variable(var_name: &String, var_value: &Expr, env: Rc<Env>) -> Rc<LispValue> {
    let value = eval_expression(var_value, env.clone());
    env.set(var_name.clone(), value);

    return Rc::new(LispValue::None);
}

pub fn eval_if(
    cond: &Expr,
    positive: &Expr,
    negative: &Option<Expr>,
    env: Rc<Env>,
) -> Rc<LispValue> {
    let cond_value = eval_expression(cond, env.clone());
    if let LispValue::Bool(ref value) = *cond_value {
        match value {
            Bool::True => {
                return eval_expression(positive, env.clone());
            }
            Bool::False => {
                if negative.is_none() {
                    return Rc::new(LispValue::None);
                }

                return eval_expression(negative.as_ref().unwrap(), env.clone());
            }
        }
    } else {
        panic!("Still don't know how to coerce")
    }
}

//#[cfg(test)]
//mod tests {
//use super::*;

//#[test]
//fn base_cases() {
//let _ = env_logger::try_init();
//let ast = Parser::new().parse("(+ 1 2)").expect("To be parsed ok");
//let ast_string = format!("{:?}", ast);
//println!("Actual \n {:#?}", ast);
//let results = eval(&ast);
//assert_eq!(results[0].clone().unwrap_number(), 3.0);
//}
//}
