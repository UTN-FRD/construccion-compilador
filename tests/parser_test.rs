use frd_lisp::ast::{Atom, Expr};
use frd_lisp::eval;

#[test]
fn parse_eq_operation() {
    let source = "(= 3 x)";
    let result = eval::parse(source);
    assert!(result.is_ok());
    let expected = Expr::List(vec![
        Expr::Atom(Atom::Id("=".to_string())),
        Expr::Atom(Atom::Number(3.0)),
        Expr::Atom(Atom::Id("x".to_string())),
    ]);
    assert_eq!(result.unwrap()[0], expected);
}

#[test]
fn parse_subtract_operation() {
    let source = "(- x 1)";
    let result = eval::parse(source);
    assert!(result.is_ok());
    let expected = Expr::List(vec![
        Expr::Atom(Atom::Id("-".to_string())),
        Expr::Atom(Atom::Id("x".to_string())),
        Expr::Atom(Atom::Number(1.0)),
    ]);
    assert_eq!(result.unwrap()[0], expected);
}

#[test]
fn parse_add_operation() {
    let source = "(+ 3 4 2 34.3)";
    let result = eval::parse(source);
    assert!(result.is_ok());
    let expected = Expr::List(vec![
        Expr::Atom(Atom::Id("+".to_string())),
        Expr::Atom(Atom::Number(3.0)),
        Expr::Atom(Atom::Number(4.0)),
        Expr::Atom(Atom::Number(2.0)),
        Expr::Atom(Atom::Number(34.3)),
    ]);
    assert_eq!(result.unwrap()[0], expected);
}

#[test]
fn parse_id_function() {
    let source = "(define (id x) x)";
    let result = eval::parse(source);
    assert!(result.is_ok());
    let expected = Expr::DefineFunction(
        "id".to_string(),
        vec!["x".to_string()],
        vec![Expr::Atom(Atom::Id("x".to_string()))],
    );
    assert_eq!(result.unwrap()[0], expected);
}
