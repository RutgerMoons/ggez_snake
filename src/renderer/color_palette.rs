use ggez::graphics::Color;

pub struct ColorPalette {
	pub BG_COLOR: Color,
	pub SNAKE_HEAD: Color,
	pub SNAKE_BODY: Color,
	pub APPLE_COLOR: Color,
}

impl ColorPalette {
	pub fn new() -> Self {
		Self {
			BG_COLOR : Color::from_rgb(30, 171, 1),
			SNAKE_HEAD : Color::from_rgb(247, 167, 27),
			SNAKE_BODY : Color::from_rgb(255, 119, 51),
			APPLE_COLOR: Color::from_rgb(200, 12, 12),
		}
	}
}
