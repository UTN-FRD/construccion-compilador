use crate::LispError;
//TODO
//- create a proper debug infra with a better strucure
//- probably we would want to generate an id for each function instance to see how they open and close
//- better structure for what and how we log it, arguments, return values, env changes, etc
//
use crate::ast::Atom;
use crate::env::Env;
use crate::lisp_value::{Bool, Func, LispValue};
use crate::tokenize;
use crate::{parse, Expr};
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

// TODO: The `env` parameter is pretty ugly, but Rust doesn't (currently) support default
// arguments.
//
// TODO: This function is still doing a lot of things instead of simply evaluating a syntax
// tree.
pub fn eval(source: &str, env: Option<Rc<Env>>) -> Result<Vec<Rc<LispValue>>, LispError> {
    // Convert the input string into a stream of tokens and their start & end positions.
    let tokens = tokenize(source);

    // Discard start & end positions from the vector of tuples, leaving only `Token`s.
    let tokens = tokens.into_iter().map(|(_, token, _)| token).collect();
    debug!("tokens: {:?}", tokens);

    // Convert the stream of tokens into an AST.
    let ast = parse(tokens);
    debug!("ast {:?}", ast);

    // NOTE: Made some changes (mainly, `global_env` became `env`), and the call to `clone`
    // disappeared.
    //
    // Prefer `unwrap_or_else` over `unwrap_or` due to lazy evaluation. If we have an `env` value
    // already, there's no need to create a new environment.
    let env = env.unwrap_or_else(|| Rc::new(Env::new_global()));
    debug!("env {:?}", env);

    // Evaluate the AST.
    match ast.clone() {
        Ok(exprs) => match eval_program(&exprs, env) {
            Ok(value) => {
                debug!("result {:?}", value);
                Ok(value)
            }
            Err(eval_error) => Err(LispError::EvaluationError(eval_error)),
        },
        Err(parse_error) => Err(LispError::ParserError(parse_error.clone())),
    }
}

pub fn eval_program(program: &[Expr], env: Rc<Env>) -> Result<Vec<Rc<LispValue>>, EvalError> {
    debug!("eval {:?}", program);

    let result: Result<Vec<Rc<LispValue>>, EvalError> = program
        .iter()
        .map(|expr| eval_expression(expr, env.clone()))
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

                // The matched function isn't part of the compiler's intrinsic functions.
                Ok(LispValue::Func(ref func)) => {
                    let arg_names = func.get_arg_names().clone();

                    // TODO: Don't we need to bring `std::collections::HashMap` to scope?
                    //
                    // TODO: Why so many `Env`s? Can we simplify this?
                    let local_env = arg_names.into_iter().zip(arg_values).collect();
                    let func_env = func.get_env();
                    let env = Rc::new(func_env.new(func_env.clone(), local_env));

                    // TODO: Is it correct to take only the first element of the resulting
                    // vector?
                    //
                    // TODO: Can this evaluate multiple `Expr` bodies?
                    let res = eval_program(func.get_body(), env)?;
                    Ok(res[0].clone())
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
