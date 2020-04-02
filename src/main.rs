// based on https://github.com/ggez/ggez/blob/master/examples/04_snake.rs

// Imports
use ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

use std::time::{Duration, Instant};

// mod
mod game_entity;
use crate::game_entity::{grid,types};
mod renderer;
use crate::renderer::renderer::{Renderable, Renderer};


// Constants

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;


/*
const SCREEN_SIZE: (f32, f32) = (
	GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
	GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
*/
// structs
struct GameState {
	grid: grid::Grid,
	renderer: Renderer,
	last_update: Instant,
}

impl GameState {
	fn new() -> Self {
		let g = grid::Grid::new(30, 20);
		GameState {
			grid: g,
			renderer: Renderer::new(),
			last_update : Instant::now(),
		}
	}

	fn get_screen_size(&self) -> (f32, f32) {
		self.renderer.get_screen_size(& self.grid)
	}
}

impl event::EventHandler for GameState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		let n = Instant::now();
		if n - self.last_update <= Duration::from_millis(MILLIS_PER_UPDATE) {
			return Ok(());
		}
		
		self.last_update = n;
		self.grid.update();
		Ok(())
	}

	fn draw(&mut self, _ctx: &mut Context) -> GameResult {
		self.renderer.draw(_ctx, &self.grid);
		//ggez::timer::yield_now();
		
		Ok(())
	}

	fn key_down_event(&mut self, _ctx: &mut Context, keyCode: KeyCode, _keyMods: KeyMods, _repeat: bool) {
		if let Some(dir) = types::Direction::from_key_code(keyCode) {
			self.grid.change_direction(dir);
		}
	}
}

// functions
fn main() -> GameResult {
	let mut state = GameState::new();
	let dims = state.get_screen_size();
	let (ctx, events_loop) = &mut ggez::ContextBuilder::new("snake", "Me")
		.window_setup(ggez::conf::WindowSetup::default().title("Snake clone"))
		.window_mode(ggez::conf::WindowMode::default().dimensions(dims.0, dims.1))
		.build()?;

	event::run(ctx, events_loop, &mut state)
}
