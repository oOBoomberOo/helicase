use super::prelude::*;

pub struct CommandLexer;

impl Lexer for CommandLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		let mut buffer = String::default();
		let start = get_pos![stream];
		let mut end = start;
		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if !token.is_alphabetic() {
				break;
			}

			buffer.push(token);
			end = index;
			stream.next();
		}
		let span = start..end;
		let value = buffer;
		let result = Token::Ident { span: span.into(), value };

		Ok(result)
	}
}