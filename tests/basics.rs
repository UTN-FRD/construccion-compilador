extern crate env_logger;
extern crate log;
use frd_lisp::eval::{eval, parse};
use std::fs::File;
use std::io::Read;

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("Can't open the file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read the file.");
    contents
}

#[test]
fn fib_test() {
    let _ = env_logger::try_init();
    let source = read_file("./tests/fib.flp");
    let ast = parse(&source).unwrap();
    let result = eval(ast);
    println!("{:?}", result);

    assert!(result.is_ok());
    // Compare strings as a way of avoiding re creating the expected array
    let string_result = format!("{:?}", result.unwrap());
    assert_eq!(string_result, "[Nil, 1, 1, 2, 3, 5, 8, 13]");
}

#[test]
fn id_test() {
    let _ = env_logger::try_init();
    let source = read_file("./tests/id.flp");
    let ast = parse(&source).unwrap();
    let result = eval(ast);
    println!("{:?}", result);

    assert!(result.is_ok());
    let string_result = format!("{:?}", result.unwrap());
    assert_eq!(string_result, "[Nil, 1, 2, 3]");
}

#[test]
fn variadic_functions_test() {
    let _ = env_logger::try_init();
    let source = read_file("./tests/variadic_functions.flp");
    let ast = parse(&source).unwrap();
    let result = eval(ast);
    println!("{:?}", result);

    assert!(result.is_ok());
    let string_result = format!("{:?}", result.unwrap());
    assert_eq!(string_result, "[15]");
}

#[test]
fn higher_order_functions_test() {
    let _ = env_logger::try_init();
    let source = read_file("./tests/hofs.flp");
    let ast = parse(&source).unwrap();
    let result = eval(ast);
    println!("{:?}", result);

    assert!(result.is_ok());
    let string_result = format!("{:?}", result.unwrap());
    assert_eq!(string_result, "[Nil, Nil, 2, 3, 4]");
}
