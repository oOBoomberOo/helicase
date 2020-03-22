
use super::prelude::*;
pub struct NumberLexer;

impl<'a> Lex<'a> for NumberLexer {
	fn lex(stream: &mut TokenStream, content: &'a str) -> Token<'a> {
		let start = *get_pos![stream.peek()];
		let mut end = start;

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if !token.is_numeric() {
				break;
			}

			end = index + 1;
			stream.next();
		}

		Token::from_span(TokenKind::Number, content, (start..end).into())
	}
}