#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar); // synthesized by LALRPOP

use std::rc::Rc;

mod ast;
mod env;
mod eval;
mod intrinsics;
mod lisp_value;

pub fn main() {}

#[test]
fn main_test() {}
