use helicase::lexer::{lex, prelude::Token};
use helicase::parser::parse;
use helicase::diagnos::{diagnostic, report};
use codespan_reporting::files::SimpleFiles;
use std::path::PathBuf;
use std::fs;

fn main() {
	let path = PathBuf::from("endonuclease/test/main.mcfunction");
	let content = fs::read_to_string(&path).unwrap();
	let mut files = SimpleFiles::new();
	let file = files.add(path.display().to_string(), &content);

	let mut stream = content.char_indices().peekable();
	let tokens = lex(&mut stream);
	// println!("{:#?}", tokens);
	let tokens: Vec<Token> = tokens
		.into_iter()
		.filter_map(|x| match x {
			Ok(result) => Some(result),
			Err(error) => {
				println!("{}", error);
				None
			}
		})
		.collect();

	let parse_result = parse(&tokens);
	let diagnostics = diagnostic(parse_result, file);
	report(&diagnostics, files).unwrap();
	// println!("{:#?}", result);
}
