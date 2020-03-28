// based on https://github.com/ggez/ggez/blob/master/examples/04_snake.rs

// Imports
use ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use ggez::graphics::Color;

use std::time::{Duration, Instant};

// mod
mod game_entity;
use crate::game_entity::{snake,types};
mod draw;
use crate::draw::grid_position_drawer::Drawable;


// Constants

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;


const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = (
	GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
	GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
// structs
struct ColorPalette {
	BG_COLOR: Color,
	SNAKE_HEAD: Color,
}

impl ColorPalette {

	fn new() ->Self {
		Self {
			BG_COLOR : Color::from_rgb(30, 171, 1),
			SNAKE_HEAD : Color::from_rgb(247, 167, 27),
		}
	}
}

struct GameState {
	palette: ColorPalette,
	snake: snake::Snake,
	last_update: Instant,
}

impl GameState {
	fn new() -> Self {
		GameState {
			palette: ColorPalette::new(),
			snake: snake::Snake::default(),
			last_update : Instant::now(),
		}
	}
}

impl event::EventHandler for GameState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		let n = Instant::now();
		if n - self.last_update <= Duration::from_millis(MILLIS_PER_UPDATE) {
			return Ok(());
		}
		
		self.last_update = n;
		self.snake.update();
		Ok(())
	}

	fn draw(&mut self, _ctx: &mut Context) -> GameResult {
		graphics::clear(_ctx, self.palette.BG_COLOR);

		self.snake.draw(_ctx, self.palette.SNAKE_HEAD);
		/*
		let rect = graphics::Mesh::new_rectangle(
			_ctx,
			graphics::DrawMode::fill(),
			self.
		)
		*/
		graphics::present(_ctx)?;
		//ggez::timer::yield_now();
		Ok(())
	}

	fn key_down_event(&mut self, _ctx: &mut Context, keyCode: KeyCode, _keyMods: KeyMods, _repeat: bool) {
		if let Some(dir) = types::Direction::from_key_code(keyCode) {
			self.snake.change_direction(dir);
		}
	}
}

// functions
fn main() -> GameResult {
	let (ctx, events_loop) = &mut ggez::ContextBuilder::new("snake", "Me")
		.window_setup(ggez::conf::WindowSetup::default().title("Snake clone"))
		.window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
		.build()?;

	let mut state = GameState::new();

	event::run(ctx, events_loop, &mut state)
}
