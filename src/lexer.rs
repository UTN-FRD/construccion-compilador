use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
enum Token<'input> {
    #[regex("-?[0-9]+\\.[0-9]+", |t| t.slice().parse())]
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
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

#[derive(Debug, PartialEq, Eq)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenInfo<'input> {
    token: Token<'input>,
    pos: Span,
    text: String,
}

impl TokenInfo<'_> {
    fn new<'a>(token: Token<'a>, pos: Span, text: String) -> TokenInfo<'a> {
        TokenInfo { token, pos, text }
    }
}

// TODO: Hacer una funcion que en vez de producir Tokens, produzca
// un vector de TokenInfo
pub fn tokenize<'input>(source: &'input str) -> Vec<TokenInfo<'input>> {
    let mut tokens: Vec<TokenInfo<'input>> = vec![];
    let mut lex = Token::lexer(source);

    loop {
        if let Some(t) = lex.next() {
            let span = Span::new(lex.span().start, lex.span().end);
            let ti = TokenInfo::new(t, span, lex.slice().to_string());
            tokens.push(ti);
        } else {
            break;
        }
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

#[test]
fn can_tokenize() {
    let expected = TokenInfo::new(Token::LParen, Span::new(0, 1), "(".to_string());
    assert_eq!(vec![expected], tokenize("("));
}
