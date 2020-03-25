#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
	Other,
	Function,
	Advancement,
	LootTable,
	UnifiedLootTable,
	Tags,
	PackMeta
}