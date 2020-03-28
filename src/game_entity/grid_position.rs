

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPos {
	pub x: i16,
	pub y: i16,
}

impl From<(i16, i16)> for GridPos {
	fn from(pos: (i16, i16)) -> Self {
		GridPos { x: pos.0, y: pos.1 }
	}
}
