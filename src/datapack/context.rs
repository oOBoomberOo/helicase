use super::*;

use std::collections::HashMap;
#[derive(Debug)]
pub struct Context<'a> {
	pub function_list: HashMap<Namespace, &'a Resource>,
	pub advancement_list: HashMap<Namespace, &'a Resource>,
	pub loot_table_list: HashMap<Namespace, &'a Resource>,
	pub tags_list: HashMap<Namespace, &'a Resource>,
	pub meta: Option<&'a Resource>,
	pub tag_kind: Option<TagKind>
}

impl<'a> Context<'a> {
	pub fn new() -> Context<'a> {
		Context {
			function_list: HashMap::default(),
			advancement_list: HashMap::default(),
			loot_table_list: HashMap::default(),
			tags_list: HashMap::default(),
			meta: None,
			tag_kind: None
		}
	}

	pub fn insert_item(&mut self, resource: &'a Resource) -> &mut Context<'a> {
		let namespace = resource.namespace();
		match resource.get_extension() {
			Extension::Function => self.function_list.insert(namespace, resource),
			Extension::Advancement => self.advancement_list.insert(namespace, resource),
			Extension::LootTable => self.loot_table_list.insert(namespace, resource),
			Extension::Tags(_) => self.tags_list.insert(namespace, resource),
			_ => None
		};

		self
	}

	pub fn with_meta(&mut self, resource: &'a Resource) -> &mut Context<'a> {
		self.meta = Some(resource);
		self
	}
	pub fn with_tagkind(&mut self, kind: TagKind) -> &mut Context<'a> {
		self.tag_kind = Some(kind);
		self
	}
}