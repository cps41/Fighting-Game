#![allow(non_snake_case)]
use sdl2::rect::Rect;
use std::cell::{self, RefCell, Ref, RefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};



pub struct NodeRef<T>(Rc<RefCell<Node<T>>>);

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    parent: WeakLink<T>,
    left: Link<T>,
    right: Link<T>,
    node: T,
}

impl<T> NodeRef<T> {
	pub fn new(node: T) -> Self {
		NodeRef(Rc::new(RefCell::new(
			Node{
				parent: None,
				left: None,
				right: None,
				node: node
			}
		)))
	}

	pub fn getParent(&self) -> Option<NodeRef<T>> {
        Some(NodeRef(self.0.borrow().parent.as_ref().unwrap().upgrade().unwrap()))
	}
}


fn is<T>(a: &Rc<T>, b: &Rc<T>) -> bool {
    let a = &**a as *const T;
    let b = &**b as *const T;
    a == b
}

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> NodeRef<T> {
        NodeRef(self.0.clone())
    }
}

impl<T: fmt::Debug> fmt::Debug for NodeRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&*self.0.borrow(), f)
    }
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

type PotentialCollision = (CollisionObject, CollisionObject);

trait Area {
	fn area(&self) -> u32;
}

impl Area for Rect {
	fn area(&self) -> u32 {
		self.width()*self.height()
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollisionObject {
    obj_type: CollisionObjectType,
	area: u32,
    rect: Rect,
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

	fn isEmpty(&self) -> bool {
		match self.obj_type {
			CollisionObjectType::Empty => true,
			_ => false,
		}
	}
}

trait Empty<T> {
	fn empty() -> T;
}

impl Empty<CollisionObject> for CollisionObject {
	fn empty() -> CollisionObject {
		CollisionObject{obj_type: CollisionObjectType::Empty, area: 0, rect: Rect::new(0, 0, 0, 0)}
	}
}

trait Refer<T> {
	fn refer<'a>(&'a self) -> &'a T;
}

impl<T> Refer<T> for Link<T> {
	fn refer<'a>(&'a self) -> &'a T {
		
	}
}

#[derive(Clone, Debug)]
pub struct BVHNode {
	obj: Link<CollisionObject>,
	area: Rect,
}

impl BVHNode {
	fn new(children: (Link<BVHNode>, Link<BVHNode>), obj: Link<CollisionObject>, area: Rect) -> BVHNode {
		let mut node = BVHNode{obj: obj, area: area};
		node.calculateArea();
		node
	}

	fn calculateArea(&mut self) {
		if let None = self.children.0 {
			if let None = self.children.1 {
				if let None = self.obj {
					// !!!!! shouldn't be possible in gameplay, here for testing !!!!!
					self.area = Rect::new(0, 0, 0, 0); // area of 0 if node has no children and points to no collision object
				}
				else {
					self.area = self.obj.refer().rect.clone(); // area = area of collision object if node has no children but points to object
				}
			}
			else {
				self.area = self.children.1.refer().area.clone(); // area = children.1 if node has (None, Some)
			}
		}

		else if let None = self.children.1 {
			self.area = self.children.0.refer().area.clone(); // area = children.0 if node has (Some, None)
		}
		else {
			self.area = self.children.1.refer().area.union(self.children.0.refer().area); // area = smallest bounding box around both children if node has two children
		}
	}

	fn isLeaf(&self) -> bool {
		if let None = self.obj {
			false // node is a leaf iff node points to collision object
		}
		else {true}
	}

	fn overlapsWith(&self, other: &BVHNode) -> bool {
		self.area.has_intersection(other.area)
	}

	fn collidingWith(& self, other: & BVHNode, potential: &mut Option<PotentialCollision>, limit: i32) -> i32 {
		if !self.overlapsWith(other) || limit == 0 {return 0;}

		if self.isLeaf() && other.isLeaf() {
			potential.as_mut().unwrap().0 = self.obj.refer().clone();
			potential.as_mut().unwrap().1 = self.obj.refer().clone();
			return 1;
		}

		if other.isLeaf() || (!self.isLeaf() && self.area.area() >= other.area.area()) {
			let count = self.children.0.refer().collidingWith(self.children.1.refer(), potential, limit);

			if limit > count {
				return count + self.children.1.refer().collidingWith(other, potential, limit);
			}

			else {return count;}
		}

		else {
			let count = self.collidingWith(self.children.0.refer(), potential, limit);

			if limit > count {
				return count + self.collidingWith(self.children.1.refer(), potential, limit);
			}

			else {return count;}
		}
	}

	fn getPotentialCollsions(&self, potential: &mut Option<PotentialCollision>, limit: i32) -> i32{
		if self.isLeaf() || limit == 0 {return 0;}
		self.children.0.refer().collidingWith(self.children.1.refer(), potential, limit)
	}

	fn insert(&mut self, new_obj: CollisionObject) {
		if self.isLeaf() {
			self.children.0 = Some(Box::new(BVHNode::new((None, None), self.obj.take(), self.area)));
			self.children.1 = Some(Box::new(BVHNode::new((None, None), Some(Box::new(new_obj)), Rect::new(0, 0, 0, 0))));
		}

		else {
			let size0 = self.children.0.as_deref().map(|node| {
				node.area.area()
			});
			let size1 = self.children.1.as_deref().map(|node| {
				node.area.area()
			});
			if size0 <= size1 {
				self.children.0.as_mut().unwrap().insert(new_obj);
			}
			else {
				self.children.1.as_mut().unwrap().insert(new_obj);
			}
		}
		self.calculateArea();
	}

	fn remove(&mut self, parent: &mut Link<BVHNode>) {
		if let Some(parent_node) = parent {
			let mut sibling: Link<BVHNode>;
			if parent_node.children.0 == Some(Box::new(self.clone())) {sibling = parent_node.children.1.take();}
			else {sibling = parent_node.children.0.take();}

			*parent = sibling.take();
			parent.as_deref_mut().map(|par| {
				par.calculateArea();
			});
		}

		self.children.0.as_deref_mut().take();
		self.children.1.as_deref_mut().take();
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
		let mut node = BVHNode::new((None, None), None, Rect::new(0, 0, 0, 0));

		assert_eq!(node.children.0, None);
		assert_eq!(node.children.1, None);
		assert_eq!(node.obj, None);
		assert_eq!(node.area, Rect::new(0,0,0,0));
	}

	#[test]
	fn testBVHNodeInsert() {
		let mut node = BVHNode::new((None, None), 
									Some(Box::new(
										CollisionObject::new(CollisionObjectType::HitBox, 
										7, 5, 4, 7))), 
										Rect::new(0, 0, 0, 0));
		node.insert(CollisionObject::new(CollisionObjectType::HitBox, 5, 5, 4, 10));

		assert_ne!(node.children.0, None);
		assert_ne!(node.children.1, None);
		assert_eq!(node.obj, None);
		assert_eq!(node.area, Rect::new(5,5,6,10));
		assert_eq!(*node.children.0.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::HitBox, 7,5,4,7))), 
													area: Rect::new(7,5,4,7)});
		assert_eq!(*node.children.1.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::HitBox, 5,5,4,10))), 
													area: Rect::new(5,5,4,10)});
													
		node.insert(CollisionObject::new(CollisionObjectType::Hazard, 5, 8, 2, 12));
		let cur = node.children.0.as_deref_mut().unwrap();

		assert_ne!(cur.children.0, None);
		assert_ne!(cur.children.1, None);
		assert_eq!(cur.obj, None);
		assert_eq!(cur.area, Rect::new(5,5,6,15));
		assert_eq!(node.area, Rect::new(5,5,6,15));
		assert_eq!(*cur.children.0.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::HitBox, 7,5,4,7))), 
													area: Rect::new(7,5,4,7)});
		assert_eq!(*cur.children.1.refer(), BVHNode{children: (None, None),
													obj: Some(Box::new(CollisionObject::new(CollisionObjectType::Hazard, 5,8,2,12))), 
													area: Rect::new(5,8,2,12)});
	}
}
