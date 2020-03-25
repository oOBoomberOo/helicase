use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use codespan_reporting::diagnostic::Diagnostic;

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorTemplates {
	pub advancement: ErrorGroup,
}

pub type ErrorGroup = HashMap<String, ErrorTemplate>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorTemplate {
	labels: Option<Vec<ErrorLabel>>,
	message: Option<String>,
	notes: Option<Vec<String>>,
}

impl ErrorTemplate {
	pub fn labels(&self) -> &[ErrorLabel] {
		if let Some(labels) = &self.labels {
			labels
		} else {
			&[]
		}
	}

	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	pub fn notes(&self) -> &[String] {
		if let Some(notes) = &self.notes {
			notes.as_slice()
		} else {
			&[]
		}
	}

	pub fn report<T: Display + Debug>(&self, spans: &[&Span<T>], replacer: &HashMap<&str, &str>) -> Diagnostic<usize> {
		let labels: Vec<Label<usize>> = self.labels().iter().map(|x| x.to_label(spans, replacer)).collect();

		let notes: Vec<String> = self.notes()
			.iter()
			.map(|x| replace_message(x, replacer))
			.collect();
		
		let mut diagnostic = Diagnostic::error()
			.with_labels(labels)
			.with_notes(notes);
		
		if let Some(message) = self.message() {
			let message = replace_message(message, replacer);
			diagnostic = diagnostic.with_message(message);
		}

		diagnostic
	}
}

use crate::datapack::processor::prelude::Span;
use codespan_reporting::diagnostic::Label;
use std::fmt::{Debug, Display};
use std::ops::Range;
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorLabel {
	pub position: usize,
	pub message: Option<String>,
	primary: Option<bool>,
}

impl ErrorLabel {
	pub fn is_primary(&self) -> bool {
		self.primary.unwrap_or(false)
	}

	pub fn to_label<T: Display + Debug>(&self, spans: &[&Span<T>], replacer: &HashMap<&str, &str>) -> Label<usize> {
		
		let span = spans[self.position];

		let file_id: usize = span.id;
		let range: Range<usize> = span.range.clone();

		let mut label = {
			if self.is_primary() {
				Label::primary(file_id, range)
			} else {
				Label::secondary(file_id, range)
			}
		};

		if let Some(message) = &self.message {
			let message = replace_message(message, replacer);
			label = label.with_message(message);
		}

		label
	}
}

pub fn replace_message(input: &str, replacer: &HashMap<&str, &str>) -> String {
	let mut result = input.to_owned();
	for (key, value) in replacer {
		result = result.replace(key, value);
	}

	result
}