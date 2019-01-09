extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::rect::Rect;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window("Ship", 800, 600).build().unwrap();

	let mut canvas: Canvas<Window> = window.into_canvas()
		.present_vsync()
		.build().unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop {
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();

		canvas.set_draw_color(Color::RGB(255, 210, 0));
		let _ = canvas.fill_rect(Rect::new(10, 10, 780, 580));

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}
		// The rest of the game loop goes here...

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
