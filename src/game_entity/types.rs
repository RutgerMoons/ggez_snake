use ggez::event::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
	pub fn from_key_code(key: KeyCode) -> Option<Direction> {
		match key {
			KeyCode::Up 	=> Some(Direction::Up),
			KeyCode::Down 	=> Some(Direction::Down),
			KeyCode::Left 	=> Some(Direction::Left),
			KeyCode::Right 	=> Some(Direction::Right),
			_ => None,
		}
	}

	pub fn inverse(&self) -> Self {
		match *self {
			Direction::Up 		=> Direction::Down,
			Direction::Down 	=> Direction::Up,
			Direction::Left 	=> Direction::Right,
			Direction::Right 	=> Direction::Left,
		}
	}
}