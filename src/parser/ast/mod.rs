mod expression;
mod function;
mod nbt;
mod statement;
mod traits;
mod enums;
mod attribute;
mod operation;

pub type Identifier = String;

pub use expression::*;
pub use function::*;
pub use statement::*;
pub use traits::*;
pub use enums::*;
pub use nbt::*;
pub use attribute::*;
pub use operation::*;

pub fn pop_string(input: &str) -> &str {
	&input[..input.len() - 1]
}