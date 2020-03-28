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
	fn process_each_value(&self, content: &str, path: &str, resource: &Resource, context: &mut Context) -> impl Iterator<Item = TagsError> {
		let kind = match resource.get_extension() {
			Extension::Tags(v) => v,
			_ => TagKind::Other,
		};

		let mut result = Vec::new();

		match kind {
			TagKind::Function => result.push(self.with_function_tags(content, path, resource, context)),
			_ => ()
		};

		result.into_iter().flatten()
	}
	
	fn with_function_tags(&self, content: &str, path: &str, resource: &Resource, context: &mut Context) -> Vec<TagsError> {
		let mut errors = Vec::new();
		let namespace = Namespace::new(path, NamespaceKind::Function);
		if !namespace.exist(context) {
			let range = get_json_field(content, path);
			let content = namespace;
			let span = Span::new(resource.id, content, range);
			errors.push(TagsError::FunctionNotFound(span));
		}
		
		errors
	}
}

impl Processor for TagsProcessor {
	fn process(resource: &Resource, context: &mut Context) -> PResult<Vec<PError>> {
		let content = resource.read()?;
		let tags: TagsProcessor = serde_json::from_str(&content)?;
		let result = tags.values
			.iter()
			.flat_map(|path| tags.process_each_value(&content, path, resource, context))
			.map(PError::from)
			.collect();
		Ok(result)
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