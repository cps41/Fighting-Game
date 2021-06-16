extern crate street_code_fighter;

use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL08 Rect Collisions";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

fn check_collision(a: &Rect, b: &Rect) -> bool {
    if a.obj_type != Character{
            if a.bottom() < b.top()
            || a.top() > b.bottom()
            || a.right() < b.left()
            || a.left() > b.right()
        {
            false
        }
        else {
            true
        }
    }
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

enum COLLISION_OBJECT_TYPE {
    Character,
    Hazard,
    Platform,
    Wall
}

pub struct COLLISION_OBJECT {
    obj_type: COLLISION_OBJECT_TYPE,
    size: i16,
    rect: Rect,
}

impl COLLISION_OBJECT {
    fn new(obj_type: COLLISION_OBJECT_TYPE, x: i32, y: i32, width: i32, height: i32) -> COLLISION_OBJECT {
        let r = Rect::new(x, y, width, height);
        COLLISION_OBJECT {
            obj_type,
            r,
        }
    }
}

fn main() {
	sdl_rust::runner(TITLE, SDL08::init);
}
