use super::grid_position::GridPos;

#[derive(PartialEq)]
pub enum FoodState {
  Fresh,
  Eaten,
}

pub struct Apple {
  pub pos: GridPos,
  pub state: FoodState,
}

impl Apple {
  pub fn spawn(pos: GridPos) -> Self {
    Self { pos: pos, state: FoodState::Fresh }
  }

  pub fn on_eat(self: &Self) -> Self {
    match self.state {
      FoodState::Fresh => Self { state: FoodState::Eaten, pos: self.pos },
      FoodState::Eaten => panic!("Can't eat apple twice"),
    }
  }
}