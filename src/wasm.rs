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
            match eval(&*s, None) {
                Ok(value) => Ok(LispVal(value)),
                Err(_eval_error) => Err(JsValue::NULL), // TODO: build the error msg
            }
        } else {
            parse_error
        }
    }

    // avoid clippy warning
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(&self) -> String {
        match self.0.last() {
            Some(val) => format!("{:?}", val),
            None => String::new(),
        }
    }
}
