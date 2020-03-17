use super::command_parser::CommandError;
use super::prelude::*;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
	CommandError(CommandError)
}

impl ParserError for ParseError {
	fn diagnos(&self, file_id: usize) -> Diagnostic<usize> {
		match self {
			ParseError::CommandError(error) => error.diagnos(file_id)
		}
	}
}

macro_rules! parse_error {
	($x:ident) => {
		impl From<$x> for ParseError {
			fn from(err: $x) -> Self {
				ParseError::$x(err)
			}
		}
	}
}

parse_error!(CommandError);