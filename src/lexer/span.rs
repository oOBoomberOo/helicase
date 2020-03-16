use std::ops::Range;
pub struct Span {
	start: usize,
	end: usize,
}

impl From<Range<usize>> for Span {
	fn from(range: Range<usize>) -> Span {
		Span {
			start: range.start,
			end: range.end,
		}
	}
}

impl From<Range<&usize>> for Span {
	fn from(range: Range<&usize>) -> Span {
		Span {
			start: *range.start,
			end: *range.end,
		}
	}
}

impl From<usize> for Span {
	fn from(value: usize) -> Self {
		Span { start: value, end: value }
	}
}

impl From<&usize> for Span {
	fn from(value: &usize) -> Self {
		Span { start: *value, end: *value }
	}
}

use std::fmt;
impl fmt::Debug for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}