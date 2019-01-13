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

#[test]
fn test() {
    let _ = env_logger::try_init();

    fn test_case(source: &str) -> Vec<Rc<lisp_value::LispValue>> {
        println!("SOURCE {:?}", source);
        // PARSE
        let parser = grammar::ProgramParser::new();
        let result = parser.parse(source);
        println!("AST {:?}", result);
        assert!(result.is_ok());

        // Eval
        let global_env = Rc::new(env::Env::new_global());
        let result = eval::eval_program(&result.unwrap(), global_env.clone());
        println!("RESULT {:?}", result);
        println!("GLOBAL ENV {:#?}", global_env);

        return result;
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
        test_case(source);
    }
}
