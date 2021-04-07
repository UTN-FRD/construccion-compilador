// This module contains test cases to ensure the parser's output is correct.
// Since the interpreter is meant to expose every part of its process (lexing,
// parsing, etc.), parsing is part of its "external" interface and so the
// tests shouldn't live within the source file.

use frd_lisp::ast as ast;
use frd_lisp::grammar as grammar;

#[test]
fn addition_test() {
    use ast::Expr as Expr;
    use ast::Atom as Atom;

    // TODO: Move this to a `setup` function that prepares the `parser`
    // object?
    //
    // TODO: The parser should take a `Lexer` argument if we're going
    // to use our own lexer.
    //
    // NOTE: Both TODOs apply for the following tests.
    let parser = grammar::ProgramParser::new();

    let expected = vec!(
        // TODO: Should it use a (yet to be created) `Atom::Op`?
        Expr::Atom(Atom::Id("+".to_string())),
        Expr::Atom(Atom::Number(1.0)),
        Expr::Atom(Atom::Number(2.0))
    );

    let result = parser.parse("+ 1 2").unwrap();

    assert_eq!(&result, &expected);
}

#[test]
fn list_addition_test() {
    use ast::Expr as Expr;
    use ast::Atom as Atom;

    let parser = grammar::ProgramParser::new();

    let expected = vec!(
        Expr::List(
            vec!(
                Expr::Atom(Atom::Id("+".to_string())),
                Expr::Atom(Atom::Number(1.0)),
                Expr::Atom(Atom::Number(2.0))
            )
        )
    );

    let result = parser.parse("(+ 1 2)").unwrap();

    assert_eq!(&result, &expected);
}
