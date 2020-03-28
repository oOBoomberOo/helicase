use super::{ExtraChar, Span, TokenStream};
use std::collections::HashMap;
use itertools::Itertools;

pub enum Nbt<'a> {
	Compound(HashMap<String, Nbt<'a>>, Span<'a>),
	List(Vec<Nbt<'a>>, Span<'a>),
	String(Span<'a>),
	Number(Span<'a>),
	Bool(Span<'a>),
	Null(Span<'a>),
}

use std::fmt;
use crate::get_pos;
impl<'a> Nbt<'a> {
	pub fn span(&'a self) -> &'a Span {
		match self {
			Nbt::Compound(_, span)
			| Nbt::List(_, span)
			| Nbt::String(span)
			| Nbt::Number(span)
			| Nbt::Bool(span)
			| Nbt::Null(span) => span,
		}
	}

	pub fn parse(stream: &mut TokenStream, source: &'a str) -> Self {
		if let Some(&token) = stream.peek() {
			let (_, token) = token;

			if token == '[' {
				Nbt::parse_list(stream, source)
			} else if token == '{' {
				Nbt::parse_compound(stream, source)
			} else if token == '"' || token == '\'' {
				Nbt::parse_string(stream, source)
			} else if token.is_number() {
				Nbt::parse_number(stream, source)
			} else {
				Nbt::parse_lookahead(stream, source)
			}
		} else {
			let pos = get_pos!(stream);
			let span = Span::from_pos(source, pos);
			Nbt::Null(span)
		}
	}

	fn parse_list(stream: &mut TokenStream, source: &'a str) -> Self {
		todo!()
	}

	fn parse_compound(stream: &mut TokenStream, source: &'a str) -> Self {
		let start = get_pos!(stream);
		let mut end = start;
		let mut result = HashMap::new();

		stream.next();

		loop {
			let (key, value, index) = Nbt::parse_compound_key(stream, source);
			result.insert(key, value);
			
			if let Some(&token) = stream.peek() {
				let (_, token) = token;
				
				if token == '}' {
					end = index;
					stream.next();
					break;
				}
			}
			else {
				break;
			}
		}
		
		let span = Span::new(source, start..=end);
		Nbt::Compound(result, span)
	}

	fn parse_compound_key(stream: &mut TokenStream, source: &'a str) -> (String, Nbt<'a>, usize) {
		let key = stream
			.skip_while(|&(_, token)| token.is_whitespace())
			.take_while(|&(_, token)| token != ':')
			.map(|(_, token)| token)
			.collect::<String>()
			.trim()
			.to_owned();
		
		stream
			.peeking_take_while(|&(_, token)| token.is_whitespace())
			.for_each(|_| {});
		let value = Nbt::parse(stream, source);
		
		let end = get_pos!(stream);

		(key, value, end)
	}

	fn parse_string(stream: &mut TokenStream, source: &'a str) -> Self {
		todo!()
	}

	fn parse_number(stream: &mut TokenStream, source: &'a str) -> Self {
		let start = get_pos!(stream);
		let end = stream
			.peeking_take_while(|(_, token)| token.is_number())
			.last()
			.map(|(index, _)| index)
			.unwrap_or(start);

		let span = Span::new(source, start..=end);
		Nbt::Number(span)
	}

	fn parse_lookahead(stream: &mut TokenStream, source: &'a str) -> Self {
		let start = get_pos!(stream);
		let mut end = start;
		
		while let Some(&token) = stream.peek() {
			let (index, _) = token;
			end = index;

			let span = Span::new(source, start..=end);
			stream.next();

			if let Some(result) = Nbt::matched(span) {
				return result;
			}
		}

		let span = Span::new(source, start..=end);
		Nbt::Null(span)
	}

	fn matched(span: Span<'a>) -> Option<Self> {
		match span.content() {
			"true" | "false" => return Nbt::Bool(span).into(),
			_ => ()
		};

		None
	}
}

impl<'a> fmt::Debug for Nbt<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Nbt::Compound(map, _) => map.fmt(f),
			Nbt::List(list, _) => list.fmt(f),
			Nbt::String(span) => span.fmt(f),
			Nbt::Number(span) => span.fmt(f),
			Nbt::Bool(span) => span.fmt(f),
			Nbt::Null(span) => write!(f, "Null Value [{}]", span),
		}
	}
}