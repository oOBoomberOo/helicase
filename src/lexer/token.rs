use super::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
	End,
	Ident { span: Span, value: String },
	Number { span: Span, value: String },
	Whitespace { span: Span, value: String },
	Linebreak { span: Span, value: char },
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

	pub fn value(&self) -> Option<String> {
		match self {
			Token::End => None,
			Token::Ident { value, .. }
			| Token::Number { value, .. }
			| Token::Whitespace { value, .. }
			| Token::Symbol { value, .. }
			| Token::String { value, .. } => Some(value.to_owned()),
			Token::Linebreak { value, .. } => Some(value.to_string())
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
}
