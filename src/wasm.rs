use crate::lisp_value::LispValue;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::env::Env;
use crate::eval::eval_program;
use crate::lexer::Token;
use crate::parse;

// TODO: Cumulative interpretation (i.e. interpret new code on top of the
// one that has been interpreted). We might need to store an environment
// to do that.
#[wasm_bindgen]
pub struct Interpreter {
    // TODO: Should we store the actual tokens & AST? What about the source
    // code which was used to produce those values?
    json_tokens: String,
    json_ast: String,

    result: Vec<Rc<LispValue>>,
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    // TODO: Improve responses on error. Yield more information.
    pub fn new(value: JsValue) -> Result<Interpreter, JsValue> {
        let source = match value.as_string() {
            Some(v) => v,
            None => return Err(JsValue::from_str("Invalid value.")),
        };

        let tokens = Token::tokenize(&source).collect();

        let json_tokens = match serde_json::to_string(&tokens) {
            Ok(v) => v,
            Err(_) => return Err(JsValue::from_str("Tokens' JSONification failed.")),
        };

        // TODO: Can't we just pass a reference to the tokens? Do they need to
        // be moved?
        let ast = match parse(tokens) {
            Ok(v) => v,
            Err(_) => return Err(JsValue::from_str("Parsing failed.")),
        };
        let json_ast = match serde_json::to_string(&ast) {
            Ok(v) => v,
            Err(_) => return Err(JsValue::from_str("AST's JSONification failed.")),
        };

        let environment = Rc::new(Env::new_global());

        // TODO: Can't we just pass a reference to the environment? Does it
        // need to be moved?
        let result = match eval_program(&ast, environment) {
            Ok(v) => v,
            Err(_) => return Err(JsValue::from_str("Evaluation failed.")),
        };

        Ok(Self {
            json_tokens,
            json_ast,
            result,
        })
    }

    // TODO: Can the front-end build JSONs from Strings? Can we return a
    // better structure?
    #[wasm_bindgen(method, js_name = getTokens)]
    pub fn get_tokens(&self) -> JsValue {
        JsValue::from_str(&self.json_tokens)
    }

    // TODO: Same as `get_tokens`.
    #[wasm_bindgen(method, js_name = getAST)]
    pub fn get_ast(&self) -> JsValue {
        JsValue::from_str(&self.json_ast)
    }

    #[wasm_bindgen(method, js_name = getResult)]
    pub fn get_result(&self) -> String {
        // TODO: Is it correct to only take the last value?
        format!(
            "{:?}",
            self.result.last().unwrap_or(&Rc::new(LispValue::Nil))
        )
    }
}
