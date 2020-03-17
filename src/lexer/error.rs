use super::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
	IncompleteSelector { span: Span }
}