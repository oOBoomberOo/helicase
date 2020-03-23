use crate::lexer::prelude::Span;
use super::grammar::prelude::*;

#[derive(Debug, Clone)]
pub enum Syntax<'a> {
	Function { span: Span, value: Function<'a> },
	Expression { span: Span, value: Expression<'a> },
	EndOfFile { span: Span, value: EndOfFile<'a> },
	Identifier { span: Span, value: &'a str },
	Literal { span: Span, value: &'a str },
}