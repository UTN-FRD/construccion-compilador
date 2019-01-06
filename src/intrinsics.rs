use std::rc::Rc;

use lisp_value::LispValue;

pub fn add(arguments: &Vec<Rc<LispValue>>) -> Rc<LispValue> {
    let res = arguments
        .iter()
        .fold(0f64, |acc, x| acc + x.unwrap_number());

    return Rc::new(LispValue::Num(res));
}
