

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPos {
	pub x: u8,
	pub y: u8,
}

impl From<(u8, u8)> for GridPos {
	fn from(pos: (u8, u8)) -> Self {
		GridPos { x: pos.0, y: pos.1 }
	}
}
