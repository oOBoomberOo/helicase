use super::*;

#[derive(Debug)]
pub enum Statement {
	Attribute(Attribute, Box<Statement>),
	FunctionDeclaration(FunctionDeclaration),
	TraitDeclaration(TraitDeclaration),
	Declare(Identifier, Expression),
	Expression(Expression),
	None
}