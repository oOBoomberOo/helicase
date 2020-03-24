mod parser;
use std::fs;

fn main() {
	let content = fs::read_to_string("test/main.mcf").unwrap();
	match parser::parse(&content) {
		Ok(result) => println!("{:#?}", result),
		Err(error) => println!("{:?}", error)
	}
}