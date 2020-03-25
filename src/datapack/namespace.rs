#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
	pub value: String,
	pub kind: String,
}

use std::path::{Path, PathBuf};
impl Namespace {
	pub fn new(value: String, kind: String) -> Namespace {
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
			match kind.as_ref() {
				"advancements" => format!("{}.json", rest),
				"functions" => format!("{}.mcfunction", rest),
				"loot_tables" => format!("{}.json", rest),
				"tags" => format!("{}.json", rest),
				"structures" => format!("{}.nbt", rest),
				_ => rest
			}
		};
		
		PathBuf::from("data")
			.join(namespace)
			.join(kind)
			.join(rest)
	}
}

use std::fmt;
impl fmt::Display for Namespace {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value)
	}
}