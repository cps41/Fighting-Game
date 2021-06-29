#![allow(non_snake_case)]
use sdl2::rect::Rect;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::{Ref, RefMut, RefCell};

pub fn check_collision(a: &CollisionObject, b: &CollisionObject) -> bool {
	if let CollisionObjectType::HurtBox = a.obj_type {
		if let CollisionObjectType::HurtBox = b.obj_type {return false}
	}
	reg_collision(&a.rect, &b.rect)
}

fn makeLink<T>(node: T) -> Link<T> {
	Some(Box::new(node))
}

fn makeParLink (node: *const BVHNode) -> ParentLink<*const BVHNode> {
	Some(Rc::new(RefCell::new(node)))
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
}

// Link type for child nodes
type Link<T> = Option<Box<T>>;

type ParentLink<T> = Option<Rc<RefCell<T>>>;

trait Refer<'a, T> {
	fn refer(&'a self) -> &'a T;
	fn defer(&'a self) -> &'a T;
}

impl<'a, T> Refer<'a, T> for Link<T> {
	fn refer(&'a self) -> &'a T {
		self.as_ref().unwrap()
	}

	fn defer(&'a self) -> &'a T {
		self.as_deref().unwrap()
	}
}

trait Defer<'a, T> {
	fn defer(&self) -> &RefCell<T>;
}

impl<'a> Defer<'a, BVHNode> for ParentLink<BVHNode> {	
	fn defer(&self) -> &RefCell<BVHNode> {
		self.as_deref().unwrap()
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct BVHNode {
	parent: ParentLink<*const BVHNode>,
	children: (Link<BVHNode>, Link<BVHNode>),
	obj: Link<CollisionObject>,
	area: Rect,
}

impl BVHNode {
	fn new(children: (Link<BVHNode>, Link<BVHNode>), obj: Link<CollisionObject>) -> BVHNode {
		let mut node = BVHNode{parent: None, children: children, obj: obj, area: Rect::new(0,0,0,0)};
		node.calculateArea();
		node
	}

	fn newp(parent: ParentLink<*const BVHNode>, children: (Link<BVHNode>, Link<BVHNode>), obj: Link<CollisionObject>) -> BVHNode {
		let mut node = BVHNode{parent: parent, children: children, obj: obj, area: Rect::new(0,0,0,0)};
		node.calculateArea();
		node
	}

	fn setParent(&mut self, parent: ParentLink<*const BVHNode>) {
		self.parent = parent;
		self.calculateArea();
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

		if self.hasParent() {self.parent.defer().borrow_mut().calculateArea();}
		println!("New Area: {}", self.area.area());
	}

	fn isLeaf(&self) -> bool {
		if let None = self.obj {
			false // node is a leaf iff node points to collision object
		}
		else {true}
	}

	fn hasParent(&self) -> bool {
		if let None = self.parent {false}
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
		if !self.isLeaf() {
			let size0 = self.children.0.refer().area.area();
			let size1 = self.children.1.refer().area.area();

			if size0 <= size1 {
				self.children.0.as_mut().unwrap().insert(new_obj);
			}
			else {
				self.children.1.as_mut().unwrap().insert(new_obj);
			}
		}
		else {
			let addr = &*self as *const _;
			self.children.0 = makeLink(BVHNode::newp(makeParLink(addr), (None, None), self.obj.take()));
			self.children.1 = makeLink(BVHNode::newp(makeParLink(self.clone()), (None, None), makeLink(new_obj)));
		}
		self.calculateArea();
	}

	fn removeChild(&mut self, child: bool) {
		if self.isLeaf() {return}

		if child {
			self.obj = self.children.1.as_deref_mut().unwrap().obj.take();
		}
		else {
			self.obj = self.children.0.as_deref_mut().unwrap().obj.take();
		}
		self.children.0.take();
		self.children.1.take();
		self.calculateArea();
	}

	fn remove(&self, parent: Option<&mut BVHNode>) -> &Self {
		if let Some(parent) = parent {
			match parent.children.0.as_deref().take() {
				Some(child0) => {
					if child0 == &self.clone() {
						println!("child 0 removed");
						parent.obj = parent.children.1.defer().obj.clone();
					}
					else {
						println!("child 1 removed");
						parent.obj = child0.obj.clone();
					}
				}
				None => {
					println!("child 0 removed");
					parent.obj = parent.children.1.defer().obj.clone();
				}
			}
			parent.children.0.take();
			parent.children.1.take();
			parent.calculateArea();
		}
		self
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
		let mut node = BVHNode::new((None, None), None);

		assert_eq!(node.children.0, None);
		assert_eq!(node.children.1, None);
		assert_eq!(node.obj, None);
		assert_eq!(node.area, Rect::new(0,0,0,0));
	}

	#[test]
	fn testBVHNodeInsert() {
		let node7547 = BVHNode::new((None, None), makeLink(CollisionObject::new(CollisionObjectType::HitBox, 7, 5, 4, 7)));
		let node55410 = BVHNode::new((None, None), makeLink(CollisionObject::new(CollisionObjectType::HitBox, 5, 5, 4, 10)));
		let node58212 = BVHNode::new((None, None), makeLink(CollisionObject::new(CollisionObjectType::HitBox, 5, 8, 2, 12)));

		let node = &mut node7547.clone();
		node.insert(CollisionObject::new(CollisionObjectType::HitBox, 5, 5, 4, 10));

		assert_eq!(node.parent, None);
		assert_eq!(node.obj, None);
		assert_eq!(node.area, Rect::new(5,5,6,10));
		
		let mut parent = makeParLink(BVHNode::new((makeLink(node7547.clone()), makeLink(node55410.clone())), None));
		assert_eq!(node.children.0.refer().parent, parent);
		assert_eq!(node.children.0.refer().children, (None, None));
		assert_eq!(node.children.0.refer().area.area(), 28);
		assert_eq!(node.children.0.refer().obj, node7547.obj);
		assert_eq!(node.children.1.refer().parent, parent);
		assert_eq!(node.children.1.refer().children, (None, None));
		assert_eq!(node.children.1.refer().area.area(), 40);
		assert_eq!(node.children.1.refer().obj, node55410.obj);

		node.insert(CollisionObject::new(CollisionObjectType::Hazard, 5, 8, 2, 12));
		let cur = node.children.0.as_deref_mut().unwrap();

		assert_eq!(node.area, Rect::new(5,5,6,15));

		// parent = makeParLink(BVHNode::new((makeLink(node7547.clone()), makeLink(node55410.clone())), None));
		// assert_eq!(node.children.0.refer().parent, parent);
		// assert_eq!(node.children.0.refer().children, (None, None));
		// assert_eq!(node.children.0.refer().area.area(), 28);
		// assert_eq!(node.children.0.refer().obj, node7547.obj);
		// assert_eq!(node.children.1.refer().parent, parent);
		// assert_eq!(node.children.1.refer().children, (None, None));
		// assert_eq!(node.children.1.refer().area.area(), 40);
		// assert_eq!(node.children.1.refer().obj, node55410.obj);
	}

	#[test]
	fn testBVHNodeRemove() {
		let node7547 = BVHNode::new((None, None), makeLink(CollisionObject::new(CollisionObjectType::HitBox, 7, 5, 4, 7)));
		let node55410 = BVHNode::new((None, None), makeLink(CollisionObject::new(CollisionObjectType::HitBox, 5, 5, 4, 10)));
		let node58212 = BVHNode::new((None, None), makeLink(CollisionObject::new(CollisionObjectType::HitBox, 5, 8, 2, 12)));

		let node = &mut node7547.clone();
		node.insert(CollisionObject::new(CollisionObjectType::HitBox, 5, 5, 4, 10));
		node.insert(CollisionObject::new(CollisionObjectType::Hazard, 5, 8, 2, 12));
		let child0 = node.children.0.as_deref_mut().unwrap();
		let removed = child0.children.1.as_deref_mut().unwrap().remove();

		assert_eq!(removed.obj.as_deref().unwrap(), &CollisionObject::new(CollisionObjectType::Hazard, 5, 8, 2, 12));
		assert_eq!(node.parent, None);
		assert_eq!(node.obj, None);
		// assert_eq!(node.area, Rect::new(5,5,6,10));
		
		let mut parent = makeParLink(BVHNode::new((makeLink(node7547.clone()), makeLink(node55410.clone())), None));
		assert_eq!(node.children.0.refer().parent, parent);
		assert_eq!(node.children.0.refer().children, (None, None));
		assert_eq!(node.children.0.refer().area.area(), 28);
		assert_eq!(node.children.0.refer().obj, node7547.obj);
		assert_eq!(node.children.1.refer().parent, parent);
		assert_eq!(node.children.1.refer().children, (None, None));
		assert_eq!(node.children.1.refer().area.area(), 40);
		assert_eq!(node.children.1.refer().obj, node55410.obj);
		let cur = node.children.0.as_deref_mut().unwrap();
	}
}
