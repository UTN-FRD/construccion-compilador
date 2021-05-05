#[macro_use]
extern crate log;
extern crate env_logger;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
); // synthesized by LALRPOP

pub mod ast;
pub mod env;
pub mod eval;
pub mod intrinsics;
pub mod lisp_value;
pub mod token;

#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

#[cfg(feature = "wasm")]
pub mod wasm;

pub type ParseError<'a> = lalrpop_util::ParseError<(), token::Token<'a>, &'static str>;

// TODO: This function should be exposed through Javascript.
pub fn parse(tokens: Vec<token::Token>) -> Result<Vec<ast::Expr>, ParseError> {
    let mut errors = Vec::new();
    let parser = grammar::ProgramParser::new();

    return parser.parse(&mut errors, tokens);
}

#[test]
fn main_test() {
    use std::rc::Rc;
    let _ = env_logger::try_init();

    fn eval_with_debugs(source: &str) -> Vec<Rc<lisp_value::LispValue>> {
        println!("SOURCE {:?}", source);
        // PARSE
        let result = eval::parse(source);
        println!("AST {:?}", result);
        assert!(result.is_ok());

        // Eval
        let global_env = Rc::new(env::Env::new_global());
        let result = eval::eval_program(&result.unwrap(), global_env.clone());
        println!("RESULT {:?}", result);
        println!("GLOBAL ENV {:#?}", global_env);

        result
    }

    let sources = vec![
        "(+ 1 2) (+ 3 2)",
        "(= true true)",
        "(define (myFn x) (+ x 2))",
        "(define (myFn x) (+ x 2)) (myFn 4)",
        "(define (my-fn-2? x) (+ x 2))",
        "(define myVar (+ 3 2))",
        "(if (= true true) (+ 1 2) (+ 3 4))",
    ];

    for source in &sources {
        eval_with_debugs(source);
    }
}
