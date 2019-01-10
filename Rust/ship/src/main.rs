extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::rect::Rect;

const CELL_SIZE: u32 = 25;
const FIELD_START_X: i32 = 50;
const FIELD_START_Y: i32 = 50;

struct Game<'a> {
	canvas: &'a mut Canvas<Window>,
	me: Field<'a>,
	enemy: Field<'a>,
    cell_size: u32,
	fields_start_xy: [i32; 2],
}

impl<'a> Game<'a> {
	pub fn new(
		canvas: &mut Canvas<Window>,
		cell_size: u32,
		fields_start_xy: [i32; 2],
	) -> Game {
		Game {
			canvas,
            me: Field::new(canvas, false),
			enemy: Field::new(canvas, true),
			cell_size,
			fields_start_xy,
		}
	}

	pub fn draw(self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();

        self.me.draw();
		self.enemy.draw();
	}
}

enum Ship {Alive, Dead}

#[derive(Clone, Copy)]
enum Cell {Ship, Empty, Dot}

struct Field<'a> {
	canvas: &'a mut Canvas<Window>,
	is_enemy: bool,
	cells: &'a [Cell; 100],
}

impl<'a> Field<'a> {
	pub fn new(canvas: &mut Canvas<Window>, is_enemy: bool) -> Field {
        Field{
			canvas,
			is_enemy,
			cells: &[Cell::Empty; 100],
		}
	}

    pub fn draw(self) {
		self.canvas.set_draw_color(Color::RGB(255, 210, 201));

        for row in 0..11 {
            let x = FIELD_START_X + (row * CELL_SIZE) as i32;
			let _ = self.canvas.fill_rect(Rect::new(FIELD_START_X, x as i32, CELL_SIZE * 10, 1));
		}

		for col in 0..11 {
			let y = FIELD_START_Y + (col * CELL_SIZE) as i32;
			let _ = self.canvas.fill_rect(Rect::new(y as i32, FIELD_START_Y, 1, CELL_SIZE * 10));
		}

		self.canvas.present();
	}
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("Ship", 700, 350).build().unwrap();

	let mut canvas: Canvas<Window> = window.into_canvas()
		.present_vsync()
		.build()
        .unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();
    let game = Game::new(&mut canvas, 25, [50, 50]);

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}

		game.draw();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
