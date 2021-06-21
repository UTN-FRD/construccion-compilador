#[macro_use]
extern crate log;
extern crate env_logger;

mod parser;
mod token;

// TODO: Is it convenient to expose `Token` and `Expr` directly instead of the module?
pub use ast::Atom;
pub use ast::Expr;
use eval::EvalError;
pub use parser::parse;
pub use token::tokenize;
pub use token::Token;

pub mod ast;
pub mod env;
pub mod eval;
mod intrinsics;
mod lisp_value;

#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

#[cfg(feature = "wasm")]
pub mod wasm;

pub type ParseError<'a> = lalrpop_util::ParseError<(), token::Token<'a>, &'static str>;

#[derive(Debug)]
pub enum LispError<'a> {
    // TODO: add a LexerError
    ParserError(ParseError<'a>),
    EvaluationError(EvalError),
}
