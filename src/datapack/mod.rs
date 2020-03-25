use std::path::PathBuf;
use super::Error;

pub mod processor;
mod resource;
mod extension;
mod namespace;
mod context;
use prelude::*;

#[derive(Debug)]
pub struct Datapack {
	path: PathBuf,
}

use codespan_reporting::diagnostic::Diagnostic;
use std::fs;
impl Datapack {
	pub fn new(path: PathBuf) -> Datapack {
		Datapack { path }
	}

	pub fn run(&self, files: &mut Files) -> Result<Vec<Diagnostic<usize>>, Error> {
		let parent = &self.path;
		let resources: Vec<Resource> = fs::read_dir(&self.path)?
			.filter_map(|entry| entry.ok())
			.filter_map(|entry| Resource::from_entry(entry, &parent, files).ok())
			.flatten()
			.collect();

		let context = &mut Context::default();

		resources.iter().for_each(|x| x.pre_process(context));

		let error_messages: ErrorTemplates = serde_json::from_str(include_str!("../../resource/message.json"))?;

		let result: Vec<Diagnostic<usize>> = resources
			.iter()
			.map(|x| x.process(context))
			.filter_map(|x| x.err())
			.map(|x| x.report(&error_messages))
			.collect();

		Ok(result)
	}
}


pub mod prelude {
	pub use crate::error::Error;
	pub use super::resource::Resource;
	pub use super::extension::Extension;
	pub use super::namespace::Namespace;
	pub use super::context::Context;
	pub use super::processor::prelude::*;
	
	use codespan_reporting::files::SimpleFiles;
	pub type Files<'a> = SimpleFiles<String, String>;
}