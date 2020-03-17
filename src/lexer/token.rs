use super::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
	End,
	Ident { span: Span, value: String },
	Number { span: Span, value: String },
	Whitespace { span: Span, value: String },
	Linebreak { span: Span, value: String },
	Symbol { span: Span, value: String },
	String { span: Span, value: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	End,
	Ident,
	Number,
	Whitespace,
	Linebreak,
	Symbol,
	String
}

impl Token {
	pub fn span(&self) -> Option<&Span> {
		match self {
			Token::End => None,
			Token::Ident { span, .. }
			| Token::Number { span, .. }
			| Token::Whitespace { span, .. }
			| Token::Linebreak { span, .. }
			| Token::Symbol { span, .. }
			| Token::String { span, .. } => Some(span),
		}
	}

	pub fn value(&self) -> Option<&str> {
		match self {
			Token::End => None,
			Token::Ident { value, .. }
			| Token::Number { value, .. }
			| Token::Whitespace { value, .. }
			| Token::Symbol { value, .. }
			| Token::String { value, .. }
			| Token::Linebreak { value, .. }=> Some(value),
		}
	}

	pub fn kind(&self) -> TokenKind {
		match self {
			Token::End => TokenKind::End,
			Token::Ident { .. } => TokenKind::Ident,
			Token::Number { .. } => TokenKind::Number,
			Token::Whitespace { .. } => TokenKind::Whitespace,
			Token::Linebreak { .. } => TokenKind::Linebreak,
			Token::Symbol { .. } => TokenKind::Symbol,
			Token::String { .. } => TokenKind::String,
		}
	}

	pub fn is_linebreak(&self) -> bool {
		match self {
			Token::Linebreak {..} | Token::End => true,
			_ => false
		}
	}
}

use std::fmt;
impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Token::End => write!(f, ""),
			other => write!(f, "{}", other.value().unwrap())
		}
	}
}