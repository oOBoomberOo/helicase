use crate::parser::prelude::*;

#[derive(Debug, Default)]
pub struct MeCommand {
	content: String
}

impl Parser for MeCommand {
	fn parse(self, stream: &mut ParseStream, _context: &mut Context) -> ParseResult<Box<dyn Parser>> {
		let content: String = stream.take_while(|x| !x.is_linebreak()).map(|x| x.to_string()).collect();
		let result = MeCommand { content };
		Ok(Box::new(result))
	}
}