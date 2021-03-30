use std::str::{CharIndices, FromStr};

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'input> {
    Num(f64),
    LParen,
    RParen,
    Minus,
    Plus,
    Times,
    Div,
    Equal,
    GreaterThan,
    LessThan,
    Comma,
    Define,
    If,
    Identifier(&'input str),
    String(&'input str),
}

// This function takes a string as parameter and returns a vector of triples
// with the following structure: (start_position, token, end_position)
pub fn tokenize<'input>(s: &'input str) -> Vec<(usize, Token<'input>, usize)> {
    let mut tokens = Vec::new();
    let mut char_indices = s.char_indices();
    let mut lookahead = char_indices.next();
    while let Some((ci, c)) = lookahead {
        // skip whitespace characters
        if !c.is_whitespace() {
            match c {
                '(' => tokens.push(Token::LParen),
                '-' => tokens.push(Token::Minus),
                ')' => tokens.push(Token::RParen),
                '=' => tokens.push(Token::Equal),
                '<' => tokens.push(Token::LessThan),
                '>' => tokens.push(Token::GreaterThan),
                '+' => tokens.push(Token::Plus),
                '*' => tokens.push(Token::Times),
                ',' => tokens.push(Token::Comma),
                '/' => tokens.push(Token::Div),
                '"' => {
                    let (ci, _) = char_indices.next().expect("Unclosed '\"'"); // consume opening '"'
                    let (slice_end, _) = take_while(ci, &mut char_indices, |c| c != '"');
                    lookahead = char_indices.next(); // consume closing '"'
                    tokens.push(Token::String(&s[ci..slice_end]));
                    continue;
                }
                _ if c.is_digit(10) => {
                    let (slice_end, next) = take_while(ci, &mut char_indices, |c| c.is_digit(10));
                    lookahead = next;
                    tokens.push(Token::Num(f64::from_str(&s[ci..slice_end]).unwrap()));
                    continue;
                }
                _ if c.is_alphanumeric() => {
                    let (slice_end, new_lookahead) = take_while(ci, &mut char_indices, |c| {
                        !c.is_whitespace() && c != '(' && c != ')'
                    });
                    lookahead = new_lookahead;
                    tokens.push(parse_identifier(&s[ci..slice_end]));
                    continue;
                }
                _ => {
                    panic!("invalid character: {:?}", c);
                }
            }
        }

        // advance to next character by default
        lookahead = char_indices.next();
    }

    tokens
        .into_iter()
        .enumerate()
        .map(|(i, tok)| (i * 2, tok, i * 2 + 1))
        .collect()
}

fn parse_identifier(s: &str) -> Token {
    match s {
        "define" => Token::Define,
        "if" => Token::If,
        ident => Token::Identifier(ident),
    }
}

fn take_while<F>(
    slice_start: usize,
    char_indices: &mut CharIndices,
    f: F,
) -> (usize, Option<(usize, char)>)
where
    F: Fn(char) -> bool,
{
    let mut slice_end = slice_start + 1;

    for (ci, c) in char_indices {
        if f(c) {
            slice_end = ci + 1;
        } else {
            return (ci, Some((ci, c)));
        }
    }

    (slice_end, None)
}

#[test]
fn tok1_test() {
    let source_code = "("; // should return (0, LParen, 1)
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::LParen, 1)])
}

#[test]
fn tok2_test() {
    let source_code = "()";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::LParen, 1), (2, Token::RParen, 3)])
}

#[test]
fn tok3_test() {
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

#[test]
fn tok4_test() {
    let source_code = "\"string\"";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::String("string"), 1)])
}

#[test]
fn tok5_test() {
    let source_code = "\"another string\"";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::String("another string"), 1)])
}

#[test]
fn tok6_test() {
    let source_code = "(define (id x) (x))";
    let tokens = tokenize(source_code);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (2, Token::Define, 3),
            (4, Token::LParen, 5),
            (6, Token::Identifier("id"), 7),
            (8, Token::Identifier("x"), 9),
            (10, Token::RParen, 11),
            (12, Token::LParen, 13),
            (14, Token::Identifier("x"), 15),
            (16, Token::RParen, 17),
            (18, Token::RParen, 19)
        ]
    )
}

// TODO: this should break
#[test]
fn tok7_test() {
    let source_code = "\"hello";
    let tokens = tokenize(source_code);
    assert_eq!(tokens, [(0, Token::String("hello"), 1)])
}

#[test]
fn tok8_test() {
    let source_code = "(define (k x) 12345)";
    let tokens = tokenize(source_code);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (2, Token::Define, 3),
            (4, Token::LParen, 5),
            (6, Token::Identifier("k"), 7),
            (8, Token::Identifier("x"), 9),
            (10, Token::RParen, 11),
            (12, Token::Num(12345.0), 13),
            (14, Token::RParen, 15)
        ]
    )
}

#[test]
fn tok9_test() {
    let source = "(= 3 x)";
    let tokens = tokenize(source);
    assert_eq!(
        tokens,
        [
            (0, Token::LParen, 1),
            (2, Token::Equal, 3),
            (4, Token::Num(3.0), 5),
            (6, Token::Identifier("x"), 7),
            (8, Token::RParen, 9)
        ]
    );
}
