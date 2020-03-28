use super::prelude::*;
use serde::{Serialize, Deserialize};
use crate::datapack::prelude::{Extension, TagKind, NamespaceKind};
use crate::utils::get_json_field;

#[derive(Debug)]
pub struct TagsProcessor<'a> {
	content: &'a str,
	template: TagsTemplate,
	resource: &'a Resource,
}

impl<'a> TagsProcessor<'a> {
	fn new(template: TagsTemplate, content: &'a str, resource: &'a Resource) -> TagsProcessor<'a> {
		TagsProcessor {
			content,
			template,
			resource,
		}
	}

	fn path_list(&self) -> &[String] {
		&self.template.values
	}
	
	fn process_each_value(&self, path: &str, context: &mut Context) -> Option<TagsError> {
		let extension = self.resource.get_extension();
		let kind = match extension {
			Extension::Tags(v) => v,
			_ => TagKind::Other,
		};

		match kind {
			TagKind::Function => self.with_function_tags(path, context),
			TagKind::Block => self.with_block_tags(),
			TagKind::Item => self.with_item_tags(),
			TagKind::Entity => self.with_entity_tags(),
			_ => None
		}
	}

	fn with_entity_tags(&self) -> Option<TagsError> {
		todo!()
	}

	fn with_item_tags(&self) -> Option<TagsError> {
		todo!()
	}

	fn with_block_tags(&self) -> Option<TagsError> {
		todo!()
	}
	
	fn with_function_tags(&self, path: &str, context: &mut Context) -> Option<TagsError> {
		let namespace = Namespace::new(path, NamespaceKind::Function);
		if !namespace.exist(context) {
			let range = get_json_field(self.content, path);
			let content = namespace;
			let span = Span::new(self.resource.id, content, range);
			return TagsError::FunctionNotFound(span).into();
		};

		None
	}
}

impl<'a> Processor for TagsProcessor<'a> {
	fn process(resource: &Resource, context: &mut Context) -> PResult<Vec<PError>> {
		let content = resource.read()?;
		let template: TagsTemplate = serde_json::from_str(&content)?;
		let tags = TagsProcessor::new(template, &content, resource);

		let result = tags.path_list()
			.iter()
			.filter_map(|path| tags.process_each_value(path, context))
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct TagsTemplate {
	replace: Option<bool>,
	values: Vec<String>
}