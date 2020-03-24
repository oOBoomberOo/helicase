use super::*;

use std::collections::HashMap;
pub enum Nbt {
	Compound(HashMap<String, Nbt>),
	List(Vec<Nbt>),
	Literal(String),
	Float(f32),
	Double(f64),
	Byte(i8),
	Short(i16),
	Integer(i32),
	Long(i64),
	Boolean(bool)
}

impl Nbt {
	pub fn new_compound(list: Vec<(Identifier, Nbt)>) -> Nbt {
		let mut result = HashMap::new();
		for (key, value) in list {
			result.insert(key, value);
		}

		Nbt::Compound(result)
	}

	pub fn new_list(item: Vec<Nbt>) -> Nbt {
		Nbt::List(item)
	}

	pub fn new_literal(value: String) -> Nbt {
		Nbt::Literal(value)
	}

	pub fn new_float(value: f32) -> Nbt {
		Nbt::Float(value)
	}

	pub fn new_double(value: f64) -> Nbt {
		Nbt::Double(value)
	}

	pub fn new_byte(value: i8) -> Nbt {
		Nbt::Byte(value)
	}

	pub fn new_short(value: i16) -> Nbt {
		Nbt::Short(value)
	}

	pub fn new_integer(value: i32) -> Nbt {
		Nbt::Integer(value)
	}

	pub fn new_long(value: i64) -> Nbt {
		Nbt::Long(value)
	}

	pub fn new_boolean(value: bool) -> Nbt {
		Nbt::Boolean(value)
	}
}

use std::fmt;
impl fmt::Debug for Nbt {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Nbt::Boolean(state) => write!(f, "{}", state),
			Nbt::Long(value) => write!(f, "{}L", value),
			Nbt::Integer(value) => write!(f, "{}", value),
			Nbt::Short(value) => write!(f, "{}s", value),
			Nbt::Byte(value) => write!(f, "{}b", value),
			Nbt::Double(value) => write!(f, "{}d", value),
			Nbt::Float(value) => write!(f, "{}f", value),
			Nbt::Literal(value) => write!(f, "{}", value),
			Nbt::List(items) => write!(f, "{:#?}", items),
			Nbt::Compound(items) => write!(f, "{:#?}", items),
		}
	}
}