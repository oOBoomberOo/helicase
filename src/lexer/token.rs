use super::Span;

#[derive(Clone, Copy)]
pub enum Token<'a> {
	Identifier { value: &'a str, span: Span, slice: &'a str },
	Whitespace { value: &'a str, span: Span, slice: &'a str },
	Number { value: &'a str, span: Span, slice: &'a str },
	String { value: &'a str, span: Span, slice: &'a str},
	Symbol { value: &'a str, span: Span, slice: &'a str},
}

impl<'a> Token<'a> {
	pub fn new(kind: TokenKind, value: &'a str, span: Span) -> Token<'a> {
		let slice = &value[span.range()];
		match kind {
			TokenKind::Identifier => Token::Identifier { value, span, slice },
			TokenKind::Whitespace => Token::Whitespace { value, span, slice },
			TokenKind::Number => Token::Number { value, span, slice },
			TokenKind::String => Token::String { value, span, slice },
			TokenKind::Symbol => Token::Symbol { value, span, slice },
		}
	}

	pub fn symbol(index: usize, value: &'a str) -> Token<'a> {
		let span = Span::from(index..index+1);
		let slice = &value[span.range()];
		Token::Symbol { span, value, slice }
	}

	pub fn from_span(kind: TokenKind, value: &'a str, span: Span) -> Token<'a> {
		let slice = &value[span.range()];
		match kind {
			TokenKind::Identifier => Token::Identifier { value, span, slice},
			TokenKind::Whitespace => Token::Whitespace { value, span, slice},
			TokenKind::Number => Token::Number { value, span, slice},
			TokenKind::String => Token::String { value, span, slice},
			TokenKind::Symbol => Token::Symbol { value, span, slice}
		}
	}

	pub fn value(&self) -> &'a str {
		match self {
			Token::Identifier { value, .. }
			| Token::Whitespace { value, .. }
			| Token::Number { value, .. }
			| Token::String { value, .. }
			| Token::Symbol { value, .. } => value,
		}
	}

	pub fn slice(&self) -> &'a str {
		match self {
			Token::Identifier { slice, ..}
			| Token::Whitespace { slice, ..}
			| Token::Number { slice, .. }
			| Token::String { slice, .. }
			| Token::Symbol { slice, .. } => slice,
		}
	}

	pub fn span(&self) -> &Span {
		match self {
			Token::Identifier { span, .. }
			| Token::Whitespace { span, .. }
			| Token::Number { span, .. }
			| Token::String { span, .. }
			| Token::Symbol { span, .. } => span,
		}
	}

	pub fn kind(&self) -> TokenKind {
		match self {
			Token::Identifier { .. } => TokenKind::Identifier,
			Token::Whitespace { .. } => TokenKind::Whitespace,
			Token::Number { .. } => TokenKind::Number,
			Token::String { .. } => TokenKind::String,
			Token::Symbol { .. } => TokenKind::Symbol,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	Identifier,
	Whitespace,
	Number,
	String,
	Symbol,
}

use std::fmt;
impl fmt::Debug for Token<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Token::Identifier { span, .. }
			| Token::Whitespace { span, .. }
			| Token::Number { span, .. }
			| Token::String { span, .. }
			| Token::Symbol { span, .. } => write!(f, "{} {:?} ({})", self.kind(), self.slice(), span),
		}
	}
}

impl fmt::Display for TokenKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			TokenKind::Identifier => write!(f, "Identifier"),
			TokenKind::Whitespace => write!(f, "Whitespace"),
			TokenKind::Number => write!(f, "Number"),
			TokenKind::String => write!(f, "String"),
			TokenKind::Symbol => write!(f, "Symbol"),
		}
	}
}