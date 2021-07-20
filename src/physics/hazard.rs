use std::rc::*;
use std::cell::*;
use crate::physics::particle::Particle;
use crate::physics::collisions::*;
use crate::physics::vecmath::PhysVec;
use crate::view::globals::*;
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
	pub hitbox: Option<RefCell<CollisionObject>>,
	pub particle: Rc<RefCell<Particle>>,
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
            hitbox: None,
            particle: Rc::new(RefCell::new(Particle::new(PhysVec::new(35f32,0f32), 0.01, 300f32, 0))),
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
	pub fn remove(link: &mut Option<RefCell<CollisionObject>>) {
		link.take().map(|l| {
			l.borrow().getNodeRef().map(|n| {
				// println!("\nremoving {:?}\n", n);
				n.remove()
			});
		});
	}
	pub fn insert(&mut self, bvh: &BVHierarchy) {
		// println!("inserting block box...");
		Hazard::remove(&mut self.hitbox);
		self.hitbox = Some(bvh.insert(
			CollisionObject::new(
				CollisionObjectType::BlockBox, self.position.x()+W_OFFSET, self.position.y()+H_OFFSET, 100, 100, self.particle.clone())
		));
	}
	pub fn update_bounding_box(&mut self, bvh: &BVHierarchy) {
		// println!("updating...");
		// println!("\nUpdating Bounding Boxes {:?}", bvh.head);
        Hazard::remove(&mut self.hitbox);
        self.insert(&bvh);
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
