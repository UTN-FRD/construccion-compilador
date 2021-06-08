// This module contains test cases to ensure the parser's output is correct.
// Since the interpreter is meant to expose every part of its process (lexing,
// parsing, etc.), parsing is part of its "external" interface and so the
// tests shouldn't live within the source file.

use frd_lisp::ast;
use frd_lisp::parse;
use frd_lisp::Token;

#[test]
fn addition_test() {
    use ast::Atom;
    use ast::Expr;

    let expected = vec![
        // TODO: Should it use a (yet to be created) `Atom::Op`?
        Expr::Atom(Atom::Id("+".to_string())),
        Expr::Atom(Atom::Number(1.0)),
        Expr::Atom(Atom::Number(2.0)),
    ];

    let tokens = vec![Token::Plus, Token::Num(1.0), Token::Num(2.0)];

    let result = parse(tokens).unwrap();

    assert_eq!(&result, &expected);
}

#[test]
fn list_addition_test() {
    use ast::Atom;
    use ast::Expr;

    let expected = vec![Expr::List(vec![
        Expr::Atom(Atom::Id("+".to_string())),
        Expr::Atom(Atom::Number(1.0)),
        Expr::Atom(Atom::Number(2.0)),
    ])];

    let tokens = vec![
        Token::LParen,
        Token::Plus,
        Token::Num(1.0),
        Token::Num(2.0),
        Token::RParen,
    ];

    let result = parse(tokens).unwrap();

    assert_eq!(&result, &expected);
}
