#[derive(Debug)]
pub enum Expression<'a> {
	Number(i32),
	Literal(&'a str),
	Block(Box<Block<'a>>)
}

#[derive(Debug)]
pub enum Block<'a> {
	Expression(Vec<Expression<'a>>),
	FunctionDeclare(FunctionDeclare<'a>)
}

#[derive(Debug)]
pub struct FunctionDeclare<'a> {
	name: &'a str,
	parameter: Vec<Parameter<'a>>,
	content: Box<Block<'a>>
}

impl<'a> FunctionDeclare<'a> {
	pub fn new(name: &'a str, parameter: Vec<Parameter<'a>>, content: Block<'a>) -> FunctionDeclare<'a> {
		Self { name, parameter, content: Box::new(content) }
	}
}

#[derive(Debug)]
pub struct Parameter<'a> {
	name: &'a str,
	kind: &'a str,
}

impl<'a> Parameter<'a> {
	pub fn new(name: &'a str, kind: &'a str) -> Parameter<'a> {
		Self { name, kind }
	}
}