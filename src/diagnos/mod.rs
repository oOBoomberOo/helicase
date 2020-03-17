use crate::parser::prelude::{ParseResult, ParserError};
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::ColorArg;
use codespan_reporting::term;
use structopt::StructOpt;

pub fn diagnostic(parse_result: Vec<ParseResult<()>>, file_id: usize) -> Vec<Diagnostic<usize>> {
	parse_result.iter().filter_map(|x| match x {
		Ok(_) => None,
		Err(error) => Some(error.diagnos(file_id))
	}).collect()
}

use std::fmt::Display;
pub fn report<A: Clone + Display, B: AsRef<str>>(diagnostics: &[Diagnostic<usize>], files: SimpleFiles<A, B>) -> std::io::Result<()> {
	let opts = Opts::from_args();
	let mut writer = StandardStream::stderr(opts.color.into());
	let config = term::Config::default();

	for diagnostic in diagnostics {
		term::emit(&mut writer, &config, &files, diagnostic)?;
	}
	Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "emit")]
struct Opts {
	#[structopt(
		long = "color",
		parse(try_from_str),
		default_value = "auto",
		possible_values = ColorArg::VARIANTS,
		case_insensitive = true
	)]
	color: ColorArg,
}