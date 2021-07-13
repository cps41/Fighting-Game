

use sdl2::rect::{Point, Rect}; // for hazard hitboxes
// maybe incorporate a
pub enum Types {
	Stalactites, // <- we can add more as we go if we want
}

// Structs
pub struct Hazard {
    pub active: bool,
	pub name: Types,
    pub falling: bool,
	pub fell: bool,
    pub hit: bool,
    pub fall_speed: f64,
    pub damage: f64,
    pub position: Point,
	pub sprite: Rect,
}

impl Hazard {
	pub fn new( ) -> Hazard {
		Hazard {
			name: Types::Stalactites,
            active: false,
            falling: false,
			fell: false,
            hit: false,
            fall_speed: 1.0, // idk something to start with
            damage: 5.0, // same as above ^^
            position: Point::new(35,0),
			sprite: Rect::new(250, 0, 100, 100),
		}
    }

	pub fn reset(&mut self, ) {
		if self.sprite.x() > 800 {
			self.sprite.offset(-650, -600);
		}
		else {
			self.sprite.offset(350, -600);
		}
		self.fell = true;
	}
        // // setters
        // pub fn set_active(&mut self) -> &mut bool { &mut self.active; }
        // pub fn set_falling(&mut self) -> &mut bool { &mut self.falling; }
        // pub fn set_hit(&mut self) -> &mut bool { &mut self.hit; }
        // pub fn set_fallspeed(&mut self) -> &mut f64{ &mut self.fall_speed; }
        // pub fn set_damage(&mut self) -> &mut f64 { &mut self.damage; }
        // pub fn set_position(&mut self) -> &mut Point { &mut self.position; }
		//
        // // getters
        // pub fn active(&self) -> &bool { &self.active; }
        // pub fn falling(&self) -> &bool { &self.falling; }
        // pub fn hit(&self) -> &bool { &self.hit; }
        // pub fn fallspeed(&self) -> &f64 { &self.fall_speed; }
        // pub fn damage(&self) -> &f64 { &self.damage; }
        // pub fn position(&self) -> &Point { &self.position; }


        // pub fn check_hit(&mut self) -> bool {
        //     if self.position.y() <= 0 { return true; } // if it hit the ground (assumed y level 0)
        //     else if self.position.y() <= 0 { return true; } // if it hit a non player
        //     else if self.position.y() <= 0 { return true; } // if it hit a player
        //     else { return false; } //
        // }
}
