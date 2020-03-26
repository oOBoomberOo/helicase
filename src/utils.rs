use std::ops::Range;

pub fn get_json_field(content: &str, value: &str) -> Range<usize> {
	let pattern = format!(r#""{}""#, value);
	let start = content.find(&pattern).unwrap_or(0);
	let end = start + pattern.len();
	start..end
}
