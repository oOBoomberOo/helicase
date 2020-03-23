mod parser;
use std::fs;

fn main() {
	let content = fs::read_to_string("test/main.mcf").unwrap();
	parser::parse(&content);
}