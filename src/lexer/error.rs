use super::prelude::*;

pub type LexResult<T> = Result<T, LexError>;
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
	IncompleteString { span: Span, value: String}
}

use std::fmt;
impl fmt::Display for LexError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			LexError::IncompleteString { span, value } => write!(f, "Incomplete String {} at {}", value, span)
		}
	}
}