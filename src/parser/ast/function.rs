use super::*;

#[derive(Debug)]
pub struct FunctionDeclaration {
	name: Identifier,
	parameters: Vec<Parameter>,
	content: Vec<Statement>
}

impl FunctionDeclaration {
	pub fn new(name: Identifier, parameters: Vec<Parameter>, content: Vec<Statement>) -> Self {
		FunctionDeclaration { name, parameters, content }
	}
}

#[derive(Debug)]
pub struct Parameter {
	name: Identifier,
	kind: String
}

impl Parameter {
	pub fn new(name: Identifier, kind: String) -> Parameter {
		Parameter { name, kind }
	}
}