pub mod ast;

use lalrpop_util::lalrpop_mod;
use lalrpop_util::{ParseError, lexer::Token};
lalrpop_mod!(pub main, "/parser/main.rs");

pub fn parse(content: &str) -> Result<Vec<ast::Statement>, ParseError<usize, Token, &'static str>> {
	let parser = main::statementsParser::new();
	parser.parse(content)
}