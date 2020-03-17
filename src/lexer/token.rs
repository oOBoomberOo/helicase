use super::prelude::*;

#[derive(Debug)]
pub enum Token {
	End,
	Ident { span: Span, value: String },
	Number { span: Span, value: String },
	Whitespace { span: Span, value: String },
	Linebreak { span: Span, value: char },
	Symbol { span: Span, value: String }
}