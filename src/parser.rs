use crate::Expr;
use crate::Token;
use lalrpop_util::lalrpop_mod;

// This macro synthetises the `grammar` module which will be used to parse files.
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    grammar
);

pub type ParserError = lalrpop_util::ParseError<(), Token, &'static str>;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, ParserError> {
    let mut errors = Vec::new();
    let parser = grammar::ProgramParser::new();

    parser.parse(&mut errors, tokens)
}
