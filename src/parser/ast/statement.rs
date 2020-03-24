use super::*;

#[derive(Debug)]
pub enum Statement {
	Attribute(Attribute, Box<Statement>),
	FunctionDeclaration(FunctionDeclaration),
	TraitDeclaration(TraitDeclaration),
	EnumDeclaration(EnumDeclaration),
	FunctionCall(FunctionCall),
	Declare(Identifier, Expression),
	Assign(Identifier, Expression),
	Expression(Expression),
	Import(String),
	None
}