use intrinsics;
use lisp_type::LispType;

use ast::{Atom, Expr, Op};

pub fn eval_program(program: &Vec<Expr>) -> Vec<LispType> {
    debug!("eval {:?}", program);

    let result: Vec<LispType> = program.iter().map(|child| eval_expression(child)).collect();

    for res in &result {
        println!("RESULT {:?}", res)
    }

    debug!("eval_program END");
    return result;
}

pub fn eval_expression(expression: &Expr) -> LispType {
    debug!("eval_expression {:?}", expression);

    match expression {
        Expr::List(list) => {
            debug!("eval_expression END");
            return eval_list(list);
        }

        Expr::Atom(atom) => {
            debug!("eval_expression END");
            return eval_atom(atom);
        }
    }
}

pub fn eval_list(list: &Vec<Expr>) -> LispType {
    debug!("eval_list {:?}", list);
    if list.len() == 0 {
        return LispType::None;
    }

    let mut parts: Vec<LispType> = list.iter().map(|child| eval_expression(child)).collect();
    let func = parts.remove(0);
    let func = func.unwrap_fn();
    let arguments = parts;
    let res = func(&arguments);
    debug!("eval_list END");
    return res;
}

pub fn eval_atom(atom: &Atom) -> LispType {
    debug!("eval_atom {:?}", atom);
    match atom {
        Atom::Num(num) => LispType::Num(*num),
        Atom::Id(id) => panic!("don know what to do"),
        Atom::Op(op) => {
            // TODO this should look in the env for information
            match op {
                Op::Add => return LispType::Fn(intrinsics::add),
                _ => panic!("WE DONT HAVE ENVS YET DOg"),
            }
        }
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
