use super::prelude::*;

#[derive(Debug, Clone)]
pub struct EndOfFile<'a> {
	content: &'a str,
}

impl<'uwu> Grammar for EndOfFile<'uwu> {
	fn build<'a>(buffer: &[Token<'a>]) -> ParseResult<'a> {
		let content = buffer.first().map(|x| x.value()).unwrap();
		let start = buffer.first().map(|x| x.span()).unwrap();
		let end = buffer.last().map(|x| x.span()).unwrap();

		let range = start.join(end).range();
		let content = &content[range];
		let span = start.join(end);
		let value = EndOfFile { content };
		Ok(Syntax::EndOfFile { span, value })
	}
}