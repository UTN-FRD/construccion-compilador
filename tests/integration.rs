// TODO: These tests use `eval::parse` which did both tokenizing and parsing.

use frd_lisp::ast::Atom;
use frd_lisp::{lexer, parse, Expr};

#[test]
fn parse_eq_operation() {
    let source = "(= 3 x)";
    let lexer = lexer::Token::tokenize(source);
    let tokens = lexer.collect();
    let ast = parse(tokens);

    assert!(ast.is_ok());

    let expected = Expr::List(vec![
        Expr::Atom(Atom::Id("=".to_string())),
        Expr::Atom(Atom::Number(3.0)),
        Expr::Atom(Atom::Id("x".to_string())),
    ]);

    assert_eq!(ast.unwrap()[0], expected);
}

#[test]
fn parse_subtract_operation() {
    let source = "(- x 1)";
    let lexer = lexer::Token::tokenize(source);
    let tokens = lexer.collect();
    let ast = parse(tokens);

    assert!(ast.is_ok());

    let expected = Expr::List(vec![
        Expr::Atom(Atom::Id("-".to_string())),
        Expr::Atom(Atom::Id("x".to_string())),
        Expr::Atom(Atom::Number(1.0)),
    ]);

    assert_eq!(ast.unwrap()[0], expected);
}

#[test]
fn parse_add_operation() {
    let source = "(+ 3 4 2 34.3)";
    let lexer = lexer::Token::tokenize(source);
    let tokens = lexer.collect();
    let ast = parse(tokens);

    assert!(ast.is_ok());

    let expected = Expr::List(vec![
        Expr::Atom(Atom::Id("+".to_string())),
        Expr::Atom(Atom::Number(3.0)),
        Expr::Atom(Atom::Number(4.0)),
        Expr::Atom(Atom::Number(2.0)),
        Expr::Atom(Atom::Number(34.3)),
    ]);

    assert_eq!(ast.unwrap()[0], expected);
}

#[test]
fn parse_id_function() {
    let source = "(define (id x) x)";
    let lexer = lexer::Token::tokenize(source);
    let tokens = lexer.collect();
    let ast = parse(tokens);

    assert!(ast.is_ok());

    let expected = Expr::DefineFunction(
        "id".to_string(),
        vec!["x".to_string()],
        vec![Expr::Atom(Atom::Id("x".to_string()))],
    );

    assert_eq!(ast.unwrap()[0], expected);
}
