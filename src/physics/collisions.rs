extern crate street_code_fighter;

use sdl2::rect::Rect;
use std::cmp::max;
use std::cmp::min;

pub fn check_collision(a: &CollisionObject, b: &CollisionObject) -> bool {
	if let CollisionObjectType::HurtBox = a.obj_type {
		if let CollisionObjectType::HurtBox = b.obj_type {return false}
	}
	reg_collision(&a.rect, &b.rect)
}

pub fn reg_collision(a: &Rect, b: &Rect) -> bool {
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

#[derive(Copy, Clone)]
enum CollisionObjectType {
	HitBox,
	HurtBox,
	BlockBox, // for if we want to implement it elsewhere
    Hazard,
    Platform,
    Wall,
	Empty,
}

struct PotentialCollision<'a> {
	potentials: (&'a CollisionObject, &'a CollisionObject),
}

impl<'a> for PotentialCollision<'a> {
	fn new(a: &'a CollisionObject, b: &'a CollisionObject) -> PotentialCollision<'a> {
		PotentialCollision{a, b}
	}
}

#[derive(Copy, Clone)]
pub struct CollisionObject {
    obj_type: CollisionObjectType,
	area: u32,
    rect: Rect,
}

trait Area {
	fn area(&self) -> u32;
}

impl Area for Rect {
	fn area(&self) -> u32 {
		self.width()*self.height()
	}
}

impl CollisionObject {
    fn new(obj_type: CollisionObjectType, x: i32, y: i32, width: u32, height: u32) -> CollisionObject {
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

#[derive(Clone)]
pub struct BVHNode<'a> {
	children: Vec<&'a BVHNode<'a>>,
	size: u32, // total area of children
	body: &'a CollisionObject,
	area: Rect,
}

impl<'a> BVHNode<'a> {
	fn new(children: Vec<&'a BVHNode>, body: &'a CollisionObject) -> BVHNode<'a> {
		// get mins and maxes to create over-arching bound
		// at most 2 iterations because there's at most 2 children
		let mut new_node = BVHNode{children: children, body: body, size: 0, area: Rect::new(0, 0, 0, 0)};
		new_node.calculateArea();

		new_node
	}

	fn calculateArea(&mut self) {
		if self.children.len() == 1 {
			self.area = self.children[0].area.clone();
			self.size = self.area.area();
			return
		}
		
		if self.children.len() == 2 {
			// min_x = min(self.children[0].body.rect.left(), self.children[1].body.rect.left());
			// max_x = max(self.children[0].body.rect.left(), self.children[1].body.rect.left());
			// min_y = min(self.children[0].body.rect.bottom(), self.children[1].body.rect.bottom());
			// max_y = max(self.children[0].body.rect.top(), self.children[1].body.rect.top());

			self.area = self.children[0].area.union(self.children[1].area);
			self.size = self.area.area();
			return;
		}

		if !self.body.isEmpty() {
			self.size = self.body.area;
			self.area = self.body.rect.clone();
			return;
		}

		self.size = 0;
		self.area = Rect::new(0, 0, 0, 0);
	}

	fn isLeaf(&self) -> bool {
		if let CollisionObjectType::Empty = self.body.obj_type {false}
		else {true}
	}

	fn overlapsWith(&self, other: &BVHNode) -> bool {
		self.area.has_intersection(other.area)
	}

	fn collidingWith(&'a self, other: &'a BVHNode, potential: &'a mut PotentialCollision, limit: i32) -> i32 {
		if !self.overlapsWith(other) || limit == 0 {return 0;}

		if self.isLeaf() && other.isLeaf() {
			potential.potentials.0 = &'a self.body;
			potential.potentials.1 = other.body;
			return 1;
		}

		if other.isLeaf() || (!self.isLeaf() && self.size >= other.size) {
			let count = self.children[0].collidingWith(&self.children[1], potential, limit);

			if limit > count {
				return count + self.children[1].collidingWith(other, potential, limit);
			}

			else {return count;}
		}

		else {
			let count = self.collidingWith(&self.children[0], potential, limit);

			if limit > count {
				return count + self.collidingWith(&self.children[1], potential, limit);
			}

			else {return count;}
		}
	}

	fn getPotentialCollsions(&self, potential: &mut PotentialCollision, limit: i32) -> i32{
		if self.isLeaf() || limit == 0 {return 0;}
		self.children[0].collidingWith(&self.children[1], potential, limit)
	}

	fn insert(&mut self, new_body: &'a CollisionObject) {
		if self.isLeaf() {
			self.children.push(&BVHNode{area: self.area, body: self.body, size: self.size, children: Vec::new()});
			self.children.push(&BVHNode::new(Vec::new(), new_body));
			self.body = &CollisionObject::empty();
			self.calculateArea();
		}

		else {
			if self.children[0].area.union(new_body.rect).area() < self.children[1].area.union(new_body.rect).area() {
				self.children[0].insert(new_body);
			}
			else {
				self.children[1].insert(new_body);
			}
		}
	}

	fn remove(&mut self, parent: &BVHNode) {
		if parent.children.len() > 0 {

		}
	}
}
