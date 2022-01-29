/*
  Button base struct/impl to use with blit graphic library
  and image file as visual appearance.
*/

extern crate blit;
extern crate image;

#[path = "./constants.rs"]
mod constants;

use constants::WIDTH;
use constants::MASK_COLOR;

use blit::*;
use image::*;
use rusttype::{Point, Rect};

pub struct Button {
	pub img: DynamicImage,
	pub rect: Rect<i32>
}
impl Button {

	// Constructor
	pub fn new(
        img: &str,
		left: i32,
		top: i32,
		right: i32,
		bottom: i32
	) -> Self {
		return Button {
			img : image::open(img).unwrap(),
			rect : Rect {
				min: Point { x: left,  y: top },
				max: Point { x: right, y: bottom }
			}
		};
    }
        
    /*
	  Display function to redraw a button. 
	  Run each time to change its look or position
	*/
	pub fn display(&self, buffer: &mut Vec<u32>) {
		let rgb = self.img.as_rgb8().unwrap();
		rgb.blit(buffer, WIDTH, (self.rect.min.x, self.rect.min.y), MASK_COLOR);	
    }

	/*
	  Check click region in window by passing mouse position.
	*/
	pub fn check_region(&self, x_pos: i32, y_pos: i32) -> bool {
		x_pos >= self.rect.min.x && x_pos <= self.rect.max.x && 
		y_pos >= self.rect.min.y && y_pos <= self.rect.max.y
	}
}

