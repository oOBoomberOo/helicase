mod token;
mod span;
mod nbt;

use span::Span;
use token::Token;
use nbt::Nbt;
use super::prelude::*;

use std::iter::Peekable;
use std::str::CharIndices;
type TokenStream<'a> = Peekable<CharIndices<'a>>;

#[derive(Debug)]
pub struct FunctionProcessor<'a> {
	content: &'a str,
	resource: &'a Resource
}

impl<'a> FunctionProcessor<'a> {
	fn new(content: &'a str, resource: &'a Resource) -> FunctionProcessor<'a> {
		FunctionProcessor {
			content,
			resource
		}
	}

	fn parse(&self, _context: &mut Context) -> Vec<FunctionError> {
		let tokens = self.tokenizer();
		tokens.iter().for_each(|token| {
			println!("{}({:#?})", token.kind(), token);
		});
		Vec::new()
	}

	fn tokenizer(&self) -> Vec<Token> {
		let stream = &mut self.content.char_indices().peekable();
		Token::parse(stream, self.content)
	}
}

impl<'a> Processor for FunctionProcessor<'a> {
	fn process(resource: &Resource, context: &mut Context) -> PResult<Vec<PError>> {
		let content = resource.read()?;
		let function = FunctionProcessor::new(&content, resource);

		let result = function
			.parse(context)
			.into_iter()
			.map(PError::from)
			.collect();
		Ok(result)
	}
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum FunctionError {

}

impl ProcessorError for FunctionError {
	fn report(&self, _message: &ErrorTemplates) -> Diagnostic<usize> {
		todo!()
	}
}

trait ExtraChar {
	fn is_number(&self) -> bool;
	fn is_vector(&self) -> bool;
	fn is_selector(&self) -> bool;
	fn is_nbt(&self) -> bool;
}

impl ExtraChar for char {
	fn is_number(&self) -> bool {
		match self {
			'0'..='9' | '.' | '-' => true,
			_ => false
		}
	}

	fn is_vector(&self) -> bool {
		match self {
			'^' | '~' => true,
			_ => self.is_number(),
		}
	}

	fn is_selector(&self) -> bool {
		*self == '@'
	}

	fn is_nbt(&self) -> bool {
		*self == '[' || *self == '{'
	}
}