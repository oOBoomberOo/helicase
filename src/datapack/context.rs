use super::*;

use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct Context<'a> {
	pub function_list: HashMap<Namespace, &'a Resource>,
	pub advancement_list: HashMap<Namespace, &'a Resource>,
	pub loot_table_list: HashMap<Namespace, &'a Resource>,
	pub tags_list: HashMap<Namespace, &'a Resource>,
	pub meta: Option<&'a Resource>,
	pub content: Option<&'a str>
}

impl<'a> Context<'a> {
	pub fn add_advancement(&mut self, resource: &'a Resource) {
		let namespace = Namespace::from_path(&resource.relative);
		self.advancement_list.insert(namespace, resource);
	}
	pub fn add_function(&mut self, resource: &'a Resource) {
		let namespace = Namespace::from_path(&resource.relative);
		self.function_list.insert(namespace, resource);
	}
	pub fn add_loot_table(&mut self, resource: &'a Resource) {
		let namespace = Namespace::from_path(&resource.relative);
		self.loot_table_list.insert(namespace, resource);
	}
	pub fn add_tags(&mut self, resource: &'a Resource) {
		let namespace = Namespace::from_path(&resource.relative);
		self.tags_list.insert(namespace, resource);
	}
	pub fn set_meta(&mut self, resource: &'a Resource) {
		self.meta = Some(resource);
	}
}