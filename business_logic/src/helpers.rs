use crate::drawing_game::MATRIX_SIZE;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
	pub x : i32,
	pub y : i32
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameMode{
	Navigating,
	Drawing,
}

impl Position{
	pub const fn new() -> Position {
		Position {x: 0 , y: 0}
	}
}

impl Position {
	pub fn translate_in_matrix_bounds(&mut self, delta: Self) {
		let mut newx = self.x + delta.x;
		let mut newy = self.y + delta.y;

		if newx >= MATRIX_SIZE as i32 {
			newx = (MATRIX_SIZE - 1) as i32;
		}
		if newy >= MATRIX_SIZE as i32 {
			newy = (MATRIX_SIZE - 1) as i32;
		}

		if newx < 0 {
			newx = 0;
		}

		if newy < 0 {
			newy = 0;
		}

		self.x = newx;
		self.y = newy;
    }
}

impl Color {
	pub const fn new() -> Color {
		Color {r: 0 , g: 0, b: 0}
	}
}

pub fn rotary_movement_to_position_delta(rotary_last: i32, rotary_current: i32) -> i32 {
	
	if rotary_last > rotary_current {
		return -1;
	}

	if rotary_last < rotary_current {
		return 1;
	}		
	0
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_color() {
		let c = Color::new();
		assert_eq!(0, c.r);
		assert_eq!(0, c.g);
		assert_eq!(0, c.b);
	}

	#[test]
	fn test_new_position() {
		let p = Position::new();
		assert_eq!(0, p.x);
		assert_eq!(0, p.y);
	}

	#[test]
	fn test_translate_position_ok() {
		let mut pos = Position{ x: 4, y: 4};
		let delta = Position { x: 1, y: -1};
		pos.translate_in_matrix_bounds(delta);
		assert_eq!(Position{ x: 5, y: 3}, pos);
	}

	#[test]
	fn test_translate_position_nok() {
		let mut pos = Position{ x: 0, y: 7};
		let delta = Position { x: -1, y: 1};
		pos.translate_in_matrix_bounds(delta);
		assert_eq!(Position{ x: 0, y: 7}, pos);
	}
}