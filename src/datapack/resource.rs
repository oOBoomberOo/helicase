use super::prelude::*;
use std::path::{PathBuf, Path};

#[derive(Debug, Clone)]
pub struct Resource {
	pub relative: PathBuf,
	pub physical: PathBuf,
	pub id: usize
}

use std::ffi::OsStr;
use std::fs::DirEntry;
use std::fs;
use std::io;
impl Resource {
	pub fn new(relative: impl Into<PathBuf>, physical: impl Into<PathBuf>, id: usize) -> Resource {
		let relative = relative.into();
		let physical = physical.into();
		Resource { relative, physical, id }
	}

	pub fn pre_process<'a>(&'a self, context: &mut Context<'a>) {
		context.insert_item(self);
	}

	pub fn process(&self, context: &mut Context) -> PResult<Vec<crate::error::PError>> {
		match self.get_extension() {
			Extension::Function => FunctionProcessor::process(&self, context),
			Extension::Advancement => AdvancementProcessor::process(&self, context),
			Extension::Tags(kind) => TagsProcessor::process(&self, context.with_tagkind(kind)),
			_ => Ok(Vec::default())
		}
	}

	pub fn from_entry(entry: DirEntry, parent: &Path, files: &mut Files) -> PResult<Vec<Resource>> {
		let metadata = entry.metadata()?;

		if metadata.is_dir() {
			let result: Vec<Resource> = fs::read_dir(entry.path())?
				.filter_map(|entry| entry.ok())
				.filter_map(|entry| Resource::from_entry(entry, parent, files).ok())
				.flatten()
				.collect();

			Ok(result)
		} else {
			let path = entry.path();
			let content = fs::read_to_string(&path)?;
			let id = files.add(path.to_string_lossy().to_string(), content);
			let physical = path.clone();
			let relative = path.strip_prefix(parent)?;
			let resource = Resource::new(relative, physical, id);
			Ok(vec![resource])
		}
	}

	pub fn get_extension(&self) -> Extension {
		let path: Vec<&OsStr> = self.relative.components().map(|x| x.as_os_str()).collect();

		match path.as_slice() {
			[item] if Extension::is_meta(item) => Extension::PackMeta,
			[_, _, kind, tag_kind, .., last] if Extension::is_tags(kind, last) => Extension::Tags(TagKind::from(tag_kind)),
			[_, _, kind, .., last] => Extension::from_str(kind, last),
			_ => Extension::Other
		}
	}

	pub fn read(&self) -> io::Result<String> {
		fs::read_to_string(&self.physical)
	}

	pub fn namespace(&self) -> Namespace {
		Namespace::from_path(&self.relative)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_mcfunction_extension() {
		let resource = Resource::new("data/namespace/functions/nested/function/reeee.mcfunction", "", 0);
		assert_eq!(resource.get_extension(), Extension::Function);
	}

	#[test]
	fn get_advancement_extension() {
		let resource = Resource::new("data/boomber/advancements/boomber.json", "", 0);
		assert_eq!(resource.get_extension(), Extension::Advancement);
	}

	#[test]
	fn get_loot_table_extension() {
		let resource = Resource::new("data/megumin/loot_tables/entities/creeper.json", "", 0);
		assert_eq!(resource.get_extension(), Extension::LootTable);
	}
	
	#[test]
	fn get_unified_loot_table_extension() {
		let resource = Resource::new("data/megumin/loot_tables/entities/creeper.ult", "", 0);
		assert_eq!(resource.get_extension(), Extension::UnifiedLootTable);
	}

	#[test]
	fn get_tags_extension() {
		let resource = Resource::new("data/boomber/tags/functions/loop.json", "", 0);
		assert_eq!(resource.get_extension(), Extension::Tags(TagKind::Function));
	}

	#[test]
	fn get_packmeta_extension() {
		let resource = Resource::new("pack.mcmeta", "", 0);
		assert_eq!(resource.get_extension(), Extension::PackMeta);
	}
}