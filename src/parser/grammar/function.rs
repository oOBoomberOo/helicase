use super::prelude::*;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Function<'a> {
	name: &'a str,
	parameter: Vec<ParseResult<'a>>
}

impl<'a> Grammar for Function<'a> {
	fn build<'b>(buffer: &[Token<'b>]) -> ParseResult<'b> {
		let mut stream = buffer.iter();
		stream.next();
		let name: Token = *stream.next().unwrap();
		let parameter: Vec<Token> = stream.cloned().collect();
		let start = name.span();
		let end = parameter.last().map(|x| x.span()).unwrap();
		let span = start.join(end);

		let parser = Parser::new(parameter);
		let parameter = parser.parse();

		let name = name.slice();
		let value = Function { name, parameter };
		Ok(Syntax::Function { span, value })
	}
}