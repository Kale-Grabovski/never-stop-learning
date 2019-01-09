extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::rect::Rect;

const SIZE: u32 = 25;
const LEN: u32 = 10;

struct Field<'a> {
	canvas: &'a mut Canvas<Window>
}

impl<'a> Field<'a> {
	pub fn new(canvas: &mut Canvas<Window>) -> Field {
        Field{canvas}
	}

    pub fn draw(self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();

		let start_x: i32 = 50;
		let start_y: i32 = 50;

		self.canvas.set_draw_color(Color::RGB(255, 210, 201));

        for row in 0..LEN + 1 {
            let x = start_x + (row * SIZE) as i32;
			let _ = self.canvas.fill_rect(Rect::new(start_x, x as i32, SIZE * LEN, 1));
		}

		for col in 0..LEN + 1 {
			let y = start_y + (col * SIZE) as i32;
			let _ = self.canvas.fill_rect(Rect::new(y as i32, start_y, 1, SIZE * LEN));
		}

		self.canvas.present();
	}
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("Ship", 800, 600).build().unwrap();

	let mut canvas: Canvas<Window> = window.into_canvas()
		.present_vsync()
		.build()
        .unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();
    let field = Field::new(&mut canvas);
	field.draw();

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

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
