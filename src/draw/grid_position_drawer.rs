use crate::game_entity::{grid_position::GridPos,snake::Snake};
use ggez::{Context,graphics};
use ggez::graphics::Color;


const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = (
	GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
	GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

pub trait Drawable {
  fn draw(&self, _ctx: &mut Context, col: Color);
}

impl Drawable for GridPos {
  fn draw(&self, _ctx: &mut Context, col: Color) {
		let rect = graphics::Mesh::new_rectangle(
			_ctx,
			graphics::DrawMode::fill(),
			self.into(),
			col,
    );
    if let Ok(r) = rect {
      graphics::draw(_ctx, &r, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
    }
	}
}

impl Drawable for Snake {
	fn draw(&self, _ctx: &mut Context, col: Color) {
		// draw segments

		// draw head
		self.head.draw(_ctx, col);
	}
}


impl From<&GridPos> for graphics::Rect {
  fn from(pos: &GridPos) -> Self {
    graphics::Rect::new_i32(
      pos.x as i32 * GRID_CELL_SIZE.0 as i32,
      pos.y as i32 * GRID_CELL_SIZE.1 as i32,
      GRID_CELL_SIZE.0 as i32,
      GRID_CELL_SIZE.1 as i32,
    )
  }
}
