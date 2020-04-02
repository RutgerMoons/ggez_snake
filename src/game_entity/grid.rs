use crate::game_entity::{grid_position::GridPos,snake::Snake,types::Direction};
use super::HasPositions;

pub struct Grid {
  nb_rows: u8,
  nb_cols: u8,
  snake: Snake,
}

impl Grid {
  pub fn new(rows: u8, cols: u8) -> Self {
    let start_pos : GridPos = (rows / 4, cols / 4).into();
    Self {
      nb_rows: rows,
      nb_cols: cols,
      snake: Snake::new(start_pos),
    }
  }

  pub fn get_dimensions(&self) -> (u8, u8) {
    (self.nb_rows, self.nb_cols)
  }

  pub fn get_snake_positions(&self) -> Box<dyn Iterator<Item = GridPos>> {
    self.snake.get_positions()
  }

  pub fn update(&mut self) {
    self.snake.update();
  }

  pub fn change_direction(&mut self, new_dir: Direction) {
    self.snake.change_direction(new_dir);
  }
}