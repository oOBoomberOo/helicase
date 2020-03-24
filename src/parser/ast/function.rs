use super::*;

#[derive(Debug)]
pub struct FunctionDeclaration {
	name: Identifier,
	parameters: Vec<ParameterDeclaration>,
	return_type: Option<Identifier>,
	content: Vec<Statement>
}

impl FunctionDeclaration {
	pub fn new(name: Identifier, parameters: Vec<ParameterDeclaration>, content: Vec<Statement>, return_type: Option<Identifier>) -> FunctionDeclaration {
		FunctionDeclaration { name, parameters, content, return_type }
	}
}

#[derive(Debug)]
pub struct ParameterDeclaration {
	name: Identifier,
	kind: String
}

impl ParameterDeclaration {
	pub fn new(name: Identifier, kind: String) -> ParameterDeclaration {
		ParameterDeclaration { name, kind }
	}
}

#[derive(Debug)]
pub struct FunctionCall {
	name: Identifier,
	parameters: Vec<Expression>
}

impl FunctionCall {
	pub fn new(name: Identifier, parameters: Vec<Expression>) -> FunctionCall {
		FunctionCall { name, parameters }
	}
}