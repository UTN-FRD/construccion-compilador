use crate::eval::eval;
use crate::lisp_value::LispValue;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LispVal(Vec<Rc<LispValue>>);

#[wasm_bindgen]
impl LispVal {
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue) -> Result<LispVal, JsValue> {
        let parse_error = Err(JsValue::from_str("Parsing Failed"));
        if let Some(s) = value.as_string() {
            eval(&*s).map_or(parse_error, |val| Ok(LispVal(val)))
        } else {
            parse_error
        }
    }

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:?}", self.0.last().unwrap())
    }
}
