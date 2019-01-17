#[macro_use]
extern crate log;
extern crate env_logger;

extern crate frd_lisp;

use frd_lisp::eval_file;

#[test]
fn basics() {
    let _ = env_logger::try_init();
    let res = eval_file("./tests/fib.flp");
    println!("{:?}", res);

    assert_eq!(4, 4);
}
