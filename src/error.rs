use std::path::PathBuf;
use std::io;
use std::path::StripPrefixError;
use serde_json as js;
use crate::datapack::prelude::{Diagnostic};
use crate::datapack::processor::prelude::*;

pub type PResult<T> = Result<T, PError>;

#[derive(Debug)]
pub enum PError {
	NotExist(PathBuf),
	Io(io::Error),
	StripPrefix(StripPrefixError),
	Serde(js::Error),
	Advancement(AdvancementError),
	Tags(TagsError),
}

use std::fmt;
impl fmt::Display for PError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			PError::NotExist(path) => write!(f, "'{}' does not exists", path.display()),
			PError::Io(error) => write!(f, "{}", error),
			PError::StripPrefix(error) => write!(f, "{}", error),
			PError::Serde(error) => write!(f, "{}", error),
			PError::Advancement(error) => write!(f, "{}", error),
			PError::Tags(error) => write!(f, "{}", error),
		}
	}
}

impl ProcessorError for PError {
	fn report(&self, message: &ErrorTemplates) -> Diagnostic<usize> {
		match self {
			PError::NotExist(path) => {
				let message = format!("'{}' does not exist", path.display());
				Diagnostic::error()
					.with_message(message)
			},
			PError::Io(error) => {
				let message = format!("{}", error);
				Diagnostic::error()
					.with_message(message)
			},
			PError::StripPrefix(error) => {
				let message = format!("{}", error);
				Diagnostic::error()
					.with_message(message)
			},
			PError::Serde(error) => {
				let message = format!("{}", error);
				Diagnostic::error()
					.with_message(message)
			},
			PError::Advancement(error) => error.report(message),
			PError::Tags(error) => error.report(message)
		}
	}
}

macro_rules! quick_from {
	($x:ty, $($y:ty => $z:tt), *) => {
		$(
			impl From<$y> for $x {
				fn from(error: $y) -> $x {
					<$x>::$z(error)
				}
			}
		)*
	}
}

quick_from!{
	PError,
	io::Error => Io,
	StripPrefixError => StripPrefix,
	js::Error => Serde,
	AdvancementError => Advancement,
	TagsError => Tags
}