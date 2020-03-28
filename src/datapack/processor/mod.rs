mod function;
mod advancement;
mod span;
mod tags;
use prelude::*;

pub trait Processor {
	fn process(resource: &Resource, context: &mut Context) -> PResult<Vec<PError>>;
}

use std::fmt::{Debug, Display};
pub trait ProcessorError: Debug + Display {
	fn report(&self, message: &ErrorTemplates) -> Diagnostic<usize>;
	fn error_id(&self) -> String {
		self.to_string()
	}
}

pub mod prelude {
	pub use super::span::Span;
	pub use super::function::{FunctionProcessor, FunctionError};
	pub use super::advancement::{AdvancementProcessor, AdvancementError};
	pub use super::tags::{TagsProcessor, TagsError};
	pub use crate::datapack::prelude::{Resource, Context, PResult, PError, Namespace};
	pub use crate::error_template::ErrorTemplates;
	pub use super::{Processor, ProcessorError};
	pub use thiserror::Error;
	pub use std::ops::Range;
	pub use codespan_reporting::diagnostic::{Diagnostic, Label};
}