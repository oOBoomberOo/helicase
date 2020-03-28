#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
	Other,
	Function,
	Advancement,
	LootTable,
	UnifiedLootTable,
	Tags(TagKind),
	PackMeta
}

impl Extension {
	pub fn from_str(kind: &OsStr, path: &OsStr) -> Extension {
		if Extension::is_function(kind, path) {
			return Extension::Function;
		}
		if Extension::is_advancement(kind, path) {
			return Extension::Advancement;
		}
		if Extension::is_loot_table(kind, path) {
			return Extension::LootTable;
		}
		if Extension::is_unified_loot_table(kind, path) {
			return Extension::UnifiedLootTable;
		}

		Extension::Other
	}

	pub fn ends_with(path: &OsStr, extension: &str) -> bool {
		path.to_string_lossy().ends_with(extension)
	}

	pub fn is_function(kind: &OsStr, path: &OsStr) -> bool {
		kind == "functions" && Extension::ends_with(path, ".mcfunction")
	}

	pub fn is_advancement(kind: &OsStr, path: &OsStr) -> bool {
		kind == "advancements" && Extension::ends_with(path, ".json")
	}

	pub fn is_loot_table(kind: &OsStr, path: &OsStr) -> bool {
		kind == "loot_tables" && Extension::ends_with(path, ".json")
	}

	pub fn is_unified_loot_table(kind: &OsStr, path: &OsStr) -> bool {
		kind == "loot_tables" && Extension::ends_with(path, ".ult")
	}

	pub fn is_tags(kind: &OsStr, path: &OsStr) -> bool {
		kind == "tags" && Extension::ends_with(path, ".json")
	}

	pub fn is_meta(path: &OsStr) -> bool {
		path == "pack.mcmeta"
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagKind {
	Function,
	Block,
	Item,
	Entity,
	Other
}

impl From<Option<&str>> for TagKind {
	fn from(value: Option<&str>) -> TagKind {
		match value {
			Some("functions") => TagKind::Function,
			Some("blocks") => TagKind::Block,
			Some("items") => TagKind::Item,
			Some("entities_type") => TagKind::Entity,
			_ => TagKind::Other
		}
	}
}

use std::ffi::OsStr;
impl From<&OsStr> for TagKind {
	fn from(value: &OsStr) -> TagKind {
		match value {
			x if x == "functions" => TagKind::Function,
			x if x == "blocks" => TagKind::Block,
			x if x == "items" => TagKind::Item,
			x if x == "entities_type" => TagKind::Entity,
			_ => TagKind::Other
		}
	}
}

impl From<&&OsStr> for TagKind {
	fn from(value: &&OsStr) -> TagKind {
		TagKind::from(*value)
	}
}