use super::prelude::*;

pub struct StringLexer;

impl<'a> Lex<'a> for StringLexer {
	fn lex(stream: &mut TokenStream, content: &'a str) -> Token<'a> {
		let start = *get_pos![stream.peek()];
		let mut end = start;

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if token == '\\' {
				if let Some(item) = stream.next() {
					let (index, _) = item;
					end = index + 1;
					continue;
				}
			}
			else if start != end && token == '"' {
				end = index + 1;
				stream.next();
				break;
			}

			end = index + 1;
			stream.next();
		}

		Token::from_span(TokenKind::String, content, (start..end).into())
	}
}