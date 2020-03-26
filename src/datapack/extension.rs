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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagKind {
	Function,
	Block,
	Item,
	Entity,
	Other
}