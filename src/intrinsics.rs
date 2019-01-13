use std::rc::Rc;

use lisp_value::{Bool, LispValue};

pub fn add(arguments: &Vec<Rc<LispValue>>) -> Rc<LispValue> {
    let res = arguments
        .iter()
        .fold(0f64, |acc, x| acc + x.unwrap_number());

    return Rc::new(LispValue::Num(res));
}

pub fn eq(arguments: &Vec<Rc<LispValue>>) -> Rc<LispValue> {
    let first = &arguments[0];
    for left_hand in arguments.iter().skip(1) {
        if !first.eq(left_hand) {
            return Rc::new(LispValue::Bool(Bool::False));
        }
    }

    return Rc::new(LispValue::Bool(Bool::True));
}
