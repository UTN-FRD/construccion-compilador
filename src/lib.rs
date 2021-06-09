#[macro_use]
extern crate log;
extern crate env_logger;

mod parser;
mod token;

// TODO: Is it convenient to expose `Token` and `Expr` directly instead of the module?
pub use ast::Atom;
pub use ast::Expr;
pub use parser::parse;
pub use token::tokenize;
pub use token::Token;

use std::rc::Rc;

pub mod ast;
pub mod env;
pub mod eval;
mod intrinsics;
mod lisp_value;

#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

#[cfg(feature = "wasm")]
pub mod wasm;

pub fn eval_file(file_name: &str) -> Vec<Rc<lisp_value::LispValue>> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    eval::eval(&contents, None)
}
