use crate::lexer;
use crate::Expr;
use crate::grammar;

pub type ParserError<'a> = lalrpop_util::ParseError<(), lexer::Token<'a>, &'static str>;

pub fn parse(tokens: Vec<lexer::Token>) -> Result<Vec<Expr>, ParserError> {
    let mut errors = Vec::new();
    let parser = grammar::ProgramParser::new();

    parser.parse(&mut errors, tokens)
}
