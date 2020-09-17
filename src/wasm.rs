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
        if let Some(s) = value.as_string() {
            let result = eval(&*s);
            Ok(LispVal(result))
        } else {
            Err(JsValue::from_str("Parsing Failed"))
        }
    }

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.0[0])
    }
}
