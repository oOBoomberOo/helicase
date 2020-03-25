use std::ops::Range;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct Span<T: Display + Debug> {
	pub id: usize,
	pub content: T,
	pub range: Range<usize>
}

impl<T: Display + Debug> Span<T> {
	pub fn new(id: usize, content: T, range: Range<usize>) -> Span<T> {
		Span { id, content, range }
	}
}

use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
impl<T: Display + Debug> fmt::Display for Span<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.content)
	}
}