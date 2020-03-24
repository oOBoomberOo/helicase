use super::*;
#[derive(Debug)]
pub struct EnumDeclaration {
	name: Identifier,
	items: Vec<Identifier>,
}

impl EnumDeclaration {
	pub fn new(name: Identifier, items: Vec<Identifier>) -> EnumDeclaration {
		EnumDeclaration { name, items }
	}
}
