use std::cmp::Ordering;
use std::rc::Rc;

use crate::eval::EvalError;
use crate::lisp_value::{Bool, LispValue};

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
    let first = arguments[0].get_number()?;
    let res = arguments
        .iter()
        .skip(1)
        .try_fold(true, |acc, x| Ok(acc && first.eq(x.get_number()?)))?;
    let lisp_result = match res {
        false => Bool::False,
        true => Bool::True,
    };
    Ok(Rc::new(LispValue::Bool(lisp_result)))
}

pub fn gt(arguments: &[Rc<LispValue>]) -> Result<Rc<LispValue>, EvalError> {
    let first = arguments[0].get_number()?;
    for left_hand in arguments.iter().skip(1) {
        match first.partial_cmp(left_hand.as_ref().get_number()?) {
            Some(Ordering::Greater) => {}
            _ => return Ok(Rc::new(LispValue::Bool(Bool::False))),
        }
    }
    Ok(Rc::new(LispValue::Bool(Bool::True)))
}

pub fn lt(arguments: &[Rc<LispValue>]) -> Result<Rc<LispValue>, EvalError> {
    let first = arguments[0].get_number()?;
    for left_hand in arguments.iter().skip(1) {
        match first.partial_cmp(left_hand.as_ref().get_number()?) {
            Some(Ordering::Less) => {}
            _ => return Ok(Rc::new(LispValue::Bool(Bool::False))),
        }
    }
    Ok(Rc::new(LispValue::Bool(Bool::True)))
}
