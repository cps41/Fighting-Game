#![allow(non_snake_case)]
use sdl2::rect::{Rect, Point};
use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::fmt;
use crate::view::globals::*;
use crate::physics::nodes::*;
use crate::physics::particle::*;
use crate::physics::vecmath::*;

pub struct BVHierarchy {
	pub head: NodeRef<CollisionObject>,
}

impl BVHierarchy {
	pub fn new(co: CollisionObject) -> BVHierarchy {
		BVHierarchy{ head: NodeRef::new(co) }
	}
	pub fn insert(&self, co: CollisionObject) -> RefCell<CollisionObject> {
		// println!("inserting {:?}", co);
		self.head.insert(co)
	}
	pub fn resolve_collisions(&self) -> bool {
		let mut potential_collisions: Vec<ParticleContact> = Vec::new();
		let count = self.head.getPotentialCollisions(&mut potential_collisions, 100);
		// println!("Counted {} collisions\n", count);
		let mut hazard_reset = false; // bool to reset hazard upon impact
		for contact in potential_collisions.iter_mut() {
			let p0 = contact.objects[0].clone();
			let p1 = contact.objects[1].clone();
			if check_collision(p0.borrow().clone(), p1.borrow().clone()) {
				// println!("Resolving....");
				if contact.resolve_velocity(FRAME_RATE as f32) {hazard_reset = true}
				contact.resolve_interpenetration();
				match (p0.borrow().obj_type, p1.borrow().obj_type) {
					(CollisionObjectType::Platform, _) => (),
					// println!("\n\nContact between\n {:?}\nand\n {:?}", contact.objects[0], contact.objects[1]),
					(_, CollisionObjectType::Platform) => (),
					// println!("\n\nContact between\n {:?}\nand\n {:?}", contact.objects[0], contact.objects[1]),
					_ => // ()
					{println!("\n\n**********BVH Head: {:?}\n", self.head);
					println!("\n\nContact between\n {:?}\nand\n {:?}", contact.objects[0], contact.objects[1])},
				}
				// println!("\nVelocities updated between\n {:?}\nand\n {:?}", contact.particles[0], contact.particles[1]);
			}
		}
		hazard_reset
	}
}

pub fn boxUp<T>(data: T) -> Option<RefCell<T>>{
	Some(RefCell::new(data))
}

pub struct Node<T> {
    pub parent: WeakLink<T>,
    pub left: Link<T>,
    pub right: Link<T>,
    pub bv: Option<RefCell<T>>, // bounding volume
	pub area: Rect, // total bounding area of children
}
impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("\n\tNode")
		.field("\n\tleft", &self.left)
		.field("\n\tright", &self.right)
		.field("\n\tbv", &self.bv)
		.finish()
    }
}

pub fn check_collision(a: CollisionObject, b: CollisionObject) -> bool {
	let types = (&a.obj_type, &b.obj_type);
	match types {
		(CollisionObjectType::HurtBox, CollisionObjectType::HurtBox) => false,
		_ => a.rect.has_intersection(b.rect.clone())
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CollisionObjectType {
	HitBox,
	HurtBox,
	BlockBox, // for if we want to implement it elsewhere
    Hazard,
    Platform,
    Wall,
	Empty,
}

pub struct ParticleContact {
	pub objects: Vec<RefCell<CollisionObject>>,
	pub restitution: f32,
	pub contact_normal: PhysVec,
	pub interpenetration: PhysVec,
}

impl ParticleContact {
	pub fn new(a:RefCell<CollisionObject>, b: RefCell<CollisionObject>, contact_normal: PhysVec, restitution: f32, interpenetration: PhysVec) -> Self {
		ParticleContact {
			objects: vec![a, b],
			restitution: restitution,
			contact_normal: contact_normal,
			interpenetration: interpenetration,
		}
	}

	fn separating_velocity(&self) -> f32 {
		let contact_0 = self.objects[0].borrow();
		let p0 = contact_0.particle.borrow();
		let contact_1 = self.objects[1].borrow();
		let p1 = contact_1.particle.borrow();
		let mut relative_velocity = p0.velocity.clone();
		relative_velocity.replace(&relative_velocity.sub(&p1.velocity));
		relative_velocity.scalar_product(&self.contact_normal)
	}

	fn resolve_velocity(&mut self, duration: f32) -> bool{
		let a = &self.objects[0].borrow().particle;
		let b = &self.objects[1].borrow().particle;
		let separating_velocity = self.separating_velocity();
		if separating_velocity > 0f32 { 
			return false
		} // contact is either separating or stationary, no impulse required

		let new_sep_velocity = -separating_velocity*self.restitution;
		let delta_velocity = new_sep_velocity - separating_velocity;

		let total_inv_mass = a.borrow().inverse_mass + b.borrow().inverse_mass;
		let impulse = delta_velocity / total_inv_mass;
		let impulse_per_mass = self.contact_normal.dot_product(impulse);

		// println!("normal: {:?}, sep_vel: {:?}, impulse/mass: {:?}", self.contact_normal, separating_velocity, impulse_per_mass);

		let types = (self.objects[0].borrow().obj_type, self.objects[1].borrow().obj_type);
		let mass_a = a.borrow().inverse_mass;
		let mass_b = b.borrow().inverse_mass;
		match &types {
			// stop y movement for platform/wall collisions
			(CollisionObjectType::Platform, _) => self.objects[1].borrow().particle.borrow_mut().reset_y(),
			(_, CollisionObjectType::Platform) => self.objects[0].borrow().particle.borrow_mut().reset_y(),
			(CollisionObjectType::Wall, _) => self.objects[1].borrow().particle.borrow_mut().reset_y(),
			(_, CollisionObjectType::Wall) => self.objects[0].borrow().particle.borrow_mut().reset_y(),

			// alter health for hit/hazard collisions
			(CollisionObjectType::HitBox, CollisionObjectType::HurtBox) | (CollisionObjectType::Hazard, CollisionObjectType::HurtBox) => {
				self.objects[1].borrow().particle.borrow_mut().update_health(50);
				self.objects[0].borrow().particle.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(mass_a));
				self.objects[1].borrow().particle.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(mass_b));
			},
			(CollisionObjectType::HurtBox, CollisionObjectType::HitBox) | (CollisionObjectType::HurtBox, CollisionObjectType::Hazard) => {
				self.objects[0].borrow().particle.borrow_mut().update_health(50);
				self.objects[0].borrow().particle.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(mass_a));
				self.objects[1].borrow().particle.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(mass_b));
			},

			// just update others
			_ => {
				self.objects[0].borrow().particle.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(mass_a));
				self.objects[1].borrow().particle.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(mass_b));
			},
		}
		match &types {
			(CollisionObjectType::Hazard, _) | (_, CollisionObjectType::Hazard) => true,
			_ => false,
		}
	}

	fn resolve_interpenetration(&self) {
		let types = (self.objects[0].borrow().obj_type, self.objects[1].borrow().obj_type);
		let a_size = self.objects[0].borrow().rect.size();
		let a_loc = self.objects[0].borrow().rect.top_left();
		let b_size = self.objects[1].borrow().rect.size();
		let b_loc = self.objects[1].borrow().rect.top_left();

		match &types {
			// stop y movement for platform collisions
			(CollisionObjectType::Platform, _) => self.objects[1].borrow().particle.borrow_mut().position.y -= self.interpenetration.y as f32,
			(_, CollisionObjectType::Platform) => self.objects[0].borrow().particle.borrow_mut().position.y -= self.interpenetration.y as f32,
			(CollisionObjectType::Wall, _) => {
				if a_loc.x() > b_loc.x() { // if wall is on the right side, shift object left
					self.objects[1].borrow().particle.borrow_mut().position.x -= self.interpenetration.x as f32;
				}
				else { // if wall is on the left side, shift object right
					self.objects[1].borrow().particle.borrow_mut().position.x += self.interpenetration.x as f32;
				}
			}
			(_, CollisionObjectType::Wall) => {
				if a_loc.x() < b_loc.x() {
					self.objects[0].borrow().particle.borrow_mut().position.x -= self.interpenetration.x as f32;
				}
				else {
					self.objects[0].borrow().particle.borrow_mut().position.x += self.interpenetration.x as f32;
				}
			}

			// just update others
			_ => {
				// if width overlap is less than height overlap, resolve x axis
				if self.interpenetration.x < self.interpenetration.y {
					if a_loc.x() < b_loc.x() {
						self.objects[0].borrow().particle.borrow_mut().position.x -= self.interpenetration.x as f32;
					}
					else {
						self.objects[0].borrow().particle.borrow_mut().position.x += (self.interpenetration.x) as f32;
					}
				}
				else {
					if a_loc.y() < b_loc.y() {
						self.objects[0].borrow().particle.borrow_mut().position.y -= self.interpenetration.y as f32;
					}
					else {
						self.objects[0].borrow().particle.borrow_mut().position.y += self.interpenetration.y as f32;
					}
				}
			},
		}
	}
}

pub trait Area {
	fn area(&self) -> u32;
}

impl Area for Rect {
	fn area(&self) -> u32 {
		self.width()*self.height()
	}
}

#[derive(Clone)]
pub struct CollisionObject {
    pub obj_type: CollisionObjectType,
	pub area: u32,
    pub rect: Rect,
	pub noderef: WeakLink<CollisionObject>,
	pub particle: Rc<RefCell<Particle>>
}


impl fmt::Debug for CollisionObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("\n\t\tCollision Object")
		.field("obj_type", &self.obj_type)
		.field("area", &self.area)
		.field("rect", &self.rect)
		.field("position", &self.particle)
		.finish()
    }
}

impl CollisionObject {
    pub fn new(obj_type: CollisionObjectType, x: i32, y: i32, width: u32, height: u32, particle: Rc<RefCell<Particle>>) -> CollisionObject {
        let rect = Rect::new(x, y, width, height);
		let area = rect.area();
		let noderef: WeakLink<CollisionObject> = None;

        CollisionObject {
            obj_type,
			area,
            rect,
			noderef,
			particle,
        }
    }
    pub fn new_from(obj_type: CollisionObjectType, rect: Rect, particle: Rc<RefCell<Particle>>) -> CollisionObject {
		let area = rect.area();
		let noderef: WeakLink<CollisionObject> = None;

        CollisionObject {
            obj_type,
			area,
            rect,
			noderef,
			particle,
        }
    }
	pub fn getNodeRef(&self) -> Option<NodeRef<CollisionObject>> {
		match &self.noderef {
       		Some(p) => p.upgrade().map(|up| NodeRef(up)), //p.upgrade().map(|u| NodeRef(u)),
			None => None
		}
	}
	pub fn update(&mut self, position: Point) {
		self.rect.reposition(position);
	}

	fn overlapsWith(&self, other: &CollisionObject) -> bool {
		self.rect.has_intersection(other.rect)
	}
}

trait Unbox<T> {
	fn unbox<'a> (&'a self) -> &'a mut T;
}

impl Node<CollisionObject> {
	pub fn new(parent: WeakLink<CollisionObject>, bv: CollisionObject) -> Self {
		let area = bv.rect.clone();
		let bv = boxUp(bv);
		Node{
			parent: parent,
			left: None,
			right: None,
			bv: bv,
			area: area,
		}
	}

	pub fn isLeaf(&self) -> bool {
		!self.bv.is_none()
	}

	pub fn detatch(&mut self) {
		let parent = self.parent.take();
		let left = self.left.take();
		let right = self.right.take();
		let mut bv = self.bv.take();
		bv.take().map(|bv| bv.borrow_mut().noderef.take());
	}
}

/*
#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn testCollide() {
		let c1 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 10, 20);
		let c2 = CollisionObject::new(CollisionObjectType::Hazard, 28, 20, 10, 20);

		assert_eq!(check_collision(&c1, &c2), true);
	}

	#[test]
	fn testCollisionInitFrom() {
		let r1 = Rect::new(0, 0, 3, 3);
		let c1 = CollisionObject::new_from(CollisionObjectType::HitBox, r1);

		assert_eq!(c1.rect, r1);
	}

	#[test]
	fn testCollisionUpdate() {
		let r1 = Rect::new(0, 0, 3, 3);
		let mut c1 = CollisionObject::new_from(CollisionObjectType::HitBox, r1);
		c1.update(Point::new(4, 4));

		assert_eq!(c1.rect, Rect::new(4,4,3,3));
	}

	#[test]
	fn testBVHNodeInit() {
		let co = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
		let node = NodeRef::new(co.clone());

		assert_eq!(node.get().left.as_ref().map(|a| Some(false)), None);
		assert_eq!(node.get().right.as_ref().map(|a| Some(false)), None);
		assert_eq!(node.get().bv.as_ref().take(), Some(&RefCell::new(co)));
		assert_eq!(node.get().area, Rect::new(0,2,3,3));
	}

	#[test]
	fn testBVHNodeInsert() {
		let co1 = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
		let co2 = CollisionObject::new(CollisionObjectType::HitBox, 5, 0, 6, 2);
		let co3 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 2, 2);
		let node = NodeRef::new(co1.clone());
		let l = node.clone();
		let new = node.insert(co2.clone());

		assert_eq!(node.getLeftChild().get().bv.as_ref().unwrap(), &RefCell::new(co1.clone()));
		assert_eq!(node.getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co2.clone()));
		assert_eq!(node.get().bv.as_ref().take(), None);
		assert_eq!(node.get().area, Rect::new(0,0,11,5));

		node.insert(co3.clone());
		let l2 = NodeRef::new(co3);
		l2.getMut().parent = Some(std::rc::Weak::new());

		assert_eq!(node.getLeftChild().getLeftChild().get().bv.as_ref().unwrap(), &RefCell::new(co1.clone()));
		assert_eq!(node.getLeftChild().getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co3.clone()));
		assert_eq!(node.getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co2.clone()));
		assert_eq!(node.get().bv.as_ref().take(), None);
		assert_eq!(node.get().area, Rect::new(0,0,22,22));
	}

	// #[test]
	// fn testBVHNodeRemove() {
	// 	let co1 = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
	// 	let co2 = CollisionObject::new(CollisionObjectType::HitBox, 5, 0, 6, 2);
	// 	let co3 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 2, 2);
	// 	let node = NodeRef::new(co1.clone());
	// 	node.insert(co2.clone());
	// 	let mut nodec3 = node.insert(co3.clone());
	// 	nodec3.remove();

	// 	assert_eq!(node.getLeftChild().get().bv.as_ref().unwrap(), &RefCell::new(co1.clone()));
	// 	assert_eq!(node.getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co2.clone()));
	// 	assert_eq!(node.get().bv.as_ref().take(), None);
	// 	assert_eq!(node.get().area, Rect::new(0,0,11,5));
	// }
}
*/
