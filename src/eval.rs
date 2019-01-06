use intrinsics;
use lisp_value::LispValue;
use std::rc::Rc;

use ast::{Atom, Expr};
use env::Env;

pub fn eval_program(program: &Vec<Expr>, env: Rc<Env>) -> Vec<Rc<LispValue>> {
    debug!("eval {:?}", program);

    let result: Vec<Rc<LispValue>> = program
        .iter()
        .map(|child| eval_expression(child, env.clone()))
        .collect();

    for res in &result {
        println!("RESULT {:?}", res)
    }

    debug!("eval_program END");
    return result;
}

pub fn eval_expression(expression: &Expr, env: Rc<Env>) -> Rc<LispValue> {
    debug!("eval_expression {:?}", expression);

    match expression {
        Expr::List(list) => {
            debug!("eval_expression END");
            return eval_list(list, env);
        }

        Expr::Atom(atom) => {
            debug!("eval_expression END");
            return eval_atom(atom, env);
        }
    }
}

pub fn eval_list(list: &Vec<Expr>, env: Rc<Env>) -> Rc<LispValue> {
    debug!("eval_list {:?}", list);
    if list.len() == 0 {
        return Rc::new(LispValue::None);
    }

    let mut parts: Vec<Rc<LispValue>> = list
        .iter()
        .map(|child| eval_expression(child, env.clone()))
        .collect();
    let func = parts.remove(0);
    if let LispValue::Intrinsic(func) = *func {
        let arguments = parts;
        //TODO how to make funcs env aware?
        let res = func(&arguments);
        debug!("eval_list END");
        return res;
    } else {
        panic!("MOFO")
    }
}

pub fn eval_atom(atom: &Atom, env: Rc<Env>) -> Rc<LispValue> {
    debug!("eval_atom {:?}", atom);
    match atom {
        Atom::Num(num) => Rc::new(LispValue::Num(*num)),
        Atom::Id(id) => return env.get(id).expect("Symbol not found").clone(),
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
