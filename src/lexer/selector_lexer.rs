use super::prelude::*;

pub struct SelectorLexer;

impl Lexer for SelectorLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		let pos = get_pos![stream];
		let span = Span::from(pos);

		let kind = match stream.next() {
			None => return Err(LexError::IncompleteSelector { span }),
			Some((_, token)) => SelectorKind::new(&token.to_string())
		};

		let result = Token::Selector { span, kind };
		Ok(result)
	}
}

#[derive(Debug)]
pub enum SelectorKind {
	Player,
	Everyone,
	Random,
	Entities,
	Raw(String)
}

impl SelectorKind {
	fn new(token: &str) -> SelectorKind {
		match token {
			"p" => SelectorKind::Player,
			"a" => SelectorKind::Everyone,
			"r" => SelectorKind::Random,
			"e" => SelectorKind::Entities,
			raw => SelectorKind::Raw(raw.to_owned())
		}
	}
}

#[derive(Debug)]
pub enum Predicate {

}