use super::*;

#[derive(Debug)]
pub struct ScoreboardDeclaration {
	name: String,
	objective: String,
	display_name: Option<Nbt>
}

impl ScoreboardDeclaration {
	pub fn new(name: Identifier, objective: Identifier, display_name: Option<Nbt>) -> Self {
		ScoreboardDeclaration { name, objective, display_name }
	}
}