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
				CollisionObjectType::Hazard, self.sprite.x(), self.sprite.y(), 100, 100, self.particle.clone())
		));
	}
	pub fn update_bounding_box(&mut self, bvh: &BVHierarchy) {
		// println!("updating...");
		// println!("\nUpdating Bounding Boxes {:?}", bvh.head);
        Hazard::remove(&mut self.hitbox);
        self.insert(&bvh);
    }
    pub fn get_bb(&self) -> Rect {
        if self.hitbox.is_some() {
        self.hitbox.clone().unwrap().borrow().rect.clone()
        }
		else {Rect::new(0,0,0,0)}
    }
}
