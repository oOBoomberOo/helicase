use super::prelude::*;
use crate::parser::Parser;

// TODO: Support multiple scope
#[derive(Debug, Clone)]
pub enum Expression<'a> {
	Block { inner: Vec<ParseResult<'a>> },
	Condition { inner: Vec<ParseResult<'a>> }
}

impl<'uwu> Grammar for Expression<'uwu> {
	fn build<'a>(buffer: &[Token<'a>]) -> ParseResult<'a> {
		match buffer {
			[start @ Token::Symbol { slice: "(", .. }, Token::Identifier { slice, .. }, end @ Token::Symbol { slice: ")", .. }] => {
				let span = start.span().join(end.span());
				Ok(Syntax::Identifier { span, value: slice })
			},
			[start @ Token::Symbol { slice: "{", .. }, rest @ .., end @ Token::Symbol { slice: "}", .. }] => {
				let token = Vec::from(rest);
				let parser = Parser::new(token);
				let inner = parser.parse();
				let span = start.span().join(end.span());
				let value = Expression::Block { inner };
				Ok(Syntax::Expression { span, value })
			},
			[start @ Token::Symbol { slice: "(", .. }, rest @ .., end @ Token::Symbol { slice: ")", .. }] => {
				let token = Vec::from(rest);
				let parser = Parser::new(token);
				let inner = parser.parse();
				let span = start.span().join(end.span());
				let value = Expression::Condition { inner };
				Ok(Syntax::Expression { span, value })
			},
			[Token::String { slice, span, .. }] => Ok(Syntax::Literal { span: *span, value: slice }),
			[Token::Number { slice, span, .. }] => Ok(Syntax::Literal { span: *span, value: slice }),
			_ => todo!()
		}
	}
}