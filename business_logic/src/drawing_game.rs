use crate::cursors::Cursor;
use crate::{helpers::*, UserInput};

pub const MATRIX_SIZE 		 : usize = 8;
pub const CMD_LEN     		 : usize = 67;
const CMD_START_MARKER   : u8 = 0x10;
const CMD_END_MARKER     : u8 = 0x20;


pub struct DrawingGame<'a> {
	pub cursor_position: Position,
	pub color_matrix : [[Color; MATRIX_SIZE]; MATRIX_SIZE],
	pub current_cursor : &'a dyn Cursor,
	pub cursor_matrix: [[u8; MATRIX_SIZE]; MATRIX_SIZE],
	pub output_matrix: [[Color; MATRIX_SIZE]; MATRIX_SIZE],
	last_user_input: UserInput,
	pub mode: GameMode 
}

impl<'a> DrawingGame<'a> {
	pub const fn new(cursor: &'a dyn Cursor) -> DrawingGame<'a> {
		DrawingGame {
			cursor_position: Position::new(), 
			output_matrix: [[Color::new(); MATRIX_SIZE]; MATRIX_SIZE], 
			color_matrix: [[Color::new(); MATRIX_SIZE]; MATRIX_SIZE], 
			cursor_matrix: [[0; MATRIX_SIZE]; MATRIX_SIZE], 
			current_cursor: cursor,
			last_user_input: UserInput::new(),
			mode: GameMode::Navigating }
	}

	fn set_new_cursor_position(&mut self, input: UserInput) {
		self.cursor_position.x = input.left_rotary_val;
		self.cursor_position.y = input.right_rotary_val;
	}

	pub fn user_input(&mut self, input: UserInput) {
		match self.mode {
			GameMode::Navigating => {
				self.set_new_cursor_position(input);
				self.gen_cursor_matrix();			
			}

			GameMode::Drawing=> {
				
			}
		}
		self.render();
	}

	fn gen_cursor_matrix(&mut self) {
		self.cursor_matrix = [[0; MATRIX_SIZE]; MATRIX_SIZE];
		let (cursor_shape, num_cursor_points) = self.current_cursor.get_cursor();
		for n_pixel in 0 .. num_cursor_points {
			let mut cursor_pixel = cursor_shape[n_pixel];
			if cursor_pixel.can_translate_inside_matrix_bounds(self.cursor_position) {
				cursor_pixel.translate(self.cursor_position);
				self.cursor_matrix[cursor_pixel.y as usize][cursor_pixel.x as usize] = 255;
			}
		}
	}

	fn render(&mut self) {
		for y in 0 .. MATRIX_SIZE {
			for x in 0 .. MATRIX_SIZE {
				self.output_matrix[y][x].r = self.color_matrix[y][x].r | self.cursor_matrix[y][x];
				self.output_matrix[y][x].g = self.color_matrix[y][x].g | self.cursor_matrix[y][x];
				self.output_matrix[y][x].b = self.color_matrix[y][x].b | self.cursor_matrix[y][x];
			}
		}
	}

	pub fn gen_set_color_cmd(&self, cmd_buf: &mut [u8]){
		for i in 0 .. 3 {
			cmd_buf[i*CMD_LEN] = CMD_START_MARKER;
			cmd_buf[i*CMD_LEN + 1] = i as u8;
			cmd_buf[(i+1)*CMD_LEN-1] = CMD_END_MARKER;		
		}
		for y in 0 .. MATRIX_SIZE {
			for x in 0 .. MATRIX_SIZE {
				cmd_buf[x+y*MATRIX_SIZE + 2 + 0*CMD_LEN] = self.output_matrix[y][x].r;
				cmd_buf[x+y*MATRIX_SIZE + 2 + 1*CMD_LEN] = self.output_matrix[y][x].g;
				cmd_buf[x+y*MATRIX_SIZE + 2 + 2*CMD_LEN] = self.output_matrix[y][x].b;
			}
		}

	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::cursors::SingleCursor;

	#[test]
	fn test_new_drawing_game() {
		let cursor = SingleCursor::new();
		let drawing_game = DrawingGame::new(&cursor);
		assert_eq!(Position::new(), drawing_game.cursor_position );
		for y in 0 .. MATRIX_SIZE {
			for x in 0 .. MATRIX_SIZE {
				assert_eq!(Color::new(), drawing_game.color_matrix[y][x]);
			}
		}
	}

	#[test]
	fn test_gen_color_cmd() {
		let cursor = SingleCursor::new();
		let drawing_game = DrawingGame::new(&cursor);
		let mut cmd: [u8; 3 * CMD_LEN] = [0; 3 * CMD_LEN];
		
		drawing_game.gen_set_color_cmd(&mut cmd);
		assert_eq!(CMD_START_MARKER, cmd[0]);
		assert_eq!(CMD_START_MARKER, cmd[CMD_LEN]);
		assert_eq!(CMD_START_MARKER, cmd[2*CMD_LEN]);

		assert_eq!(0x0, cmd[1]);
		assert_eq!(0x1, cmd[CMD_LEN + 1]);
		assert_eq!(0x2, cmd[2*CMD_LEN + 1]);
		
		assert_eq!(CMD_END_MARKER, cmd[CMD_LEN - 1]);
		assert_eq!(CMD_END_MARKER, cmd[2* CMD_LEN - 1]);
		assert_eq!(CMD_END_MARKER, cmd[3* CMD_LEN - 1]);
	}
}