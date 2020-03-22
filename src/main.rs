mod lexer;
use lexer::prelude::*;

use std::fs;

fn main() {
	let content = fs::read_to_string("test/main.mcf").unwrap();
	let lexer = Lexer::new(content);
	let result = lexer.lex();
	println!("{:#?}", result);
}