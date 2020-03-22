use super::Span;

pub enum Token<'a> {
	Identifier { value: &'a str, span: Span },
	Whitespace { value: &'a str, span: Span },
	Number { value: &'a str, span: Span },
	String { value: &'a str, span: Span },
	Symbol { value: &'a str, span: Span },
}

impl<'a> Token<'a> {
	pub fn new(kind: TokenKind, value: &'a str, span: Span) -> Token<'a> {
		match kind {
			TokenKind::Identifier => Token::Identifier { value, span },
			TokenKind::Whitespace => Token::Whitespace { value, span },
			TokenKind::Number => Token::Number { value, span },
			TokenKind::String => Token::String { value, span },
			TokenKind::Symbol => Token::Symbol { value, span },
		}
	}

	pub fn symbol(index: usize, content: &'a str) -> Token<'a> {
		Token::Symbol { span: Span::from(index..index+1), value: &content[index..index+1] }
	}

	pub fn from_span(kind: TokenKind, content: &'a str, span: Span) -> Token<'a> {
		match kind {
			TokenKind::Identifier => Token::Identifier { value: &content[span.range()], span },
			TokenKind::Whitespace => Token::Whitespace { value: &content[span.range()], span },
			TokenKind::Number => Token::Number { value: &content[span.range()], span },
			TokenKind::String => Token::String { value: &content[span.range()], span },
			TokenKind::Symbol => Token::Symbol { value: &content[span.range()], span }
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

#[derive(Debug)]
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
			Token::Identifier { span, value }
			| Token::Whitespace { span, value }
			| Token::Number { span, value }
			| Token::String { span, value }
			| Token::Symbol { span, value } => write!(f, "{} {:?} ({})", self.kind(), value, span),
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