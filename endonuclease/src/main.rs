use helicase::lexer::{lex, prelude::Token};
use helicase::parser::parse;

fn main() {
	let content: &str = include_str!("../test/main.mcfunction");

	let mut stream = content.char_indices().peekable();
	let tokens = lex(&mut stream);
	println!("{:#?}", tokens);
	let tokens: Vec<Token> = tokens.into_iter().filter_map(|x| {
		match x {
			Ok(result) => Some(result),
			Err(error) => { println!("{}", error); None }
		}
	})
	.collect();
	let result = parse(&tokens);
	println!("{:#?}", result);
}