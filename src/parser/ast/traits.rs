use super::*;

#[derive(Debug)]
pub struct TraitDeclaration {
	name: Identifier,
	inner: TraitInner
}

impl TraitDeclaration {
	pub fn new(name: Identifier, inner: TraitInner) -> TraitDeclaration {
		TraitDeclaration { name, inner }
	}
}

#[derive(Debug, Default)]
pub struct TraitInner {
	nbt: Option<Nbt>,
	functions: Vec<FunctionDeclaration>
}

impl TraitInner {
	pub fn new(nbt: Option<Nbt>, functions: Vec<FunctionDeclaration>) -> Self {
		TraitInner { nbt, functions }
	}
}