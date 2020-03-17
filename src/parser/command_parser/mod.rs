mod say;
mod me;

use say::SayCommand;
use me::MeCommand;

use super::prelude::*;
use crate::lexer::prelude::{Span, Token};

pub struct CommandParser;

impl Parser for CommandParser {
	fn parse(self, stream: &mut ParseStream, context: &mut Context) -> ParseResult<Box<dyn Parser>> {
		if let Some(command) = stream.take(1).next() {
			match command.value() {
				Some("say") => SayCommand::default().parse(stream, context),
				Some("me") => MeCommand::default().parse(stream, context),
				_ => Err(CommandError::unknown_command(command).into())
			}
		}
		else {
			unreachable!()
		}
	}
}

pub enum CommandError {
	UnknownCommand { span: Span, value: String }
}

use std::fmt;
impl fmt::Debug for CommandError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CommandError::UnknownCommand { span, value } => write!(f, "Unknown command '{}' at {}", value, span)
		}
	}
}

impl CommandError {
	fn unknown_command(command: &Token) -> CommandError {
		let span = command.span().expect("Unable to get span of a token").to_owned();
		let value = command.value().expect("Invalid command token").to_owned();
		CommandError::UnknownCommand { span, value }
	}
}

impl ParserError for CommandError {
	fn diagnos(&self, file_id: FileId) -> Diagnostic<FileId> {
		match self {
			CommandError::UnknownCommand { span, value } => {
				let range = span;
				Diagnostic::error()
					.with_message(format!("Unknown command: {}", value))
					.with_labels(vec![
						Label::primary(file_id, range)
					])
			}
		}
	}
}