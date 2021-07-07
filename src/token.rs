use std::str::{CharIndices, FromStr};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Token {
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
    Identifier(String),
    String(String),
}

// This function takes a string as parameter and returns a vector of triples
// with the following structure: (start_position, token, end_position)
pub fn tokenize(s: &str) -> Vec<(usize, Token, usize)> {
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
                    tokens.push(Token::String(String::from(&s[ci..slice_end])));
                    continue;
                }
                _ if c.is_digit(10) => {
                    let mut has_dot = false;
                    let (slice_end, next) = take_while(ci, &mut char_indices, |c| {
                        if c.is_digit(10) {
                            true
                        } else if c == '.' {
                            if !has_dot {
                                has_dot = true;
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    });
                    lookahead = next;
                    // TODO: return an error
                    let parsed_float = f64::from_str(&s[ci..slice_end]);
                    let token = match parsed_float {
                        Ok(num) => Token::Num(num),
                        Err(e) => panic!("{}", e),
                    };
                    tokens.push(token);
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
        ident => Token::Identifier(String::from(ident)),
    }
}

fn take_while<F>(
    slice_start: usize,
    char_indices: &mut CharIndices,
    mut f: F,
) -> (usize, Option<(usize, char)>)
where
    F: FnMut(char) -> bool,
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
