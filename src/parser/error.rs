use super::command_parser::CommandError;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
	CommandError(CommandError)
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