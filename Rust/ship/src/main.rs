extern crate sdl2;
extern crate rand;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use sdl2::video::{Window, WindowContext};
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};

#[derive(Clone, Copy, PartialEq)]
enum Cell {Alive, Injured, Dead, Empty, Miss}

#[derive(Copy, Clone, PartialEq)]
enum State {Me, Enemy, Over}

struct Game<'a, 'b> {
	canvas: &'a mut Canvas<Window>,
	tc: &'b TextureCreator<WindowContext>,
	me: Vec<Cell>,
	enemy: Vec<Cell>,
	cell_size: u32,
	field_size: u32,
	xy: [i32; 2],
	fields_space_px: i32,
    state: State,
}

impl<'a, 'b> Game<'a, 'b> {
	pub fn new(
		canvas: &'a mut Canvas<Window>,
		tc: &'b TextureCreator<WindowContext>,
		cell_size: u32,
		xy: [i32; 2],
	) -> Game<'a, 'b> {
		let mut me = vec![Cell::Empty; 100];
		let mut enemy = vec![Cell::Empty; 100];

		Game::gen_ships(&mut me);
		Game::gen_ships(&mut enemy);

		Game {
			canvas,
            tc,
			me,
			enemy,
			cell_size,
			field_size: cell_size * 10 + 1,
			xy,
			fields_space_px: 50,
            state: State::Me,
		}
	}

	pub fn get_state(&self) -> State {self.state}
	pub fn set_state(&mut self, state: State) {self.state = state;}

	fn gen_ships(cells: &mut Vec<Cell>) {
		enum Dir {Hor, Ver}

		let mut rng = rand::thread_rng();

        for i in &[4, 3, 3, 2, 2, 2, 1, 1, 1, 1] {
			loop {
				let dir = match rng.gen_range(0, 2) {
					0 => Dir::Hor,
					_ => Dir::Ver,
				};

				let shift = match dir {
					Dir::Hor => 1,
					Dir::Ver => 10,
				};
				let upper = match dir {
					Dir::Hor => (10 - i + 1, 10),
					Dir::Ver => (10, 10 - i + 1),
				};

				let row = rng.gen_range(0, upper.0);
				let col = rng.gen_range(0, upper.1);

				let mut is_ok = true;
				let mut coords = vec![];
				'outer: for k in 0..*i {
					let t = row + col * 10 + k * shift;
					let check_cells: [i16; 9] = [
						t, t + 1, t - 1,
						t + 11, t + 10, t + 9,
						t - 11, t - 10, t - 9,
					];
					for c in &check_cells {
                        if *c >= 0 && *c < 100 && cells[*c as usize] != Cell::Empty {
							is_ok = false;
							break 'outer;
						}
					}

					coords.push(t);
				}

				if is_ok {
					for t in coords {
                        cells[t as usize] = Cell::Alive;
					}
					break;
				}
			}
		}
	}

	pub fn draw(&mut self) {
		if self.state == State::Enemy {
			self.enemy_turn();
		}

		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();

		let me = self.draw_field(false);
		let enemy = self.draw_field(true);

		self.canvas.copy(
			&me,
			None,
			Rect::new(self.xy[0], self.xy[1], self.field_size, self.field_size),
		).unwrap();
		self.canvas.copy(
			&enemy,
			None,
			Rect::new(
				self.xy[0] + (self.field_size as i32) + self.fields_space_px,
				self.xy[1],
				self.field_size,
				self.field_size,
			),
		).unwrap();

		self.canvas.present();
	}

	fn enemy_turn(&mut self) {
        loop {
            let mut rng = rand::thread_rng();
            let cell = rng.gen_range(0, 100);
			match self.me[cell] {
				Cell::Alive => {
					::std::thread::sleep(Duration::new(1, 0));
					self.me[cell] = Cell::Injured;
				},
				Cell::Empty => {
					self.me[cell] = Cell::Miss;
					self.set_state(State::Me);
					break;
                }
				_ => {},
			}
		}
	}

	pub fn shot(&mut self, x: i32, y: i32) {
		let cell_x = (x - (self.xy[0] + (self.field_size as i32) + self.fields_space_px)) / self.cell_size as i32;
		let cell_y = (y - self.xy[1]) / self.cell_size as i32;
		if cell_x < 0 || cell_y < 0 || cell_x > 9 || cell_y > 9 {
			return;
		}

		let cell = (cell_x + cell_y * 10) as usize;
        match self.enemy[cell] {
            Cell::Alive => self.enemy[cell] = Cell::Injured,
			Cell::Injured | Cell::Dead => {},
			_ => {
				self.enemy[cell] = Cell::Miss;
				self.set_state(State::Enemy);
			},
		}
	}

	fn draw_field(&mut self, is_enemy: bool) -> Texture<'b> {
		let field_size = &self.field_size;
		let mut t: Texture = self.tc.create_texture_target(None, self.field_size, self.field_size).unwrap();

		let cell_size = &self.cell_size;
        let cells = match is_enemy {
			true => &self.enemy,
			false => &self.me,
		};

		self.canvas.with_texture_canvas(&mut t, |tc| {
			tc.set_draw_color(Color::RGB(0, 0, 0));
			tc.clear();

			tc.set_draw_color(Color::RGB(99, 159, 255));
			for row in 0..11 {
				tc.fill_rect(Rect::new(0, (row * cell_size) as i32, cell_size * 10, 1));
				tc.fill_rect(Rect::new((row * cell_size) as i32, 0, 1, cell_size * 10));
			}

			for (i, cell) in cells.iter().enumerate() {
				let x = (i as u32 % 10 * cell_size) as i32 + 1;
				let y = (i as u32 / 10 * cell_size) as i32 + 1;

				match cell {
					Cell::Miss => {
						tc.set_draw_color(Color::RGB(255, 255, 255));
						tc.draw_rect(Rect::new(
							x + (cell_size / 2) as i32,
							y + (cell_size / 2) as i32,
							2,
							2,
						));
					},
					Cell::Alive if !is_enemy => {
						tc.set_draw_color(Color::RGB(225, 225, 225));
						tc.fill_rect(Rect::new(x, y, cell_size - 1, cell_size - 1));
					},
					Cell::Injured => {
						tc.set_draw_color(Color::RGB(0, 155, 155));
						tc.fill_rect(Rect::new(x, y, cell_size - 1, cell_size - 1));
					},
					Cell::Dead => {
						tc.set_draw_color(Color::RGB(215, 215, 0));
						tc.fill_rect(Rect::new(x, y, cell_size - 1, cell_size - 1));
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
				Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                    game.shot(x, y);
				},
				_ => {}
			}
		}

		if game.get_state() == State::Over {
			println!("Over");
			break;
		}

		game.draw();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
