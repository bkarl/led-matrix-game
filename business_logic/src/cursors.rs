const MAX_CURSOR_POINTS  : usize = 10;
use crate::helpers::Position;

pub trait Cursor {
	fn get_cursor(&self) -> (&[Position; MAX_CURSOR_POINTS], usize);
}

pub struct SingleCursor {
	cursor_shape: [Position; MAX_CURSOR_POINTS],
	num_cursor_points: usize
}

impl SingleCursor {
	pub const fn new() -> SingleCursor {
		SingleCursor { cursor_shape: [Position {x:0, y:0}; MAX_CURSOR_POINTS], num_cursor_points: 1} 
	}
}

// impl From<Cursor> for SingleCursor{
//     fn from(val: i64) -> Bar {
//         Bar(val)
//     }
// }

impl Cursor for SingleCursor {
	fn get_cursor(&self) -> (&[Position; MAX_CURSOR_POINTS], usize) {
		(&self.cursor_shape, self.num_cursor_points)
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_single_sursor() {
		let c = SingleCursor::new();
		assert_eq!(1, c.num_cursor_points);
		assert_eq!(Position{x: 0, y: 0}, c.cursor_shape[0]);
	}
}