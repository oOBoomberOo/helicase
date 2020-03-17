use std::ops::Range;
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
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
		write!(f, "{:?}..{:?}", self.start, self.end)
	}
}
impl fmt::Display for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}..{}", self.start, self.end)
	}
}

use std::ops::{Add, Sub, AddAssign, SubAssign};
impl Add<Span> for Span {
	type Output = Span;
	fn add(self, rhs: Span) -> Span {
		Span {
			start: self.start + rhs.start,
			end: self.end + rhs.end
		}
	}
}
impl AddAssign<Span> for Span {
	fn add_assign(&mut self, rhs: Span) {
		self.start += rhs.start;
		self.end += rhs.end;
	}
}
impl Add<usize> for Span {
	type Output = Span;
	fn add(self, rhs: usize) -> Span {
		Span {
			start: self.start,
			end: self.end + rhs
		}
	}
}
impl AddAssign<usize> for Span {
	fn add_assign(&mut self, rhs: usize) {
		self.end += rhs;
	}
}

impl Sub<Span> for Span {
	type Output = Span;
	fn sub(self, rhs: Span) -> Span {
		Span {
			start: self.start - rhs.start,
			end: self.end - rhs.end
		}
	}
}
impl SubAssign<Span> for Span {
	fn sub_assign(&mut self, rhs: Span) {
		self.start -= rhs.start;
		self.end -= rhs.end;
	}
}
impl Sub<usize> for Span {
	type Output = Span;
	fn sub(self, rhs: usize) -> Span {
		Span {
			start: self.start,
			end: self.end - rhs
		}
	}
}
impl SubAssign<usize> for Span {
	fn sub_assign(&mut self, rhs: usize) {
		self.end -= rhs;
	}
}

impl From<Span> for Range<usize> {
	fn from(span: Span) -> Self {
		span.start..span.end
	}
}

impl From<&Span> for Range<usize> {
	fn from(span: &Span) -> Self {
		span.start..span.end
	}
}