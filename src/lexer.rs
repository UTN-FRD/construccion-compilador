use logos::{Lexer, Logos};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Logos, Serialize, Deserialize)]
pub enum Token<'input> {
    #[regex("-?[0-9]+(\\.[0-9])*", |t| t.slice().parse())]
    Num(f64),
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("*")]
    Times,
    #[token("/")]
    Div,
    #[token("=")]
    Equal,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token(",")]
    Comma,
    #[token("define")]
    Define,
    #[token("if")]
    If,
    // #[regex("[a-zA-Z$_][a-zA-Z0-9$_]*")]
    #[regex("[a-zA-Z][a-zA-Z0-9:&-]*")]
    Identifier(&'input str),
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |t| {let s = t.slice(); &s[1..(s.len() - 1)]})]
    String(&'input str),
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

pub fn get_token_info<'a>(mut lexer: Lexer<'a, Token<'a>>) -> Vec<(usize, Token<'a>, usize)> {
    let mut tokens = vec![];

    while let Some(token) = lexer.next() {
        let span = lexer.span();
        tokens.push((span.start, token, span.end));
    }

    tokens
}

#[test]
fn can_parse_strings() {
    let mut lex = Token::lexer("\"hello\"");

    assert_eq!(lex.next(), Some(Token::String("hello")));
    assert_eq!(lex.slice(), "\"hello\"");
}

#[test]
fn can_parse_float_numbers() {
    let mut lex = Token::lexer("3.4");

    assert_eq!(lex.next(), Some(Token::Num(3.4)));
    assert_eq!(lex.slice(), "3.4");
}

#[test]
fn can_parse_numbers() {
    let mut lex = Token::lexer("3");

    assert_eq!(lex.next(), Some(Token::Num(3.0)));
    assert_eq!(lex.slice(), "3");
}

#[test]
fn can_parse_a_list() {
    let lex = Token::lexer("(+ 1 2)");

    let tokens: Vec<Token> = lex.collect();

    assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::Plus,
            Token::Num(1.0),
            Token::Num(2.0),
            Token::RParen
        ]
    );
}
