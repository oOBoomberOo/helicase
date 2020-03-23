use crate::lexer::prelude::*;

mod grammar;
// mod context;
mod syntax;
mod error;

// use context::Context;
use error::Error;
use grammar::prelude::*;

pub struct Parser<'a> {
	token: Vec<Token<'a>>
}

impl<'a> Parser<'a> {
	pub fn new(token: Vec<Token<'a>>) -> Parser<'a> {
		Parser { token }
	}

	pub fn parse(&self) -> Vec<ParseResult<'a>> {
		// let context = &Context::default();
		let stream = &mut self.token.iter().peekable();
		let mut result = Vec::default();

		let mut buffer = Vec::default();
		loop {
			let token = stream.peek();
			match self.compute_state(&buffer, token) {
				State::Shift => {
					let token: Token = *stream.next().unwrap_or_else(|| unreachable!());
					buffer.push(token);
				},
				State::Reduce(message) => {
					let syntax = match message {
						Message::Identifier => Identifier::build(&buffer),
						Message::Function => Function::build(&buffer),
						Message::Expression => Expression::build(&buffer),
						Message::EndOfFile => {
							if !buffer.is_empty() {
								result.push(EndOfFile::build(&buffer));
							}
							break
						}
					};
					result.push(syntax);
					buffer.clear();
				}
			}
		}

		result
	}

	fn compute_state(&self, buffer: &[Token], peek: Option<&&Token>) -> State {
		match buffer {
			[Token::Identifier { slice: "fn", .. }, Token::Identifier { .. }, Token::Symbol { slice: "(", .. }, .., Token::Symbol { slice: ")", ..}] => return State::Reduce(Message::Function),
			[Token::Symbol { slice: "{", .. }, .., Token::Symbol { slice: "}", .. }] => return State::Reduce(Message::Expression),
			[Token::Symbol { slice: "(", .. }, .., Token::Symbol { slice: ")", .. }] => return State::Reduce(Message::Expression),
			[Token::Identifier { .. }, .., Token::Symbol { slice: ";", .. }] => return State::Reduce(Message::Identifier),
			_ => ()
		};

		if peek.is_none() {
			return State::Reduce(Message::EndOfFile);
		}

		State::Shift
	}
}

pub enum State {
	Shift,
	Reduce(Message)
}

#[derive(Debug, Clone)]
pub enum Message {
	Function,
	Identifier,
	Expression,
	EndOfFile
}

impl Default for State {
	fn default() -> State {
		State::Shift
	}
}

pub mod prelude {
	pub use super::{Parser, State, Message};
}