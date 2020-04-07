
use super::types::Direction;
use super::grid_position::GridPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snake {
	pub head: GridPos,
	dir: Direction,
	new_dir: Direction,
	vel: u8,
}

impl Snake {
	pub fn new(head_pos: GridPos) -> Self {
		Self {
			head: head_pos,
			dir: Direction::Right,
			new_dir: Direction::Right,
			vel: 1,
		}
	}

	pub fn update(&mut self) {
		self.dir = self.new_dir;
		self.move_snake();
	}

	fn move_snake(&mut self) {
		let new_head: GridPos = match self.dir {
			Direction::Left		=> (self.head.x - self.vel, self.head.y),
			Direction::Right 	=> (self.head.x + self.vel, self.head.y),
			Direction::Up 		=> (self.head.x, self.head.y - self.vel),
			Direction::Down		=> (self.head.x, self.head.y + self.vel),
		}.into();
		self.head = new_head
	}

	pub fn change_direction(&mut self, new_dir: Direction) {
		if new_dir == self.dir.inverse() {
			return;
		}

		self.new_dir = new_dir
	}

  pub fn get_head_pos(self: &Self) -> GridPos {
    self.head
	}
	
	pub fn on_eat_apple(self: &mut Self) {

	}
}

impl super::HasPositions for Snake {
	fn get_positions(&self) -> Box<dyn Iterator<Item = GridPos>> {
		Box::new(vec!(self.head).into_iter())
	}
}