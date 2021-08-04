use std::cmp::Ordering;
use std::rc::Rc;

use crate::eval::EvalError;
use crate::lisp_value::{Bool, LispValue};

// TODO: instead of defaulting to 0.0 we should return an error
pub fn add(arguments: &[Rc<LispValue>]) -> Result<Rc<LispValue>, EvalError> {
    let res = arguments
        .iter()
        .try_fold(0.0, |acc, x| Ok(acc + x.get_number()?))?;

    Ok(Rc::new(LispValue::Number(res)))
}

pub fn sub(arguments: &[Rc<LispValue>]) -> Result<Rc<LispValue>, EvalError> {
    let first = arguments[0].get_number()?;

    let res = arguments
        .iter()
        .skip(1)
        .try_fold(*first, |acc, x| Ok(acc - x.get_number()?))?;

    Ok(Rc::new(LispValue::Number(res)))
}

pub fn eq(arguments: &[Rc<LispValue>]) -> Result<Rc<LispValue>, EvalError> {
    let first = &arguments[0];
    for left_hand in arguments.iter().skip(1) {
        if !first.eq(left_hand) {
            return Ok(Rc::new(LispValue::Bool(Bool::False)));
        }
    }

    Ok(Rc::new(LispValue::Bool(Bool::True)))
}

pub fn gt(arguments: &[Rc<LispValue>]) -> Result<Rc<LispValue>, EvalError> {
    let first = &arguments[0];
    for left_hand in arguments.iter().skip(1) {
        match first.cmp(left_hand) {
            Ordering::Greater => {}
            _ => return Ok(Rc::new(LispValue::Bool(Bool::False))),
        }
    }
    Ok(Rc::new(LispValue::Bool(Bool::True)))
}

pub fn lt(arguments: &[Rc<LispValue>]) -> Result<Rc<LispValue>, EvalError> {
    let first = &arguments[0];
    for left_hand in arguments.iter().skip(1) {
        match first.cmp(left_hand) {
            Ordering::Less => {}
            _ => return Ok(Rc::new(LispValue::Bool(Bool::False))),
        }
    }
    Ok(Rc::new(LispValue::Bool(Bool::True)))
}
