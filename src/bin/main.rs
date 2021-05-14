use std::rc::Rc;

use frd_lisp::env;
use frd_lisp::eval;

use std::io;
use std::io::prelude::*;

fn input() -> io::Result<String> {
    print!("frd_lisp$");
    io::stdout().flush()?;

    let mut reply = String::new();
    io::stdin().read_line(&mut reply)?;
    Ok(reply)
}

pub fn main() -> io::Result<()> {
    println!("FRD LISP: REPL (interactive) MODE \n\n");
    let global_env = Rc::new(env::Env::new_global());

    //TODO use a real REPL crate for this
    loop {
        let line = input()?;
        match eval::parse(&line) {
            Ok(ast) => match eval::eval_program(&ast, global_env.clone()) {
                Ok(values) => println!("{}", values[0]),
                Err(eval_error) => println!("ERROR: {}", eval_error),
            },
            Err(error) => println!("ERROR: {:?}", error),
        }
    }
}
