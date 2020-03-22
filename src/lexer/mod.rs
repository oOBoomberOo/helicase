mod token;
mod span;

mod identifier;
mod whitespace;
mod number;
mod string;

use identifier::IdentLexer;
use whitespace::WhitespaceLexer;
use number::NumberLexer;
use string::StringLexer;

use prelude::*;

#[derive(Debug, Default)]
pub struct Lexer {
	content: String
}

impl Lexer {
	pub fn new(content: String) -> Lexer {
		Lexer { content }
	}

	pub fn lex(&self) -> Vec<Token> {
		let content = &self.content;
		let stream = &mut self.content.char_indices().peekable();
		let mut result = Vec::default();

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if token.is_alphabetic() {
				result.push(IdentLexer::lex(stream, content));
			}
			else if token.is_numeric() {
				result.push(NumberLexer::lex(stream, content));
			}
			else if token.is_whitespace() {
				result.push(WhitespaceLexer::lex(stream, content));
			}
			else if token == '"' {
				result.push(StringLexer::lex(stream, content));
			}
			else {
				result.push(Token::symbol(index, content));
				stream.next();
			}
		}

		result
	}
}

pub trait Lex<'a> {
	fn lex(stream: &mut TokenStream, content: &'a str) -> Token<'a>;
}

#[macro_export]
macro_rules! get_pos {
	($x:expr) => {
		match $x {
			Some((index, _)) => index,
			None => unreachable!(),
		}
	}
}

#[doc(hidden)]
pub mod prelude {
	pub use super::token::{Token, TokenKind};
	pub use super::span::Span;
	pub use super::{Lexer, Lex};

	use std::iter::Peekable;
	use std::str::CharIndices;
	pub type TokenStream<'a> = Peekable<CharIndices<'a>>;
	#[macro_use]
	pub use crate::get_pos;
}