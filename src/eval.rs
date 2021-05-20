//TODO
//- create a proper debug infra with a better strucure
//- probably we would want to generate an id for each function instance to see how they open and close
//- better structure for what and how we log it, arguments, return values, env changes, etc
//
//
//TODO
//- instead of Panic! on every error it would nice to a hace Result infra
//
use crate::ast::{Atom};
use crate::env::Env;
use crate::lisp_value::{Bool, Func, LispValue};
use std::rc::Rc;

use crate::{tokenize, Token};
use crate::{parse, Expr};

// TODO: The `env` parameter is pretty ugly, but Rust doesn't (currently) support default
// arguments.
//
// TODO: This function is still doing a lot of things instead of simply evaluating a syntax
// tree.
#[allow(dead_code)]
pub fn eval(source: &str, env: Option<Rc<Env>>) -> Vec<Rc<LispValue>> {
    // Convert the input string into a stream of tokens and their start & end positions.
    let tokens = tokenize(source);

    // Discard start & end positions from the vector of tuples, leaving only `Token`s.
    let tokens = tokens.into_iter().map(|(_, token, _)| token).collect();
    debug!("tokens: {:?}", tokens);

    // Convert the stream of tokens into an AST.
    let ast = parse(tokens);
    debug!("ast {:?}", ast);

    // TODO: Is this assert's location correct? What happens if this is called expecting a
    // failure when testing?
    assert!(ast.is_ok(), "Syntax error {:?}", ast);

    // NOTE: Made some changes (mainly, `global_env` became `env`), and the call to `clone`
    // disappeared.
    //
    // Prefer `unwrap_or_else` over `unwrap_or` due to lazy evaluation. If we have an `env` value
    // already, there's no need to create a new environment.
    let env = env.unwrap_or_else(|| Rc::new(Env::new_global()));
    debug!("env {:?}", env);

    // Evaluate the AST.
    let result = eval_program(&ast.unwrap(), env.clone());
    debug!("result {:?}", result);

    result
}

pub fn eval_program(program: &[Expr], env: Rc<Env>) -> Vec<Rc<LispValue>> {
    debug!("eval {:?}", program);

    let result: Vec<Rc<LispValue>> = program
        .iter()
        .map(|expr| eval_expression(expr, env.clone()))
        .collect();

    debug!("eval_program END");
    result
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

    debug!("eval_expression END {:?}", result);
    result
}

// evaluates a list
pub fn eval_list(list: &[Expr], env: Rc<Env>) -> Rc<LispValue> {
    debug!("eval_list {:?}", list);
    if list.is_empty() {
        return Rc::new(LispValue::Nil);
    }

    let mut list = list.to_vec();
    let first = list.remove(0);

    match first {
        Expr::Atom(atom) => {
            let id = atom.expect_id("Unexpected non id");
            let func = env
                .get(&id)
                .unwrap_or_else(|| panic!("Symbol `{}` not found", id));
            let arg_values: Vec<Rc<LispValue>> = list
                .iter()
                .map(|expr| eval_expression(expr, env.clone()))
                .collect();

            match *func {
                LispValue::Intrinsic(ref func) => {
                    let res = func(&arg_values);
                    debug!("eval_list END Intrinsice {:?}", res);
                    res
                }

                LispValue::Func(ref func) => {
                    let res = func.call(arg_values);
                    debug!("eval_list END FUNC {:?}", res);
                    res
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
        Atom::Number(num) => Rc::new(LispValue::Number(*num)),
        Atom::StringAtom(s) => Rc::new(LispValue::StringValue(s.to_string())),
        Atom::Id(id) => match id.as_str() {
            "true" => Rc::new(LispValue::Bool(Bool::True)),
            "false" => Rc::new(LispValue::Bool(Bool::False)),
            _ => env
                .get(&id)
                .unwrap_or_else(|| panic!("Symbol {} not found", id)),
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

    Rc::new(LispValue::Nil)
}

pub fn eval_define_variable(var_name: &str, var_value: &Expr, env: Rc<Env>) -> Rc<LispValue> {
    let value = eval_expression(var_value, env.clone());
    env.set(var_name.to_string(), value);

    Rc::new(LispValue::Nil)
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
            Bool::True => eval_expression(positive, env),
            Bool::False => {
                if negative.is_none() {
                    return Rc::new(LispValue::Nil);
                }

                eval_expression(negative.as_ref().unwrap(), env)
            }
        }
    } else {
        panic!("Still don't know how to coerce")
    }
}
