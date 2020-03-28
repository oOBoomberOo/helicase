use super::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::utils::get_json_field;

#[derive(Debug, Deserialize, Serialize)]
pub struct AdvancementProcessor {
	criteria: HashMap<String, Criteria>,
	parent: Option<String>,
}

impl AdvancementProcessor {
	fn process_with_parent(&self, resource: &Resource, parent: &Namespace, content: &str, context: &mut Context) -> Vec<AdvancementError> {
		let mut result = Vec::new();

		if !parent.exist(context) {
			let range = get_json_field(content, &parent.value);
			let span = Span::new(resource.id, parent.clone(), range);
			result.push(AdvancementError::ParentNotFound(span));
		}
		
		result
	}

	fn process_without_parent(&self, _resource: &Resource, _context: &mut Context) -> Vec<AdvancementError> {
		Vec::default()
	}
}

impl Processor for AdvancementProcessor {
	fn process(resource: &Resource, context: &mut Context) -> PResult<Vec<PError>> {
		let content = resource.read()?;
		let advancement: AdvancementProcessor = serde_json::from_str(&content)?;

		let mut result = Vec::new();

		if let Some(parent) = &advancement.parent.clone().map(|value| Namespace::new(&value, "advancements")) {
			result.append(&mut advancement.process_with_parent(resource, parent, &content, context));
		} else {
			result.append(&mut advancement.process_without_parent(resource, context));
		}

		let result = result.into_iter().map(PError::from).collect();
		Ok(result)
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct Criteria {
	trigger: String,
	conditions: Option<Value>,
}

#[derive(Debug, Error)]
pub enum AdvancementError {
	#[error("invalid-parent")]
	ParentNotFound(Span<Namespace>),
	#[error("circular-reference")]
	CircularReference(Span<String>, Span<String>),
}

impl ProcessorError for AdvancementError {
	fn report(&self, message: &ErrorTemplates) -> Diagnostic<usize> {
		match self {
			AdvancementError::ParentNotFound(span) => {
				let error = message.advancement.get(&self.error_id()).unwrap();

				let path = span.content.to_path().display().to_string();

				let mut message: HashMap<&str, &str> = HashMap::default();
				message.insert("${namespace}", &span.content.value);
				message.insert("${path}", &path);

				error.report(&[span], &message)
			}
			AdvancementError::CircularReference(first, second) => {
				let error = message.advancement.get(&self.error_id()).unwrap();
				let mut message: HashMap<&str, &str> = HashMap::default();
				message.insert("${self_namespace}", &first.content);
				message.insert("${other_namespace}", &second.content);

				error.report(&[first, second], &message)
			}
		}
	}
}
