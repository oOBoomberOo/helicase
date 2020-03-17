use super::prelude::*;

pub type LexResult<T> = Result<T, LexError>;
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
	IncompleteString { span: Span, value: String}
}