use crate::lexer::prelude::Token;
#[derive(Debug, Clone)]
pub enum Error<'a> {
	Unknown(Token<'a>),
}