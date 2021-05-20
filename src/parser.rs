use lalrpop_util::lalrpop_mod;
use crate::Token;
use crate::Expr;

// This macro synthetises the `grammar` module which will be used to parse files.
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    grammar
);

pub type ParserError<'a> = lalrpop_util::ParseError<(), Token<'a>, &'static str>;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, ParserError> {
    let mut errors = Vec::new();
    let parser = grammar::ProgramParser::new();

    return parser.parse(&mut errors, tokens);
}
