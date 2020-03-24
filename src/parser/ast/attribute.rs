use super::*;

#[derive(Debug)]
pub struct Attribute {
	values: Vec<Identifier>
}

impl Attribute {
	pub fn new(values: Vec<Identifier>) -> Self {
		Attribute { values }
	}
}