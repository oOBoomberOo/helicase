mod function;
mod advancement;
mod span;
use prelude::*;

pub trait Processor {
	fn process(resource: &Resource, context: &mut Context) -> Result<(), Error>;
}

use std::fmt::{Debug, Display};
pub trait ProcessorError: Debug + Display {
	fn report(&self, message: &ErrorTemplates) -> Diagnostic<usize>;
	fn error_id(&self) -> String {
		format!("{}", &self)
	}
}

pub mod prelude {
	pub use super::span::Span;
	pub use super::function::FunctionProcessor;
	pub use super::advancement::{AdvancementProcessor, AdvancementError};
	pub use crate::datapack::prelude::{Resource, Context, Error, Namespace};
	pub use crate::error_template::ErrorTemplates;
	pub use super::{Processor, ProcessorError};

	pub use std::ops::Range;
	pub use codespan_reporting::diagnostic::{Diagnostic, Label};
}