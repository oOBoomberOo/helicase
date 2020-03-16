#[macro_export]
macro_rules! get_pos {
	($x:expr) => {
		match $x.peek() {
			Some((index, _)) => *index,
			None => unreachable!()
		}
	}
}