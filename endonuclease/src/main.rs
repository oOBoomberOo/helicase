use helicase::lexer::{lex};
fn main() {
	let content: &str = include_str!("../test/main.mcfunction");

	let mut stream = content.char_indices().peekable();
	lex(&mut stream).into_iter().for_each(|x| match x {
		Ok(x) => println!("{:?}", x),
		Err(x) => println!("{:?}", x),
	});
}