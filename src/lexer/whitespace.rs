
use super::prelude::*;
pub struct WhitespaceLexer;

impl<'a> Lex<'a> for WhitespaceLexer {
	fn lex(stream: &mut TokenStream, content: &'a str) -> Token<'a> {
		let start = *get_pos![stream.peek()];
		let mut end = start;

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if !token.is_whitespace() {
				break;
			}

			end = index;
			stream.next();
		}

		Token::from_span(TokenKind::Whitespace, content, (start..end).into())
	}
}