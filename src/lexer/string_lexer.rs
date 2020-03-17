use super::prelude::*;

pub struct DoubleStringLexer;

impl Lexer for DoubleStringLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		let mut buffer = String::default();

		let start = get_pos![stream];
		let mut end = start;
		let mut escaping = false;
		
		// Push the first quote to buffer before anything
		match stream.next() {
			Some((_, token)) => buffer.push(token),
			None => unreachable!(),
		}

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if token == '\\' {
				escaping = true;
				end = index;
				buffer.push(token);
				continue;
			}

			buffer.push(token);
			end = index;
			stream.next();

			if !escaping && token == '"' {
				break;
			}

			escaping = false;
		}

		if !buffer.ends_with('"') {
			return Err(LexError::IncompleteString { span: (start..end).into(), value: buffer})
		}

		let result = Token::String { span: (start..end).into(), value: buffer };
		Ok(result)
	}
}
pub struct SingleStringLexer;

impl Lexer for SingleStringLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		let mut buffer = String::default();
		let start = get_pos![stream];
		let mut end = start;
		let mut escaping = false;

		while let Some(&item) = stream.peek() {
			let (index, token) = item;

			if token == '\\' {
				escaping = true;
				end = index;
				buffer.push(token);
				continue;
			}

			buffer.push(token);
			end = index;
			stream.next();

			if !escaping && token == '\'' {
				break;
			}

			escaping = false;
		}

		if !buffer.ends_with('\'') {
			return Err(LexError::IncompleteString { span: (start..end).into(), value: buffer})
		}

		let result = Token::String { span: (start..end).into(), value: buffer };
		Ok(result)
	}
}