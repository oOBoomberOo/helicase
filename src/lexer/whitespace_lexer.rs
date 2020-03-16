use super::prelude::*;

pub struct WhitespaceLexer;

impl Lexer for WhitespaceLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		let mut buffer = String::default();
		let start = get_pos![stream];
		let mut end = start;

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if !token.is_whitespace() {
				break;
			}

			end = index;
			buffer.push(token);
			stream.next();
		}

		let span = (start..end).into();
		let value = buffer;
		let result = Token::Whitespace { span, value };

		Ok(result)
	}
}