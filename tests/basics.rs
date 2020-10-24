extern crate env_logger;
extern crate log;
use frd_lisp::eval_file;

#[test]
fn fib_test() {
    let _ = env_logger::try_init();
    let result = eval_file("./tests/fib.flp");
    println!("{:?}", result);

    // Compare strings as a way of avoiding re creating the expected array
    let string_result = format!("{:?}", result);
    assert_eq!(string_result, "[Nil, 1, 1, 2, 3, 5, 8, 13]");
}

#[test]
fn id_test() {
    let _ = env_logger::try_init();
    let result = eval_file("./tests/id.flp");
    println!("{:?}", result);

    let string_result = format!("{:?}", result);
    assert_eq!(string_result, "[Nil, 1, 2, 3]");
}

#[test]
fn variadic_functions_test() {
    let _ = env_logger::try_init();
    let result = eval_file("./tests/variadic_functions.flp");
    println!("{:?}", result);

    let string_result = format!("{:?}", result);
    assert_eq!(string_result, "[15]");
}

#[test]
fn higher_order_functions_test() {
    let _ = env_logger::try_init();
    let result = eval_file("./tests/hofs.flp");
    println!("{:?}", result);

    let string_result = format!("{:?}", result);
    assert_eq!(string_result, "[Nil, Nil, 2, 3, 4]");
}
