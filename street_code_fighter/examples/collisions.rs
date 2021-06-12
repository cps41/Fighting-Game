extern crate street_code_fighter;

use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use street_code_fighter::SDLCore;
use street_code_fighter::Demo;

const TITLE: &str = "SDL08 Rect Collisions";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

fn check_collision(a: &CollisionObject, b: &CollisionObject) -> bool {
	let mut res = false;
	if let CollisionObjectType::Character = a.obj_type {
			if let CollisionObjectType::Character = b.obj_type {
				return res;
			}
			
	else {
		if a.rect.bottom() < b.rect.top()
			|| a.rect.top() > b.rect.bottom()
			|| a.rect.right() < b.rect.left()
			|| a.rect.left() > b.rect.right()
		{
			res = false;
		}
		else {
			res = true;
		}
	}
}
	return res;
}

fn resist(vel: i32, deltav: i32) -> i32 {
	if deltav == 0 {
		if vel > 0 {
			-1
		}
		else if vel < 0 {
			1
		}
		else {
			deltav
		}
	}
	else {
		deltav
	}
}

pub struct SDL08 {
	core: SDLCore,
}

enum CollisionObjectType {
    Character,
    Hazard,
    Platform,
    Wall
}

pub struct CollisionObject {
    obj_type: CollisionObjectType,
    rect: Rect,
}

impl CollisionObject {
    fn new(obj_type: CollisionObjectType, x: i32, y: i32, width: u32, height: u32) -> CollisionObject {
        let rect = Rect::new(x, y, width, height);
        CollisionObject {
            obj_type,
            rect,
        }
    }
}

impl Demo for SDL08 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL08{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let w = 25;

		let static_box = CollisionObject::new(CollisionObjectType::Hazard, (CAM_W/2 + 2*w) as i32, (CAM_H/2 - w/2) as i32, w, w);

		let x_pos = (CAM_W/2 - w/2) as i32;
		let y_pos = (CAM_H/2 - w/2) as i32;		
		let mut player_box = CollisionObject::new(CollisionObjectType::Character, x_pos, y_pos, w, w);
		let mut player_box_2 = CollisionObject::new(CollisionObjectType::Character, x_pos-30, y_pos-30, w, w);

		let mut x_vel = 0;
		let mut y_vel = 0;

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

			let mut x_deltav = 0;
			let mut y_deltav = 0;
			if keystate.contains(&Keycode::W) {
				y_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::A) {
				x_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::S) {
				y_deltav += ACCEL_RATE;
			}
			if keystate.contains(&Keycode::D) {
				x_deltav += ACCEL_RATE;
			}

			// Slow down to 0 vel if no input and non-zero velocity
			x_deltav = resist(x_vel, x_deltav);
			y_deltav = resist(y_vel, y_deltav);

			// Don't exceed speed limit
			x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
			y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

			// Try to move horizontally
			player_box.rect.set_x(player_box.rect.x() + x_vel);
			// Use the "go-back" approach to collision resolution
			if check_collision(&player_box, &static_box)
				|| player_box.rect.left() < 0
				|| player_box.rect.right() > CAM_W as i32
			{
				player_box.rect.set_x(player_box.rect.x() - x_vel);
			}

			// Try to move vertically
			player_box.rect.set_y(player_box.rect.y() + y_vel);
			if check_collision(&player_box, &static_box)
				|| player_box.rect.top() < 0
				|| player_box.rect.bottom() > CAM_H as i32
			{
				player_box.rect.set_y(player_box.rect.y() - y_vel);
			}	

			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			self.core.wincan.set_draw_color(Color::RED);
			self.core.wincan.fill_rect(static_box.rect)?;

			self.core.wincan.set_draw_color(Color::CYAN);
			self.core.wincan.fill_rect(player_box.rect)?;

			self.core.wincan.set_draw_color(Color::CYAN);
			self.core.wincan.fill_rect(player_box_2.rect)?;

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	street_code_fighter::runner(TITLE, SDL08::init);
}
