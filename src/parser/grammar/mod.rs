mod function;
mod expression;
mod end_of_file;
mod identifier;

use prelude::*;

pub trait Grammar {
	fn build<'a>(buffer: &[Token<'a>]) -> ParseResult<'a>;
}

pub mod prelude {
	// use std::iter::Peekable;
	// use std::slice::Iter;
	pub use crate::lexer::prelude::{Token, Span};
	// pub type TokenStream<'a> = Peekable<Iter<'a, Token<'a>>>;

	pub use super::function::Function;
	pub use super::expression::Expression;
	pub use super::end_of_file::EndOfFile;
	pub use super::identifier::Identifier;

	pub use super::Grammar;
	// pub use crate::parser::context::Context;
	pub use crate::parser::syntax::Syntax;
	pub use crate::parser::error::Error;
	pub use crate::parser::{State, Message};

	pub type ParseResult<'a> = Result<Syntax<'a>, Error<'a>>;
}