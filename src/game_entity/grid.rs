use crate::game_entity::{grid_position::GridPos,snake::Snake,types::Direction,apple::Apple,wall::Wall};
use super::HasPositions;

pub struct Grid {
  nb_rows: u8,
  nb_cols: u8,
  snake: Snake,
  apple: Apple,
  pub walls: Vec<Wall>,
}

impl Grid {
  pub fn new(rows: u8, cols: u8) -> Self {
    let start_pos : GridPos = (rows / 4, cols / 4).into();
    let mut new_grid = Self {
      nb_rows: rows,
      nb_cols: cols,
      snake: Snake::new(start_pos),
      apple: Apple::spawn((1,1).into()),
      walls: Vec::new(),
    };
    new_grid.walls = new_grid.get_walls();
    new_grid.apple = Apple::spawn(new_grid.get_random_free_position());
    new_grid
  }

  pub fn get_dimensions(&self) -> (u8, u8) {
    (self.nb_rows, self.nb_cols)
  }

  fn get_walls(&self) -> Vec<Wall> {
    let mut walls : Vec<Wall> = Vec::new();
    for i in 0..self.nb_rows {
      walls.push(Wall { pos: GridPos {x: i, y: 0}});
      walls.push(Wall { pos: GridPos {x: i, y: self.nb_cols as u8 - 1}});
    }

    for i in 1..self.nb_cols - 1 {
      walls.push(Wall { pos: GridPos {x: 0, y: i}});
      walls.push(Wall { pos: GridPos {x: self.nb_rows as u8 - 1, y: i}});
    }
    walls
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
    // incorrect for the tail, but nevermind ATM
    let free = self.get_free_positions();
    self.snake.update();
    let snake_head_pos = self.snake.get_head_pos();

    if !free.contains(&snake_head_pos) {
      panic!("Bump");
    }

    if snake_head_pos == self.apple.pos {
      self.snake.on_eat_apple();
      self.apple = self.apple.on_eat();
      self.on_eat_apple();
    }
  }

  pub fn change_direction(&mut self, new_dir: Direction) {
    self.snake.change_direction(new_dir);
  }

  fn get_free_positions(self: &Self) -> Vec<GridPos> {
    let mut used_pos : Vec<GridPos> = self.get_snake_positions().collect();
    let wall_pos : Vec<GridPos> = self.walls.iter()
                                            .map(|w| w.pos)
                                            .collect::<Vec<GridPos>>();
                    
    used_pos.extend(wall_pos);
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