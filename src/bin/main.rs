use std::rc::Rc;

use frd_lisp::env;
use frd_lisp::eval;
use frd_lisp::lisp_value;

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
        let result = eval::eval(&line, Some(global_env.clone()));

        // Display the evaluator's output.
        println!(">>> {:?}", result.first());
    }
}
