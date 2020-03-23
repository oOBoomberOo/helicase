mod lexer;
mod parser;

use lexer::prelude::*;
use parser::prelude::*;

use std::fs;

fn main() {
	let content = fs::read_to_string("test/main.mcf").unwrap();
	let lexer = Lexer::new(content);
	let result = lexer.lex();
	
	let parser = Parser::new(result);
	let result = parser.parse();
	println!("{:#?}", result);
}