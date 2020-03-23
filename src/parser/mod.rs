pub mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub main, "/parser/main.rs");

pub fn parse(content: &str) {
	let parser = main::TermParser::new();
	let result = parser.parse(content).unwrap();
	println!("{:#?}", result);
}