extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::{Window, WindowContext};
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};

#[derive(Clone, Copy)]
enum Cell {Alive, Injured, Dead, Empty, Dot}

struct Game<'a, 'b> {
	canvas: &'a mut Canvas<Window>,
	tc: &'b TextureCreator<WindowContext>,
	me: Vec<Cell>,
	enemy: Vec<Cell>,
	cell_size: u32,
	xy: [i32; 2],
}

impl<'a, 'b> Game<'a, 'b> {
	pub fn new(
		canvas: &'a mut Canvas<Window>,
		tc: &'b TextureCreator<WindowContext>,
		cell_size: u32,
		xy: [i32; 2],
	) -> Game<'a, 'b> {
		let me = vec![Cell::Empty; 100];
		let enemy = vec![Cell::Dot; 100];
		Game {
			canvas,
            tc,
			me,
			enemy,
			cell_size,
			xy,
		}
	}

	pub fn draw(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();

		self.me[15] = Cell::Dead;
		self.me[25] = Cell::Dead;
		self.me[35] = Cell::Dead;

		let me = self.draw_field(false);
		let enemy = self.draw_field(true);

		let field_size = self.cell_size * 10 + 1;
		self.canvas.copy(&me, None, Rect::new(self.xy[0], self.xy[1], field_size, field_size)).unwrap();
		self.canvas.copy(&enemy, None, Rect::new(self.xy[0] + (field_size as i32) + 50, self.xy[1], field_size, field_size)).unwrap();

		self.canvas.present();
	}

	fn draw_field(&mut self, is_enemy: bool) -> Texture<'b> {
		let field_size = self.cell_size * 10 + 1;
		let mut t: Texture = self.tc.create_texture_target(None, field_size, field_size).unwrap();

		let cell_size = &self.cell_size;
        let cells = match is_enemy {
			true => &self.enemy,
			false => &self.me,
		};

		self.canvas.with_texture_canvas(&mut t, |tc| {
			tc.set_draw_color(Color::RGB(0, 0, 0));
			tc.clear();

			tc.set_draw_color(Color::RGB(200, 200, 200));
			for row in 0..11 {
				tc.fill_rect(Rect::new(0, (row * cell_size) as i32, cell_size * 10, 1));
				tc.fill_rect(Rect::new((row * cell_size) as i32, 0, 1, cell_size * 10));
			}

			for (i, cell) in cells.iter().enumerate() {
				let x = (i as u32 % 10 * cell_size) as i32;
				let y = (i as u32 / 10 * cell_size) as i32;

				match cell {
					Cell::Dot => {
						tc.set_draw_color(Color::RGB(255, 255, 255));
						tc.draw_rect(Rect::new(
							x + (cell_size / 2) as i32,
							y + (cell_size / 2) as i32,
							2,
							2,
						));
					},
					Cell::Alive => {
						tc.set_draw_color(Color::RGB(255, 255, 255));
						tc.fill_rect(Rect::new(x, y, cell_size + 0, cell_size + 0));
					},
					Cell::Injured => {
						tc.set_draw_color(Color::RGB(0, 155, 155));
						tc.fill_rect(Rect::new(x, y, cell_size + 0, cell_size + 0));
					},
					Cell::Dead => {
						tc.set_draw_color(Color::RGB(215, 215, 0));
						tc.fill_rect(Rect::new(x, y, cell_size + 0, cell_size + 0));
					},
                    _ => {},
				}
			}
		});

		t
	}
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("Ship", 660, 350).build().unwrap();

	let mut canvas: Canvas<Window> = window.into_canvas()
		.present_vsync()
		.build()
        .unwrap();

    let texture_creator : TextureCreator<_> = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game = Game::new(&mut canvas, &texture_creator, 25, [50, 50]);

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
