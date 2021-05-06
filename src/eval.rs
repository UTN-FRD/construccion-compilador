//TODO
//- create a proper debug infra with a better strucure
//- probably we would want to generate an id for each function instance to see how they open and close
//- better structure for what and how we log it, arguments, return values, env changes, etc
//
use crate::ast::{Atom, Expr};
use crate::env::Env;
use crate::grammar;
use crate::lisp_value::{Bool, Func, LispValue};
use crate::token;
use crate::ParseError;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Symbol `{symbol:?}` not found.")]
    SymbolNotFound { symbol: String },
    #[error("Unexpected Value in the function name position.")]
    UnexpectedFunctionValue,
    #[error("Unhandled Error.")]
    UnhandledError,
}

pub fn parse(source: &'_ str) -> Result<Vec<Expr>, ParseError> {
    let mut errors = Vec::new();
    debug!("eval {:?}", source);
    let parser = grammar::ProgramParser::new();
    let tokens = token::tokenize(source);
    let tokens: Vec<token::Token> = tokens.into_iter().map(|(_, tok, _)| tok).collect();
    parser.parse(&mut errors, tokens)
}

#[allow(dead_code)]
pub fn eval(source: &str) -> Result<Vec<Rc<LispValue>>, ParseError> {
    let result = parse(source);
    assert!(result.is_ok(), "Syntax error {:?}", result);
    debug!("ast {:?}", result);

    let global_env = Rc::new(Env::new_global());
    let result = result.map(|value| eval_program(&value, global_env.clone()));
    debug!("env {:?}", global_env);
    debug!("result {:?}", result);

    result
}

pub fn eval_program(program: &[Expr], env: Rc<Env>) -> Vec<Rc<LispValue>> {
    debug!("eval {:?}", program);

    let result: Vec<Rc<LispValue>> = program
        .iter()
        .flat_map(|expr| eval_expression(expr, env.clone()))
        .collect();

    debug!("eval_program END");
    result
}

pub fn eval_expression(expression: &Expr, env: Rc<Env>) -> Result<Rc<LispValue>, EvalError> {
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
pub fn eval_list(list: &[Expr], env: Rc<Env>) -> Result<Rc<LispValue>, EvalError> {
    debug!("eval_list {:?}", list);
    if list.is_empty() {
        return Ok(Rc::new(LispValue::Nil));
    }

    let mut list = list.to_vec();
    let first = list.remove(0);

    match first {
        Expr::Atom(atom) => {
            let id = atom.expect_id("Unexpected non id");
            let func = env.get(&id).ok_or(EvalError::SymbolNotFound {
                symbol: id.to_string(),
            });
            let arg_values: Vec<Rc<LispValue>> = list
                .iter()
                .flat_map(|expr| eval_expression(expr, env.clone()))
                .collect();

            match func.as_deref() {
                Ok(LispValue::Intrinsic(ref func)) => {
                    let res = func(&arg_values);
                    debug!("eval_list END Intrinsice {:?}", res);
                    Ok(res)
                }

                Ok(LispValue::Func(ref func)) => {
                    let res = func.call(arg_values);
                    debug!("eval_list END FUNC {:?}", res);
                    res
                }
                _ => Err(EvalError::UnexpectedFunctionValue),
            }
        }
        //Expr::List(ref list) =>  {
        // evaluate the first element as a list, check what it evaluates to
        // and do something
        //let first = eval_list()
        //}
        _ => Err(EvalError::UnhandledError),
    }
}

pub fn eval_atom(atom: &Atom, env: Rc<Env>) -> Result<Rc<LispValue>, EvalError> {
    debug!("eval_atom {:?}", atom);
    match atom {
        Atom::Number(num) => Ok(Rc::new(LispValue::Number(*num))),
        Atom::StringAtom(s) => Ok(Rc::new(LispValue::StringValue(s.to_string()))),
        Atom::Id(id) => match id.as_str() {
            "true" => Ok(Rc::new(LispValue::Bool(Bool::True))),
            "false" => Ok(Rc::new(LispValue::Bool(Bool::False))),
            _ => env.get(&id).ok_or(EvalError::SymbolNotFound {
                symbol: id.to_string(),
            }),
        },
    }
}

pub fn eval_define_function(
    fn_name: String,
    arg_names: Vec<String>,
    body: Vec<Expr>,
    env: Rc<Env>,
) -> Result<Rc<LispValue>, EvalError> {
    let func = Func::new(fn_name, arg_names, body, env.clone());
    env.set(func.get_name().clone(), Rc::new(LispValue::Func(func)));

    Ok(Rc::new(LispValue::Nil))
}

pub fn eval_define_variable(
    var_name: &str,
    var_value: &Expr,
    env: Rc<Env>,
) -> Result<Rc<LispValue>, EvalError> {
    let res = eval_expression(var_value, env.clone()).map(|val| env.set(var_name.to_string(), val));
    debug!("defined variable result: {:?}", res);

    Ok(Rc::new(LispValue::Nil))
}

pub fn eval_if(
    cond: &Expr,
    positive: &Expr,
    negative: &Option<Expr>,
    env: Rc<Env>,
) -> Result<Rc<LispValue>, EvalError> {
    let cond_value = eval_expression(cond, env.clone());
    if let Ok(LispValue::Bool(ref value)) = cond_value.as_deref() {
        match value {
            Bool::True => eval_expression(positive, env),
            Bool::False => {
                if negative.is_none() {
                    return Ok(Rc::new(LispValue::Nil));
                }
                match negative.as_ref() {
                    Some(bool_expr) => eval_expression(bool_expr, env),
                    None => Err(EvalError::UnhandledError),
                }
            }
        }
    } else {
        Err(EvalError::UnhandledError) // TODO: should we coerce values to booleans?
    }
}
