pub struct Span {
	start: usize,
	end: usize
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

impl Span {
	pub fn range(&self) -> Range<usize> {
		self.start..self.end
	}
}