use crate::lexer::prelude::{Token, TokenKind};

mod command_parser;
use command_parser::CommandParser;

mod error;
use error::ParseResult;
use prelude::*;

#[derive(Debug, Default)]
pub struct Context {}

/// Lifetime is getting confusing...
pub trait Parser {
	fn parse(self, stream: &mut ParseStream, context: &mut Context)
		-> ParseResult<Box<dyn Parser>>;
}

pub trait ParserError {
	fn diagnos(&self, file: usize) -> Diagnostic<usize>;
}

pub fn parse(tokens: &[Token]) -> Vec<ParseResult<()>> {
	let stream = &mut tokens.iter().peekable();
	let context = &mut Context::default();
	let mut result = Vec::default();
	while let Some(token) = stream.peek() {
		if token.kind() == TokenKind::Ident {
			let parse_result = CommandParser.parse(stream, context).map(|_| ());
			result.push(parse_result);
		}
		else {
			stream.next();
		}
	}

	result
}

pub mod prelude {
	use super::Token;
	use std::iter::Peekable;
	use std::slice::Iter;
	pub type ParseStream<'megumin> = Peekable<Iter<'megumin, Token>>;
	pub use super::error::{ParseError, ParseResult};
	pub use super::{Context, Parser, ParserError};

	pub use codespan::FileId;
	pub use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle};
}
