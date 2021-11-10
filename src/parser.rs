use crate::Expr;
use crate::lexer;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    grammar
);

pub type ParserError<'a> = lalrpop_util::ParseError<(), lexer::Token<'a>, &'static str>;

pub fn parse<'a>(tokens: Vec<lexer::Token<'a>>) -> Result<Vec<Expr>, ParserError<'a>> {
    let mut errors = Vec::new();
    let parser = grammar::ProgramParser::new();

    parser.parse(&mut errors, tokens)
}
