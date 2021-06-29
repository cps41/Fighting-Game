#![allow(non_snake_case)]
use sdl2::rect::Rect;
use std::cell::{self, RefCell};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

use crate::physics::nodes::{NodeRef, Link, WeakLink, BoxRef};

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
			area: Rect::new(0,0,0,0),
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

		assert_eq!(node.get().left, None);
		assert_eq!(node.get().right, None);
		assert_eq!(node.get().bv, co);
		assert_eq!(node.get().area, Rect::new(0,2,3,3));
	}
	#[test]
	fn testBVHNodeInsert() {
		let mut node = BVHNode::new((None, None), 
									Some(Box::new(
										CollisionObject::new(CollisionObjectType::HitBox, 
										7, 5, 4, 7))), 
										Rect::new(0, 0, 0, 0));
		node.insert(CollisionObject::new(CollisionObjectType::HitBox, 5, 5, 4, 10));

		assert_ne!(node.left, None);
		assert_ne!(node.right, None);
		assert_eq!(node.bv, None);
		assert_eq!(node.area, Rect::new(5,5,6,10));
		assert_eq!(*node.left.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::HitBox, 7,5,4,7))), 
													area: Rect::new(7,5,4,7)});
		assert_eq!(*node.right.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::HitBox, 5,5,4,10))), 
													area: Rect::new(5,5,4,10)});
													
		node.insert(CollisionObject::new(CollisionObjectType::Hazard, 5, 8, 2, 12));
		let cur = node.left.as_deref_mut().unwrap();

		assert_ne!(cur.left, None);
		assert_ne!(cur.right, None);
		assert_eq!(cur.bv, None);
		assert_eq!(cur.area, Rect::new(5,5,6,15));
		assert_eq!(node.area, Rect::new(5,5,6,15));
		assert_eq!(*cur.left.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::HitBox, 7,5,4,7))), 
													area: Rect::new(7,5,4,7)});
		assert_eq!(*cur.right.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::Hazard, 5,8,2,12))), 
													area: Rect::new(5,8,2,12)});
	}
}
