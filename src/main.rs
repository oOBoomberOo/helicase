use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "test", about = "A test command line")]
struct Opt {
	#[structopt(parse(from_os_str))]
	path: PathBuf,
}

mod datapack;
mod error;
mod error_template;

fn main() {
	if let Err(e) = run() {
		println!("{}", e);
	}
}

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::{ColorArg, Config};
use datapack::prelude::Files;
use datapack::Datapack;
use error::Error;
use std::str::FromStr;

fn run() -> Result<(), Error> {
	let opt = Opt::from_args();
	let datapack_dir = opt.path;

	if !datapack_dir.exists() {
		return Err(Error::NotExist(datapack_dir));
	}

	let mut files: Files = SimpleFiles::new();

	let datapack = Datapack::new(datapack_dir);
	let diagnostics = datapack.run(&mut files)?;

	let mut writer = StandardStream::stderr(ColorArg::from_str("auto").unwrap().into());
	let config = Config::default();

	for diagnostic in diagnostics {
		term::emit(&mut writer, &config, &files, &diagnostic)?;
	}

	Ok(())
}
