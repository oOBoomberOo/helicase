use super::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct AdvancementProcessor {
	criteria: HashMap<String, Criteria>,
	parent: Option<String>,
}

impl AdvancementProcessor {
	fn get_field(&self, content: &str, value: &str) -> Range<usize> {
		let pattern = format!(r#""{}""#, value);
		let start = content.find(&pattern).unwrap_or(0);
		let end = start + pattern.len();
		start..end
	}

	fn process_with_parent(&self, resource: &Resource, parent: &Namespace, content: &str, context: &mut Context) -> Result<(), Error> {
		if !parent.exist(context) {
			let range = self.get_field(content, &parent.value);
			let span = Span::new(resource.id, parent.clone(), range);
			return Err(AdvancementError::ParentNotFound(span).into());
		}
		Ok(())
	}

	fn process_without_parent(&self, _resource: &Resource, _context: &mut Context) -> Result<(), Error> {
		Ok(())
	}
}

impl Processor for AdvancementProcessor {
	fn process(resource: &Resource, context: &mut Context) -> Result<(), Error> {
		let content = fs::read_to_string(&resource.physical)?;
		let advancement: AdvancementProcessor = serde_json::from_str(&content)?;

		if let Some(parent) = &advancement.parent.clone().map(|value| Namespace::new(&value, "advancements")) {
			advancement.process_with_parent(resource, parent, &content, context)
		} else {
			advancement.process_without_parent(resource, context)
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct Criteria {
	trigger: String,
	conditions: Option<Value>,
}

use thiserror::Error;
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
