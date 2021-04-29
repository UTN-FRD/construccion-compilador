#[macro_use]
extern crate log;
extern crate env_logger;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
); // synthesized by LALRPOP

use std::rc::Rc;

use frd_lisp::ast;
use frd_lisp::env;
use frd_lisp::eval;
use frd_lisp::intrinsics;
use frd_lisp::lisp_value;
use frd_lisp::token;

use std::io;
use std::io::prelude::*;

fn input() -> String {
    print!("frd_lisp$");
    io::stdout().flush().unwrap();

    let mut reply = String::new();
    io::stdin().read_line(&mut reply).unwrap();
    reply
}

pub fn main() {
    println!("FRD LISP: REPL (interactive) MODE \n\n");
    let global_env = Rc::new(env::Env::new_global());

    //TODO use a real REPL crate for this
    loop {
        let line = input();
        println!(">>> {:?}", repl_eval(&line, global_env.clone())[0]);
    }
}

fn repl_eval(source: &str, env: Rc<env::Env>) -> Vec<Rc<lisp_value::LispValue>> {
    let result = eval::parse(source);
    assert!(result.is_ok(), "Syntax error {:?}", result);

    eval::eval_program(&result.unwrap(), env)
}

#[test]
fn main_test() {}