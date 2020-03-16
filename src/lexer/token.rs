use super::prelude::*;
use super::selector_lexer::{SelectorKind, Predicate};

#[derive(Debug)]
pub enum Token {
	End,
	Ident { span: Span, value: String },
	Bracket { span: Span, value: String },
	Number { span: Span, value: String },
	Whitespace { span: Span, value: String },
	Linebreak { span: Span, value: char },
	Selector { span: Span, kind: SelectorKind, predicate: Predicate }
}