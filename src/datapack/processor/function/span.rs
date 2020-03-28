use std::ops::RangeInclusive;

#[derive(Clone, PartialEq, Eq)]
pub struct Span<'a> {
	pub source: &'a str,
	pub range: RangeInclusive<usize>,
}

impl<'a> Span<'a> {
	pub fn new(source: &'a str, range: impl Into<RangeInclusive<usize>>) -> Span<'a> {
		let range = range.into();
		Span { source, range }
	}

	pub fn from_pos(source: &'a str, pos: usize) -> Span<'a> {
		Span::new(source, pos..=pos)
	}

	pub fn get_range(&self) -> RangeInclusive<usize> {
		self.range.clone()
	}

	pub fn content(&self) -> &str {
		&self.source[self.get_range()]
	}
}

use std::fmt;
impl<'a> fmt::Display for Span<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}..{}", self.range.start(), self.range.end())
	}
}

impl<'a> fmt::Debug for Span<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", &self.source[self.get_range()])
	}
}

impl<'a> From<(usize, &'a str)> for Span<'a> {
	fn from((pos, source): (usize, &'a str)) -> Self {
		Span::new(source, pos..=pos)
	}
}