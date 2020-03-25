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
use std::fs::File;
use std::io;
impl Resource {
	pub fn new(relative: impl Into<PathBuf>, physical: impl Into<PathBuf>, id: usize) -> Resource {
		let relative = relative.into();
		let physical = physical.into();
		Resource { relative, physical, id }
	}

	pub fn pre_process<'a>(&'a self, context: &mut Context<'a>) {
		match self.get_extension() {
			Extension::Advancement => context.add_advancement(&self),
			Extension::Function => context.add_function(&self),
			Extension::LootTable => context.add_loot_table(&self),
			Extension::Tags => context.add_tags(&self),
			Extension::PackMeta => context.set_meta(&self),
			_ => ()
		}
	}

	pub fn process(&self, context: &mut Context) -> Result<(), Error> {
		match self.get_extension() {
			Extension::Function => FunctionProcessor::process(&self, context),
			Extension::Advancement => AdvancementProcessor::process(&self, context),
			_ => Ok(())
		}
	}

	pub fn from_entry(entry: DirEntry, parent: &Path, files: &mut Files) -> Result<Vec<Resource>, Error> {
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
			[_, _, kind, .., last] if *kind == "functions" && last.to_string_lossy().ends_with(".mcfunction") => Extension::Function,
			[_, _, kind, .., last] if *kind == "advancements" && last.to_string_lossy().ends_with(".json") => Extension::Advancement,
			[_, _, kind, .., last] if *kind == "loot_tables" && last.to_string_lossy().ends_with(".json") => Extension::LootTable,
			[_, _, kind, .., last] if *kind == "loot_tables" && last.to_string_lossy().ends_with(".ult") => Extension::UnifiedLootTable,
			[_, _, kind, .., last] if *kind == "tags" && last.to_string_lossy().ends_with(".json") => Extension::Tags,
			[item] if *item == "pack.mcmeta" => Extension::PackMeta,
			_ => Extension::Other
		}
	}

	pub fn read(&self) -> io::Result<File> {
		File::open(&self.physical)
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
		assert_eq!(resource.get_extension(), Extension::Tags);
	}

	#[test]
	fn get_packmeta_extension() {
		let resource = Resource::new("pack.mcmeta", "", 0);
		assert_eq!(resource.get_extension(), Extension::PackMeta);
	}
}