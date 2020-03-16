// Lexers
mod command_lexer;
mod string_lexer;
mod vector_lexer;
mod whitespace_lexer;
mod selector_lexer;

use command_lexer::CommandLexer;
use vector_lexer::VectorLexer;
use whitespace_lexer::WhitespaceLexer;
use selector_lexer::SelectorLexer;

// Utils
mod error;
mod span;
mod token;
mod utils;

pub use error::LexError;
pub use span::Span;
pub use token::Token;

use std::iter::Peekable;
use std::str::CharIndices;

pub type TokenStream<'a> = Peekable<CharIndices<'a>>;
pub type LexResult<T> = Result<T, LexError>;

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
	let mut context = Context::default();
	while let Some(item) = stream.peek() {
		let (index, token) = item;

		if token.is_linebreak() {
			let span = index.into();
			let value = token.to_owned();
			let value = Token::Linebreak { span, value };
			result.push(Ok(value));
			stream.next();
		} else if token.is_selector() {
			result.push(SelectorLexer::lex(stream, &mut context));
		}
		else if token.is_alphabetic() {
			result.push(CommandLexer::lex(stream, &mut context));
		} else if token.is_whitespace() {
			result.push(WhitespaceLexer::lex(stream, &mut context));
		} else if token.is_vector() {
			result.push(VectorLexer::lex(stream, &mut context));
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

	/// Check if token is an entity selector
	/// 
	/// わははは-！
	fn is_selector(&self) -> bool;
}

impl ExtendedChar for char {
	fn is_vector(&self) -> bool {
		self.is_numeric() || *self == '~' || *self == '^'
	}

	fn is_linebreak(&self) -> bool {
		*self == '\n'
	}

	fn is_selector(&self) -> bool {
		*self == '@'
	}
}

mod prelude {
	pub use super::{Context, LexError, LexResult, Span, Token, TokenStream};
	pub use super::{ExtendedChar, Lexer};
	pub use crate::get_pos;
}
