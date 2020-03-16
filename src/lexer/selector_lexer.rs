use super::prelude::*;

pub struct SelectorLexer;

impl Lexer for SelectorLexer {
	fn lex(stream: &mut TokenStream, _context: &mut Context) -> LexResult<Token> {
		let start = get_pos![stream];

		if let Some(item) = stream.next() {
			let (_, token) = item;
			
		}
		else {

		}

		let mut end = start;

		todo!()
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