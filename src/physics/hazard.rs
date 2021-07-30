use std::rc::*;
use std::cell::*;
use crate::physics::particle::Particle;
use crate::physics::collisions::*;
use crate::physics::vecmath::PhysVec;
use crate::view::globals::*;
use sdl2::rect::{Point, Rect}; // for hazard hitboxes
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};
// maybe incorporate a
pub enum Types {
	Stalactites, // <- we can add more as we go if we want
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HazardVar {
	pub pos_x: 		i32,
	pub pos_y: 		i32,
	pub sprite_x: 	i32,
	pub sprite_y:	i32,
	pub sprite_w:	u32,
	pub sprite_h:	u32,
}

impl HazardVar{
	pub fn new (hazard: &Hazard) -> HazardVar {
		HazardVar{
			pos_x: 		hazard.position.x(),
			pos_y: 		hazard.position.y(),
			sprite_x:	hazard.sprite.x(),
			sprite_y:	hazard.sprite.y(),
			sprite_w:	hazard.sprite.width(),
			sprite_h:	hazard.sprite.height(),
		}
	}

	pub fn from_hazvar(&mut self, other: &HazardVar){
		self.pos_x = other.pos_x;
		self.pos_y = other.pos_y;
		self.sprite_x = other.sprite_x;
		self.sprite_y = other.sprite_y;
		self.sprite_w = other.sprite_w;
		self.sprite_h = other.sprite_h;
	}
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
            particle: Rc::new(RefCell::new(Particle::new(PhysVec::new(35f32,0f32), 0.01, 300f32, 0, 20))),
		}
    }
	pub fn update_position(&mut self) {
		let mut scaled = PhysVec::new(0.0, 0.0);
		self.particle.borrow_mut().velocity.y = 200.0;
		// scaled.dot_replace(1.0/0.0002645833);
		self.particle.borrow_mut().add_force(&scaled);
		self.particle.borrow_mut().integrate(FRAME_RATE as f32);
		self.sprite.reposition(self.particle.borrow().to_point());
	}
	pub fn reset(&mut self, ) {
		self.sprite.set_y(0);
		if self.sprite.x() > 800 {
			self.sprite.offset(-650, 0);
		}
		else {
			self.sprite.offset(350, 0);
		}
		self.particle.borrow_mut().reset_x();
		self.particle.borrow_mut().reset_y();
		self.particle.borrow_mut().position.x = self.sprite.x() as f32;
		self.particle.borrow_mut().position.y = self.sprite.y() as f32;
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
		// println!("\nUpdating Hazard\n {:?}\n", self.hitbox);
        Hazard::remove(&mut self.hitbox);
        self.insert(&bvh);
    }
    
    pub fn get_bb(&self) -> Rect {
        if self.hitbox.is_some() {
        self.hitbox.clone().unwrap().borrow().rect.clone()
        }
		else {Rect::new(0,0,0,0)}
    }

    pub fn from_packet(&mut self, packet: &HazardVar){
    	self.position.offset( (packet.pos_x - self.position.x()), (packet.pos_y - self.position.y()));
    	self.sprite.set_x(packet.sprite_x);
    	self.sprite.set_y(packet.sprite_y);
    	self.sprite.set_width(packet.sprite_w);
    	self.sprite.set_height(packet.sprite_h);
    }

}
