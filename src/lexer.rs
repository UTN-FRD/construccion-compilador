use logos::Logos;
use std::str::FromStr;
#[derive(Debug, Copy, Clone, PartialEq, Logos)]
enum Token<'input> {
	#[regex("(\\-)?[0-9]+(\\.[0-9]+)?", |t| f64::from_str(t.slice()))]
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
	#[regex("[A-Za-z][A-Za-z0-9]+")]
    Identifier(&'input str),
	#[regex("\"[a-zA-Z]+\"", |t| {let s = t.slice(); &s[1..(s.len() - 1)]})]
    String(&'input str),
	#[error]
	Error,
}



#[test]
fn can_parse_strings() {
	let mut lex = Token::lexer("\"hello\"");

	assert_eq!(lex.next(), Some(Token::String("hello")));
    assert_eq!(lex.slice(), "\"hello\"");
}
