use std::rc::Rc;

use frd_lisp::env::{Env};
use frd_lisp::eval;

use std::io;
use std::io::prelude::*;

/// Imprime el prompt y lee una nueva linea.
fn input() -> io::Result<String> {
    print!("frd_lisp$");
    io::stdout().flush()?;

    let mut reply = String::new();
    io::stdin().read_line(&mut reply)?;
    Ok(reply)
}

/// Programa principal del interprete por consola.
/// Consiste de un loop infinito que espera una entrada por parte
/// del usuario para luego evaluarla e imprimir su resultado.
pub fn main() -> io::Result<()> {
    println!("FRD LISP: REPL (interactive) MODE \n\n");
    // se crea un ambiente global, con las operaciones basicas. 
    let global_env = Rc::new(Env::new_global());

    loop {
        // se lee una linea.
        let line = input()?;
        // se evalua la linea y se imprime el resultado.
        match eval::eval(&line, Some(global_env.clone())) {
            Ok(value) => println!(">>> {:?}", value.first().unwrap()),
            Err(lisp_error) => println!("Error: {:?}", lisp_error),
        }
    }
}
