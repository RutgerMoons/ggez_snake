use crate::game_entity::{grid_position::GridPos,snake::Snake,grid::Grid};
use super::renderer::Renderable;
use super::color_palette::ColorPalette;
use ggez::{Context,graphics};
use ggez::graphics::Color;

pub struct GridRenderer {
  grid_cell_size: (u8, u8),
}

impl GridRenderer {
  pub fn new() -> Self {
    Self {
      grid_cell_size: (32, 32),
    }
  }

  pub fn get_cell_size(&self) -> (u8, u8) {
    self.grid_cell_size
  }
}

impl GridRenderer {
  pub fn draw(&self, _ctx: &mut Context, grid: &Grid, palette: &ColorPalette) {
    self.draw_snake(_ctx, grid, palette.SNAKE_HEAD, palette.SNAKE_BODY);  
  }

  fn draw_snake(&self, _ctx: &mut Context, grid: &Grid, head_color: Color, body_color: Color) {
    let positions = grid.get_snake_positions();
    for pos in positions {
      pos.draw(_ctx, head_color, self.grid_cell_size);
    }
  }
}

pub trait Drawable {
  fn draw(&self, _ctx: &mut Context, col: Color, cell_size: (u8, u8));
}

impl Drawable for GridPos {
  fn draw(&self, _ctx: &mut Context, col: Color, cell_size: (u8, u8)) {
		let rect = graphics::Mesh::new_rectangle(
			_ctx,
			graphics::DrawMode::fill(),
        graphics::Rect::new_i32(
          self.x as i32 * cell_size.0 as i32,
          self.y as i32 * cell_size.1 as i32,
          cell_size.0 as i32,
          cell_size.1 as i32,
      ),
			col,
    );
    if let Ok(r) = rect {
      graphics::draw(_ctx, &r, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
    }
	}
}
