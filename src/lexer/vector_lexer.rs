use super::prelude::*;

pub struct VectorLexer;

impl Lexer for VectorLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		let mut buffer = String::default();
		let start = get_pos![stream];
		let mut end = start;

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if !token.is_vector() {
				break;
			}

			end = index;
			buffer.push(token);
			stream.next();
		}
		
		let result = Token::Number { span: (start..end).into(), value: buffer };
		Ok(result)
	}
}