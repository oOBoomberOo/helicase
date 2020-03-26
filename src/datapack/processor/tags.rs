use super::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::datapack::prelude::{Extension, TagKind, NamespaceKind};
use crate::utils::get_json_field;

#[derive(Debug, Deserialize, Serialize)]
pub struct TagsProcessor {
	replace: Option<bool>,
	values: Vec<String>,
}

impl TagsProcessor {
	fn process_each_value(&self, content: &str, path: &str, resource: &Resource, context: &mut Context) -> PResult<()> {
		let kind = match resource.get_extension() {
			Extension::Tags(v) => v,
			_ => TagKind::Other,
		};

		match kind {
			TagKind::Function => self.with_function_tags(content, path, resource, context),
			_ => Ok(())
		}
	}
	
	fn with_function_tags(&self, content: &str, path: &str, resource: &Resource, context: &mut Context) -> PResult<()> {
		let namespace = Namespace::new(path, NamespaceKind::Function);
		if !namespace.exist(context) {
			let range = get_json_field(content, path);
			let content = namespace;
			let span = Span::new(resource.id, content, range);
			return Err(TagsError::FunctionNotFound(span).into());
		}
		Ok(())
	}
}

impl Processor for TagsProcessor {
	fn process(resource: &Resource, context: &mut Context) -> PResult<()> {
		let content = fs::read_to_string(&resource.physical)?;
		let tags: TagsProcessor = serde_json::from_str(&content)?;

		tags.values
			.iter()
			.try_for_each(|path| tags.process_each_value(&content, path, resource, context))?;
		Ok(())
	}
}

#[derive(Debug, Error)]
pub enum TagsError {
	#[error("function-not-found")]
	FunctionNotFound(Span<Namespace>)
}

use std::collections::HashMap;
impl ProcessorError for TagsError {
	fn report(&self, message: &ErrorTemplates) -> Diagnostic<usize> {
		match self {
			TagsError::FunctionNotFound(span) => {
				let error = message.tag.get(&self.error_id()).unwrap();
				let mut replacer: HashMap<&str, &str> = HashMap::default();
				let path = span.content.to_path().display().to_string();
				replacer.insert("${path}", &path);
				
				error.report(&[span], &replacer)
			}
		}
	}
}