use frd_lisp::lexer;
use frd_lisp::lexer::Token;
use logos::Logos;

#[test]
fn left_parenthesis_test() {
    let source_code = "("; // should return (0, LParen, 1)
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(tokens, [(0, Token::LParen, 1)])
}

#[test]
fn both_parenthesis_test() {
    let source_code = "()";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(tokens, [(0, Token::LParen, 1), (1, Token::RParen, 2)])
}

#[test]
fn parenthesis_and_numbers_test() {
    let source_code = "(1 2 3)";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    let expected = [
        (0, Token::LParen, 1),
        (1, Token::Num(1.0), 2),
        (3, Token::Num(2.0), 4),
        (5, Token::Num(3.0), 6),
        (6, Token::RParen, 7),
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
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(tokens, [(0, Token::String("string"), 8)])
}

#[test]
fn another_completed_string_test() {
    let source_code = "\"another string\"";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(tokens, [(0, Token::String("another string"), 16)])
}

// TODO: We should add a message to the error?
#[test]
fn incomplete_string_test() {
    let source_code = "\"hello";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(tokens, [(0, Token::Error, 6)])
}

#[test]
fn function_definition_test() {
    let source_code = "(define (id x) (x))";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (1, Token::Define, 7),
            (8, Token::LParen, 9),
            (9, Token::Identifier("id"), 11),
            (12, Token::Identifier("x"), 13),
            (13, Token::RParen, 14),
            (15, Token::LParen, 16),
            (16, Token::Identifier("x"), 17),
            (17, Token::RParen, 18),
            (18, Token::RParen, 19)
        ]
    )
}

#[test]
fn another_function_definition_test() {
    let source_code = "(define (k x) 12345)";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (1, Token::Define, 7),
            (8, Token::LParen, 9),
            (9, Token::Identifier("k"), 10),
            (11, Token::Identifier("x"), 12),
            (12, Token::RParen, 13),
            (14, Token::Num(12345.0), 19),
            (19, Token::RParen, 20)
        ]
    )
}

#[test]
fn equality_test() {
    let source_code = "(= 3 x)";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (1, Token::Equal, 2),
            (3, Token::Num(3.0), 4),
            (5, Token::Identifier("x"), 6),
            (6, Token::RParen, 7)
        ]
    );
}

#[test]
fn number_test() {
    let source_code = "10.5";
    let lexer = lexer::Token::lexer(source_code);
    let tokens = lexer::get_token_info(lexer);
    assert_eq!(tokens, [(0, Token::Num(10.5), 4)])
}
