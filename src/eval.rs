use lisp_value::{Func, LispValue};
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

    match expression {
        Expr::List(list) => {
            debug!("eval_expression END");
            return eval_list(list, env);
        }

        Expr::Atom(atom) => {
            debug!("eval_expression END");
            return eval_atom(atom, env);
        }

        Expr::DefineFunction(fn_name, arg_names, body) => {
            debug!("eval_expression END");
            return eval_define_function(fn_name.clone(), arg_names.clone(), body.clone(), env);
        }
    }
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
        Expr::Atom(ref atom) => match atom {
            //TODO maybe expect_atom ?
            Atom::Id(ref id) => {
                match id.as_str() {
                    //"define" => {
                        //if list[0].is_list() {
                            //// We are defining a func
                            //let func = Func::from_expr(list.clone(), env.clone());
                            //env.set_global(
                                //func.get_name().to_string(),
                                //Rc::new(LispValue::Func(func)),
                            //);

                            //return Rc::new(LispValue::None);
                        //} else {
                            //// we are defining a variable
                            ////TODO
                            //return Rc::new(LispValue::None);
                        //}
                    //}
                    _ => {
                        let func = env.get(id).expect("Symbol not found");
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
                }
            }
            _ => panic!("Unexpected Atom type"),
        },
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
            _ => env.get(&id).expect("Symbol not found"),
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
    env.set_global(func.get_name().clone(), Rc::new(LispValue::Func(func)));

    return Rc::new(LispValue::None);
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
