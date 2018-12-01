#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar); // synthesized by LALRPOP

mod ast;
mod eval;
mod intrinsics;
mod lisp_type;

#[test]
fn test() {
    let _ = env_logger::try_init();
    let parser = grammar::ExprParser::new();
    let result = parser.parse("(+ 1 2)");
    println!("{:?}", result);
    assert!(result.is_ok());

    let result = eval::eval_expression(&result.unwrap());
    println!("{:?}", result);
}
