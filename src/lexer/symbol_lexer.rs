use super::prelude::*;

pub struct SymbolLexer;

impl Lexer for SymbolLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		if let Some(symbol) = stream.peek() {
			let (pos, token) = symbol;

			let result = Token::Symbol { span: pos.into(), value: token.to_string() };
			Ok(result)
		}
		else {
			unreachable!()
		}
	}
}