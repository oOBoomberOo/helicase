use super::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
	pub value: String,
	pub kind: String,
}

use std::path::{Path, PathBuf};
impl Namespace {
	pub fn new(value: impl Into<String>, kind: impl Into<String>) -> Namespace {
		let value = value.into();
		let kind = kind.into();
		Namespace { value, kind }
	}

	pub fn from_path(path: &Path) -> Namespace {
		let mut stream = path.components().map(|x| x.as_os_str()).map(|x| x.to_string_lossy()).map(|x| x.to_string()).skip(1);
		
		let namespace = stream.next().unwrap_or_else(|| "minecraft".to_owned());
		let kind = stream.next().unwrap_or_else(|| "functions".to_owned());
		let mut rest = stream.collect::<Vec<String>>().join("/");
		
		if let Some(extension) = path.extension() {
			let extension = extension.to_string_lossy();
			rest = rest.replace(&format!(".{}", extension), "");
		}

		let value = format!("{}:{}", namespace, rest);
		Namespace { value, kind }
	}

	pub fn to_path(&self) -> PathBuf {
		let mut components = self.value.split(':');
		let namespace = components.next().unwrap_or("minecraft");
		let kind = &self.kind;
		let rest = {
			let rest = components.next().unwrap_or_default().to_owned();
			let extension = Namespace::extension_from_kind(kind);
			match extension {
				Some(extension) => format!("{}.{}", rest, extension),
				None => rest
			}
		};
		
		PathBuf::from("data")
			.join(namespace)
			.join(kind)
			.join(rest)
	}

	pub fn extension_from_kind(kind: &str) -> Option<&str> {
		match kind {
			"advancements" => Some("json"),
			"functions" => Some("mcfunction"),
			"loot_tables" => Some("json"),
			"tags" => Some("json"),
			"structures" => Some("nbt"),
			_ => None
		}
	}

	pub fn get_resource<'a>(&'a self, context: &'a Context) -> Option<&'a &Resource> {
		match self.kind.as_ref() {
			"advancements" => context.advancement_list.get(&self),
			"functions" => context.function_list.get(&self),
			"loot_tables" => context.loot_table_list.get(&self),
			"tags" => context.tags_list.get(&self),
			_ => None
		}
	}

	pub fn exist(&self, context: &Context) -> bool {
		self.get_resource(context).is_some()
	}
}

use std::fmt;
impl fmt::Display for Namespace {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value)
	}
}