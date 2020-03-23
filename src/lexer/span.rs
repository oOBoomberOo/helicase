#[derive(Clone, Copy)]
pub struct Span {
	pub start: usize,
	pub end: usize
}

use std::fmt;
impl fmt::Debug for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}

impl fmt::Display for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

use std::ops::Range;
impl From<Range<&usize>> for Span {
	fn from(range: Range<&usize>) -> Span {
		Span {
			start: *range.start,
			end: *range.end
		}
	}
}
impl From<Range<usize>> for Span {
	fn from(range: Range<usize>) -> Span {
		Span {
			start: range.start,
			end: range.end
		}
	}
}

use std::cmp::{min, max};
impl Span {
	pub fn range(&self) -> Range<usize> {
		self.start..self.end
	}

	pub fn join(&self, other: &Span) -> Span {
		Span {
			start: min(self.start, other.start),
			end: max(self.end, other.end)
		}
	}
}