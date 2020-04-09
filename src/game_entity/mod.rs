pub mod snake;
pub mod types;
pub mod grid_position;
pub mod grid;
pub mod apple;
pub mod wall;

trait HasPositions {
  fn get_positions(&self) -> Box<dyn Iterator<Item = grid_position::GridPos>>; 
}
