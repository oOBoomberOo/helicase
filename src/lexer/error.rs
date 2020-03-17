use super::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
	IncompleteString { span: Span, value: String}
}