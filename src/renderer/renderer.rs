use super::{grid_renderer,color_palette::ColorPalette};
use crate::game_entity::grid::Grid;
use ggez::{Context,GameResult,graphics};

pub struct Renderer {
  pub grid_renderer: grid_renderer::GridRenderer,
  palette: ColorPalette,
}

impl Renderer {
  pub fn new() -> Self {
    Self {
      grid_renderer: grid_renderer::GridRenderer::new(),
      palette: ColorPalette::new(),
    }
  }

	pub fn get_screen_size(&self, grid: &Grid) -> (f32, f32) {
    let cell_size = self.grid_renderer.get_cell_size();
    let grid_size = grid.get_dimensions();
    (grid_size.0 as f32 * cell_size.0 as f32,
     grid_size.1 as f32 * cell_size.1 as f32)
  }
}

pub trait Renderable {
  fn draw(&self, _ctx: &mut Context, grid: &Grid) -> GameResult;
}


impl Renderable for Renderer {
  fn draw(&self, _ctx: &mut Context, grid: &Grid) -> GameResult {
    graphics::clear(_ctx, self.palette.BG_COLOR);
    self.grid_renderer.draw(_ctx, grid, &self.palette);
		graphics::present(_ctx)
  }
}