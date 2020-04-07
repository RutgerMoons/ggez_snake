use crate::game_entity::{grid_position::GridPos,snake::Snake,types::Direction,apple::Apple};
use super::HasPositions;

pub struct Grid {
  nb_rows: u8,
  nb_cols: u8,
  snake: Snake,
  apple: Apple,
}

impl Grid {
  pub fn new(rows: u8, cols: u8) -> Self {
    let start_pos : GridPos = (rows / 4, cols / 4).into();
    let mut new_grid = Self {
      nb_rows: rows,
      nb_cols: cols,
      snake: Snake::new(start_pos),
      apple: Apple::spawn((1,1).into())
    };
    new_grid.apple = Apple::spawn(new_grid.get_random_free_position());
    new_grid
  }

  pub fn get_dimensions(&self) -> (u8, u8) {
    (self.nb_rows, self.nb_cols)
  }

  pub fn get_snake_positions(&self) -> Box<dyn Iterator<Item = GridPos>> {
    self.snake.get_positions()
  }

  pub fn get_apple(&self) -> &Apple {
    &self.apple
  }

  pub fn on_eat_apple(&mut self) {
    self.spawn_apple();
  }

  fn get_random_free_position(&self) -> GridPos {
    let free = self.get_free_positions();
    let r = rand::random::<usize>() % free.len();
    free[r]
  }

  fn spawn_apple(&mut self) {
    let new_pos = self.get_random_free_position();
    self.apple = Apple::spawn(new_pos);
  }

  pub fn update(&mut self) {
    self.snake.update();
    if self.snake.get_head_pos() == self.apple.pos {
      self.snake.on_eat_apple();
      self.apple = self.apple.on_eat();
      self.on_eat_apple();
    }
  }

  pub fn change_direction(&mut self, new_dir: Direction) {
    self.snake.change_direction(new_dir);
  }

  fn get_free_positions(self: &Self) -> Vec<GridPos> {
    let used_pos : Vec<GridPos> = self.get_snake_positions().collect();
    let mut free_pos : Vec<GridPos> = Vec::with_capacity(self.nb_rows as usize * self.nb_cols as usize);
    for i in 0..self.nb_rows {
      for j in 0..self.nb_cols {
        let new_pos = GridPos { x:i, y:j };
        if ! used_pos.contains(&new_pos) {
          free_pos.push(new_pos);
        }
      }
    }
    free_pos
  }
}