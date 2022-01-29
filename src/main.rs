extern crate minifb;

// Module includes
//#[path = "./constants.rs"]
mod constants;
mod button;

// Use statements
use minifb::*;
use constants::WIDTH;
use constants::HEIGHT;
use button::Button;

fn main() {

	// Main draw buffer
	let mut buffer: Vec<u32> = vec![0x00_00_FF_FF; WIDTH * HEIGHT];

	let mut window = Window::new(
		"Test - ESC to exit",
		WIDTH,
		HEIGHT,
		WindowOptions::default(),
	)
	.unwrap_or_else(|e| {
		panic!("{}", e);
	});

	let btn = Button::new("button.png",
		10,
		20,
		80,
		40  // button dimensions
	);

	// Passing buffer to draw on.
	btn.display(&mut buffer);

	// Limit to max ~60 fps update rate
	window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

	// Preventing repetitive clicks with this var
	let mut countdown: i8 = 0;

	while window.is_open() && !window.is_key_down(Key::Escape) {
		if let Some(mouse) = window.get_mouse_pos(MouseMode::Discard) {
			if countdown == 0 && window.get_mouse_down(MouseButton::Left) {
				let x_pos = mouse.0 as i32;
				let y_pos = mouse.1 as i32;

				if btn.test_region(x_pos, y_pos) {
					println!("Clicked!");
				}

				countdown = 20;
			} else {
				if countdown > 0 {
					countdown = countdown - 1;
				}
			}
		}

		// Unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, WIDTH, HEIGHT)
			.unwrap();
	}
}

