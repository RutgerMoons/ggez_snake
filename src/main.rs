// based on https://github.com/ggez/ggez/blob/master/examples/04_snake.rs

// Imports
use ggez;
use rand;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use ggez::graphics::Color;

use std::time::{Duration, Instant};

// Constants
const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);
const SCREEN_SIZE: (f32, f32) = (
	GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
	GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

// enums
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
	fn from_key_code(key: KeyCode) -> Option<Direction> {
		match key {
			KeyCode::Up 	=> Some(Direction::Up),
			KeyCode::Down 	=> Some(Direction::Down),
			KeyCode::Left 	=> Some(Direction::Left),
			KeyCode::Right 	=> Some(Direction::Right),
			_ => None,
		}
	}

	fn inverse(&self) -> Self {
		match *self {
			Direction::Up 		=> Direction::Down,
			Direction::Down 	=> Direction::Up,
			Direction::Left 	=> Direction::Right,
			Direction::Right 	=> Direction::Left,
		}
	}
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GridPos {
	x: i16,
	y: i16,
}

impl GridPos {
	fn draw(self, _ctx: &mut Context, col: Color) -> GameResult<()> {
		let rect = graphics::Mesh::new_rectangle(
			_ctx,
			graphics::DrawMode::fill(),
			self.into(),
			col,
		)?;
		graphics::draw(_ctx, &rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
		Ok(())
	}
}

impl From<GridPos> for graphics::Rect {
	fn from(pos: GridPos) -> Self {
		graphics::Rect::new_i32(
			pos.x as i32 * GRID_CELL_SIZE.0 as i32,
			pos.y as i32 * GRID_CELL_SIZE.1 as i32,
			GRID_CELL_SIZE.0 as i32,
			GRID_CELL_SIZE.1 as i32,
		)
	}
}

impl From<(i16, i16)> for GridPos {
	fn from(pos: (i16, i16)) -> Self {
		GridPos { x: pos.0, y: pos.1 }
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Snake {
	head: GridPos,
	dir: Direction,
	vel: i16,
}

impl Snake {
	fn new(head_pos: GridPos) -> Self {
		Self {
			head: head_pos,
			dir: Direction::Right,
			vel: 1,
		}
	}

	fn update(&mut self) {
		self.move_snake();
	}

	fn draw(&self, _ctx: &mut Context, palette: & ColorPalette) -> GameResult<()> {
		// draw segments

		// draw head
		self.head.draw(_ctx, 
					   palette.SNAKE_HEAD)?;
		Ok(())
	}

	fn move_snake(&mut self) -> GameResult {
		let new_head: GridPos = match self.dir {
			Direction::Left		=> (self.head.x - self.vel, self.head.y),
			Direction::Right 	=> (self.head.x + self.vel, self.head.y),
			Direction::Up 		=> (self.head.x, self.head.y - self.vel),
			Direction::Down		=> (self.head.x, self.head.y + self.vel),
		}.into();
		self.head = new_head;
		Ok(())
	}

	fn change_direction(&mut self, new_dir: Direction) {
		if new_dir == self.dir.inverse() {
			return;
		}

		self.dir = new_dir
	}
}

struct GameState {
	palette: ColorPalette,
	snake: Snake,
	last_update: Instant,
}

impl GameState {
	fn new() -> Self {
		GameState {
			palette: ColorPalette::new(),
			snake: Snake::new(GridPos { x: 5, y: 5}),
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

		self.snake.draw(_ctx, & self.palette);
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
		if let Some(dir) = Direction::from_key_code(keyCode) {
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
