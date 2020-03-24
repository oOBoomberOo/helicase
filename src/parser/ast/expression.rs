use super::*;

#[derive(Debug)]
pub enum Expression {
	Number(i32),
	Literal(String),
	Ident(Identifier),
	Expression(Box<Expression>),
}