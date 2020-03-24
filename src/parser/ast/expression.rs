use super::*;

#[derive(Debug)]
pub enum Expression {
	Number(i32),
	Literal(String),
	Ident(Identifier),
	Operation(Box<Expression>, Operation, Box<Expression>),
	Access(Box<Expression>, Box<Identifier>)
}