
use super::prelude::*;
pub struct IdentLexer;

impl<'a> Lex<'a> for IdentLexer {
	fn lex(stream: &mut TokenStream, content: &'a str) -> Token<'a> {
		let start = *get_pos![stream.peek()];
		let mut end = start;

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if !token.is_alphabetic() {
				break;
			}

			end = index + 1;
			stream.next();
		}

		Token::from_span(TokenKind::Identifier, content, (start..end).into())
	}
}