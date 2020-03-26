use super::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
	pub value: String,
	pub kind: NamespaceKind,
}

use std::path::{Path, PathBuf};
impl Namespace {
	pub fn new(value: impl Into<String>, kind: impl Into<NamespaceKind>) -> Namespace {
		let value = value.into();
		let kind = kind.into();
		Namespace { value, kind }
	}

	pub fn from_path(path: &Path) -> Namespace {
		let mut stream = path.components().map(|x| x.as_os_str()).map(|x| x.to_string_lossy()).map(|x| x.to_string()).skip(1);
		
		let namespace = stream.next().unwrap_or_else(|| "minecraft".to_owned());
		let kind = stream.next().unwrap_or_else(|| "unknown".to_owned());
		let mut rest = stream.collect::<Vec<String>>().join("/");
		
		if let Some(extension) = path.extension() {
			let extension = extension.to_string_lossy();
			rest = rest.replace(&format!(".{}", extension), "");
		}

		let value = format!("{}:{}", namespace, rest);
		let kind = NamespaceKind::from(kind);
		Namespace { value, kind }
	}

	pub fn to_path(&self) -> PathBuf {
		let mut components = self.value.split(':');
		let namespace = components.next().unwrap_or("minecraft");
		let kind = &self.kind;
		let rest = {
			let rest = components.next().unwrap_or_default().to_owned();
			let extension = kind.get_extension();
			match extension {
				Some(extension) => format!("{}.{}", rest, extension),
				None => rest
			}
		};
		
		PathBuf::from("data")
			.join(namespace)
			.join(kind.value())
			.join(rest)
	}

	pub fn get_resource<'a>(&'a self, context: &'a Context) -> Option<&'a &Resource> {
		match self.kind {
			NamespaceKind::Advancement => context.advancement_list.get(&self),
			NamespaceKind::Function => context.function_list.get(&self),
			NamespaceKind::LootTable => context.loot_table_list.get(&self),
			NamespaceKind::Tags => context.tags_list.get(&self),
			NamespaceKind::Other(_) => None
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NamespaceKind {
	Function,
	Advancement,
	LootTable,
	Tags,
	Other(String)
}

impl NamespaceKind {
	pub fn get_extension(&self) -> Option<&str> {
		match self {
			NamespaceKind::Function => Some("mcfunction"),
			NamespaceKind::Advancement => Some("json"),
			NamespaceKind::LootTable => Some("json"),
			NamespaceKind::Tags => Some("json"),
			NamespaceKind::Other(_) => None
		}
	}

	pub fn value(&self) -> &str {
		match self {
			NamespaceKind::Function => "functions",
			NamespaceKind::Advancement => "advancements",
			NamespaceKind::LootTable => "loot_tables",
			NamespaceKind::Tags => "tags",
			NamespaceKind::Other(v) => v
		}
	}
}

impl From<String> for NamespaceKind {
	fn from(value: String) -> NamespaceKind {
		match value.as_ref() {
			"advancements" => NamespaceKind::Advancement,
			"functions" => NamespaceKind::Function,
			"loot_tables" => NamespaceKind::LootTable,
			"tags" => NamespaceKind::Tags,
			_ => NamespaceKind::Other(value)
		}
	}
}


impl From<&str> for NamespaceKind {
	fn from(value: &str) -> NamespaceKind {
		match value {
			"advancements" => NamespaceKind::Advancement,
			"functions" => NamespaceKind::Function,
			"loot_tables" => NamespaceKind::LootTable,
			"tags" => NamespaceKind::Tags,
			other => NamespaceKind::Other(other.to_owned())
		}
	}
}