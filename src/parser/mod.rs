use crate::lexer::prelude::{Token, TokenKind};

mod command_parser;

use command_parser::CommandParser;

mod error;
use error::ParseResult;

use prelude::*;

#[derive(Debug, Default)]
pub struct Context {}

pub trait Parser {
	fn parse(stream: &mut ParseStream, context: &mut Context) -> ParseResult;
}

pub fn parse(tokens: &[Token]) -> Vec<ParseResult> {
	let stream = &mut tokens.iter().peekable();
	let context = &mut Context::default();
	let mut result = Vec::default();
	while let Some(token) = stream.peek(){
		if token.kind() == TokenKind::Ident {
			result.push(CommandParser::parse(stream, context));
		}
	}

	result
}

pub mod prelude {
	use super::Token;
	use std::iter::Peekable;
	use std::slice::Iter;
	pub type ParseStream<'megumin> = Peekable<Iter<'megumin, Token>>;
	pub use super::{Parser, Context};
	pub use super::error::{ParseResult, ParseError};
}