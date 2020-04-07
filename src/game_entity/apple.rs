use super::grid_position::GridPos;

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

  pub fn onEat(fresh: & Apple) -> Self {
    match fresh.state {
      FoodState::Fresh => Self { state: FoodState::Eaten, pos: fresh.pos },
      FoodState::Eaten => panic!("Can't eat apple twice"),
    }
  }
}