// Lexers
mod command_lexer;
mod string_lexer;
mod symbol_lexer;
mod vector_lexer;
mod whitespace_lexer;

use command_lexer::CommandLexer;
use string_lexer::{DoubleStringLexer, SingleStringLexer};
use symbol_lexer::SymbolLexer;
use vector_lexer::VectorLexer;
use whitespace_lexer::WhitespaceLexer;

// Utils
mod error;
mod span;
mod token;
mod utils;

use std::iter::Peekable;
use std::str::CharIndices;

use prelude::*;

pub type TokenStream<'a> = Peekable<CharIndices<'a>>;

/// This trait handle lexing behavior of all lexer
///
/// May return LexError if there are lexing error like 'String not end with quote'
///
/// ![](https://pbs.twimg.com/media/ERLhwpGXYAANEQP.jpg)
/// 「エクｽプロションー！」
pub trait Lexer {
	fn lex(stream: &mut TokenStream, context: &mut Context) -> LexResult<Token>;
}

#[derive(Debug, Default)]
pub struct Context {}

pub fn lex(stream: &mut TokenStream) -> Vec<LexResult<Token>> {
	let mut result = Vec::default();
	let context = &mut Context::default();
	while let Some(item) = stream.peek() {
		let (index, token) = item;

		if token.is_linebreak() {
			let span = index.into();
			let value = token.to_string();
			let value = Token::Linebreak { span, value };
			result.push(Ok(value));
			stream.next();
		} else if *token == '"' {
			result.push(DoubleStringLexer::lex(stream, context));
		} else if *token == '\'' {
			result.push(SingleStringLexer::lex(stream, context));
		} else if token.is_symbol() {
			result.push(SymbolLexer::lex(stream, context));
		} else if token.is_alphabetic() {
			result.push(CommandLexer::lex(stream, context));
		} else if token.is_whitespace() {
			result.push(WhitespaceLexer::lex(stream, context));
		} else if token.is_vector() {
			result.push(VectorLexer::lex(stream, context));
		} else {
			stream.next();
		}
	}
	result
}

/// Extending trait for 'char' type
pub trait ExtendedChar {
	/// Similar to `is_numberic()` but including `~` and `^`
	///
	/// ナイス！
	fn is_vector(&self) -> bool;

	fn is_linebreak(&self) -> bool;

	fn is_symbol(&self) -> bool;
}

impl ExtendedChar for char {
	fn is_vector(&self) -> bool {
		self.is_numeric() || *self == '~' || *self == '^'
	}

	fn is_linebreak(&self) -> bool {
		*self == '\n'
	}

	fn is_symbol(&self) -> bool {
		*self == '['
			|| *self == ']'
			|| *self == '{'
			|| *self == '}'
			|| *self == ':'
			|| *self == '='
			|| *self == '!'
			|| *self == '#'
			|| *self == '@'
	}
}

pub mod prelude {
	pub use super::{Context, TokenStream};
	pub use super::error::{LexResult, LexError};
	pub use super::span::Span;
	pub use super::token::{Token, TokenKind};
	pub use super::{ExtendedChar, Lexer};
	pub use crate::get_pos;
}
