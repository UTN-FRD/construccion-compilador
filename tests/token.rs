use frd_lisp::{tokenize, Token};

#[test]
fn left_parenthesis_test() {
    let source_code = "("; // should return (0, LParen, 1)
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::LParen, 1)])
}

#[test]
fn both_parenthesis_test() {
    let source_code = "()";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::LParen, 1), (2, Token::RParen, 3)])
}

#[test]
fn parenthesis_and_numbers_test() {
    let source_code = "(1 2 3)";
    let tokens = tokenize(source_code);
    let expected = [
        (0, Token::LParen, 1),
        (2, Token::Num(1.0), 3),
        (4, Token::Num(2.0), 5),
        (6, Token::Num(3.0), 7),
        (8, Token::RParen, 9),
    ];
    assert_eq!(tokens, expected)
}

// TODO: Maybe compress functions `completed_string_test` and
// `another_completed_string_test` into one? `FromIterable` isn't
// implemented for `Vec<usize, Token, usize>` currently.
// Same goes for `function_definition_test` and
// `another_function_definition_test`.
#[test]
fn completed_string_test() {
    let source_code = "\"string\"";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::String("string".into()), 1)])
}

#[test]
fn another_completed_string_test() {
    let source_code = "\"another string\"";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::String("another string".into()), 1)])
}

// TODO: This should break.
// TODO: If the " is on the right, the assertion fails... only because the
// tokenized input becomes "hello\"" (without the outer quotation marks).
#[test]
fn incomplete_string_test() {
    let source_code = "\"hello";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::String("hello".into()), 1)])
}

#[test]
fn function_definition_test() {
    let source_code = "(define (id x) (x))";
    let tokens = tokenize(source_code);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (2, Token::Define, 3),
            (4, Token::LParen, 5),
            (6, Token::Identifier("id".into()), 7),
            (8, Token::Identifier("x".into()), 9),
            (10, Token::RParen, 11),
            (12, Token::LParen, 13),
            (14, Token::Identifier("x".into()), 15),
            (16, Token::RParen, 17),
            (18, Token::RParen, 19)
        ]
    )
}

#[test]
fn another_function_definition_test() {
    let source_code = "(define (k x) 12345)";
    let tokens = tokenize(source_code);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (2, Token::Define, 3),
            (4, Token::LParen, 5),
            (6, Token::Identifier("k".into()), 7),
            (8, Token::Identifier("x".into()), 9),
            (10, Token::RParen, 11),
            (12, Token::Num(12345.0), 13),
            (14, Token::RParen, 15)
        ]
    )
}

#[test]
fn equality_test() {
    let source = "(= 3 x)";
    let tokens = tokenize(source);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (2, Token::Equal, 3),
            (4, Token::Num(3.0), 5),
            (6, Token::Identifier("x".into()), 7),
            (8, Token::RParen, 9)
        ]
    );
}

#[test]
fn number_test() {
    let source_code = "10.5";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::Num(10.5), 1)])
}
