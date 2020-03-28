use crate::get_pos;
use super::{ExtraChar, Nbt, Span, TokenStream};
use itertools::Itertools;
pub enum Token<'a> {
	Ident(Span<'a>),
	Number(Span<'a>),
	Selector(Span<'a>),
	Other(Span<'a>),
	Whitespace(Span<'a>),
	Nbt(Span<'a>, Nbt<'a>),
}

use std::fmt;
impl<'a> Token<'a> {
	pub fn span(&self) -> &Span {
		match self {
			Token::Ident(span)
			| Token::Number(span)
			| Token::Selector(span)
			| Token::Other(span)
			| Token::Whitespace(span)
			| Token::Nbt(span, _) => span,
		}
	}

	pub fn kind(&self) -> TokenKind {
		match self {
			Token::Ident(_) => TokenKind::Ident,
			Token::Number(_) => TokenKind::Number,
			Token::Selector(_) => TokenKind::Selector,
			Token::Other(_) => TokenKind::Other,
			Token::Whitespace(_) => TokenKind::Whitespace,
			Token::Nbt(_, _) => TokenKind::Nbt,
		}
	}
}

impl<'a> fmt::Debug for Token<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Token::Nbt(_, nbt) => nbt.fmt(f),
			_ => self.span().fmt(f)
		}
	}
}


impl<'a> Token<'a> {
	pub fn parse(stream: &mut TokenStream, source: &'a str) -> Vec<Self> {
		let mut result = Vec::new();

		while let Some(token) = stream.peek() {
			let (_, token) = token;
			let value = {
				if token.is_selector() {
					Token::parse_selector(stream, source)
				} else if token.is_whitespace() {
					Token::parse_whitespace(stream, source)
				} else if token.is_nbt() {
					Token::parse_nbt(stream, source)
				} else if token.is_alphabetic() {
					Token::parse_identifier(stream, source)
				} else if token.is_vector() {
					Token::parse_number(stream, source)
				} else {
					Token::parse_other(stream, source)
				}
			};
			result.push(value);
		}

		result
	}

	fn parse_nbt(stream: &mut TokenStream, source: &'a str) -> Self {
		let nbt = Nbt::parse(stream, source);
		// TODO: shorten this to `nbt.span().clone()` if possible
		let span = Span::new(source, nbt.span().get_range());
		Token::Nbt(span, nbt)
	}

	fn parse_whitespace(stream: &mut TokenStream, source: &'a str) -> Self {
		let pos = get_pos!(stream);
		stream.next();
		let span = Span::from_pos(source, pos);
		Token::Whitespace(span)
	}

	fn parse_selector(stream: &mut TokenStream, source: &'a str) -> Self {
		let start = get_pos!(stream);
		stream.next();

		if let Some(token) = stream.next() {
			let (end, _) = token;
			let span = Span::new(source, start..=end);
			Token::Selector(span)
		} else {
			let span = Span::new(source, start..=start);
			Token::Other(span)
		}
	}

	fn parse_other(stream: &mut TokenStream, source: &'a str) -> Self {
		let start = get_pos!(stream);
		let end = start;
		stream.next();
		let span = Span::new(source, start..=end);
		Token::Other(span)
	}

	fn parse_identifier(stream: &mut TokenStream, source: &'a str) -> Self {
		let start = get_pos!(stream);
		let end = stream
			.peeking_take_while(|(_, token)| token.is_alphabetic())
			.last()
			.map(|(index, _)| index)
			.unwrap_or(start);

		let span = Span::new(source, start..=end);
		Token::Ident(span)
	}

	fn parse_number(stream: &mut TokenStream, source: &'a str) -> Self {
		let start = get_pos!(stream);
		let end = stream
			.peeking_take_while(|(_, token)| token.is_vector())
			.last()
			.map(|(index, _)| index)
			.unwrap_or(start);

		let span = Span::new(source, start..=end);
		Token::Number(span)
	}
}

#[macro_export]
macro_rules! get_pos {
	($x:expr) => {
		match $x.peek() {
			Some(token) => token.0,
			None => 0,
		}
	};
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum TokenKind {
	#[error("Identifier")]
	Ident,
	#[error("Number")]
	Number,
	#[error("Selector")]
	Selector,
	#[error("Other")]
	Other,
	#[error("Whitespace")]
	Whitespace,
	#[error("NBT")]
	Nbt,
}
