use std::path::PathBuf;
use std::io;
use std::path::StripPrefixError;
use serde_json as js;
use crate::datapack::prelude::{Diagnostic};
use crate::datapack::processor::prelude::*;

pub type PResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	NotExist(PathBuf),
	Io(io::Error),
	StripPrefix(StripPrefixError),
	Serde(js::Error),
	Advancement(AdvancementError),
	Tags(TagsError),
}

use std::fmt;
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::NotExist(path) => write!(f, "'{}' does not exists", path.display()),
			Error::Io(error) => write!(f, "{}", error),
			Error::StripPrefix(error) => write!(f, "{}", error),
			Error::Serde(error) => write!(f, "{}", error),
			Error::Advancement(error) => write!(f, "{}", error),
			Error::Tags(error) => write!(f, "{}", error),
		}
	}
}

impl ProcessorError for Error {
	fn report(&self, message: &ErrorTemplates) -> Diagnostic<usize> {
		match self {
			Error::NotExist(path) => {
				let message = format!("'{}' does not exist", path.display());
				Diagnostic::error()
					.with_message(message)
			},
			Error::Io(error) => {
				let message = format!("{}", error);
				Diagnostic::error()
					.with_message(message)
			},
			Error::StripPrefix(error) => {
				let message = format!("{}", error);
				Diagnostic::error()
					.with_message(message)
			},
			Error::Serde(error) => {
				let message = format!("{}", error);
				Diagnostic::error()
					.with_message(message)
			},
			Error::Advancement(error) => error.report(message),
			Error::Tags(error) => error.report(message)
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
	Error,
	io::Error => Io,
	StripPrefixError => StripPrefix,
	js::Error => Serde,
	AdvancementError => Advancement,
	TagsError => Tags
}