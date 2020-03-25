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

impl Processor for AdvancementProcessor {
	fn process(resource: &Resource, context: &mut Context) -> Result<(), Error> {
		let content = fs::read_to_string(&resource.physical)?;
		let data: AdvancementProcessor = serde_json::from_reader(resource.read()?)?;

		if let Some(parent) = data.parent {
			let namespace = Namespace::new(parent.to_owned(), "advancements".to_owned());
			if let Some(other) = context.advancement_list.get(&namespace) {
				let other_content = fs::read_to_string(&other.physical)?;
				let other_advancement: AdvancementProcessor =
					serde_json::from_reader(other.read()?)?;

				if let Some(other_parent) = other_advancement.parent {
					let this_path = resource.namespace();
					let other_path =
						Namespace::new(other_parent.to_owned(), "advancements".to_owned());

					if this_path == other_path {
						let first = {
							let start = content.find(&parent).unwrap();
							let end = start + parent.len();
							Span::new(resource.id, parent, start..end)
						};
						let second = {
							let start = other_content.find(&other_parent).unwrap();
							let end = start + other_parent.len();
							Span::new(other.id, other_parent, start..end)
						};

						return Err(AdvancementError::CircularReference(first, second).into());
					}
				}
			} else {
				let id = resource.id;
				let start = content.find(&parent).unwrap();
				let end = start + parent.len();
				let span = Span::new(id, namespace, start..end);
				return Err(AdvancementError::ParentNotFound(span).into());
			}
		}

		Ok(())
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

			},
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
