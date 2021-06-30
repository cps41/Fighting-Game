
#![allow(non_snake_case)]
use sdl2::rect::Rect;
use crate::physics::nodes::*;

fn boxUp<T>(data: T) -> BoxRef<T>{
	Some(Box::new(data))
}

#[derive(Debug)]
pub struct Node<T> {
    pub parent: WeakLink<T>,
    pub left: Link<T>,
    pub right: Link<T>,
    pub bv: BoxRef<T>, // bounding volume
	pub area: Rect, // total bounding area of children
}

pub fn check_collision(a: &CollisionObject, b: &CollisionObject) -> bool {
	if let CollisionObjectType::HurtBox = a.obj_type {
		if let CollisionObjectType::HurtBox = b.obj_type {return false}
	}
	reg_collision(&a.rect, &b.rect)
}

fn reg_collision(a: &Rect, b: &Rect) -> bool {
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

pub fn resist(vel: i32, deltav: i32) -> i32 {
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

pub type PotentialCollision = (CollisionObject, CollisionObject);

pub trait Area {
	fn area(&self) -> u32;
}

impl Area for Rect {
	fn area(&self) -> u32 {
		self.width()*self.height()
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollisionObject {
    pub obj_type: CollisionObjectType,
	pub area: u32,
    pub rect: Rect,
}

impl CollisionObject {
    pub fn new(obj_type: CollisionObjectType, x: i32, y: i32, width: u32, height: u32) -> CollisionObject {
        let rect = Rect::new(x, y, width, height);
		let area = rect.area();

        CollisionObject {
            obj_type,
			area,
            rect,
        }
    }

	fn overlapsWith(&self, other: &CollisionObject) -> bool {
		check_collision(self, other)
	}
}

trait Unbox<T> {
	fn unbox<'a> (&'a self) -> &'a mut T;
}

impl Node<CollisionObject> {
	pub fn new(parent: WeakLink<CollisionObject>, bv: CollisionObject) -> Self {
		Node{
			parent: parent,
			left: None,
			right: None,
			bv: boxUp(bv),
			area: bv.rect,
		}
	}

	pub fn isLeaf(&self) -> bool {
		if let None = self.bv {
			false // node is a leaf iff node points to collision object
		}
		else {true}
	}

	pub fn detatch(&mut self) {
		let parent = self.parent.take();
		let left = self.left.take();
		let right = self.right.take();
	}
}


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
	fn testBVHNodeInit() {
		let co = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
		let node = NodeRef::new(co.clone());

		assert_eq!(node.get().left.as_ref().map(|a| Some(false)), None);
		assert_eq!(node.get().right.as_ref().map(|a| Some(false)), None);
		assert_eq!(node.get().bv.as_ref().take(), Some(&Box::new(co)));
		assert_eq!(node.get().area, Rect::new(0,2,3,3));
	}

	#[test]
	fn testBVHNodeInsert() {
		let co1 = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
		let co2 = CollisionObject::new(CollisionObjectType::HitBox, 5, 0, 6, 2);
		let co3 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 2, 2);
		let node = NodeRef::new(co1.clone());
		let (left, right) = node.insert(co2.clone());

		assert_eq!(node.getLeftChild(), left);
		assert_eq!(node.getRightChild(), right);
		assert_eq!(node.get().bv.as_ref().take(), None);
		assert_eq!(node.get().area, Rect::new(0,0,11,5));

		node.insert(co3.clone());
		let l2 = NodeRef::new(co3);
		l2.getMut().parent = Some(std::rc::Weak::new());

		assert_eq!(node.getLeftChild(), left);
		assert_eq!(node.getLeftChild().getRightChild().get().bv.as_deref().unwrap(), &co3.clone());
		assert_eq!(node.getRightChild(), right);
		assert_eq!(node.get().bv.as_ref().take(), None);
		assert_eq!(node.get().area, Rect::new(0,0,22,22));
	}
	
	#[test]
	fn testBVHNodeRemove() {
		let co1 = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
		let co2 = CollisionObject::new(CollisionObjectType::HitBox, 5, 0, 6, 2);
		let co3 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 2, 2);
		let node = NodeRef::new(co1.clone());
		let (left, right) = node.insert(co2.clone());
		node.insert(co3.clone());
		left.getRightChild().clone().remove();

		assert_eq!(node.getLeftChild(), left);
		assert_eq!(node.getLeftChild().get().bv.as_deref().unwrap(), &co1.clone());
		assert_eq!(node.getRightChild(), right);
		assert_eq!(node.get().bv.as_ref().take(), None);
		assert_eq!(node.get().area, Rect::new(0,0,11,5));
	}
}
