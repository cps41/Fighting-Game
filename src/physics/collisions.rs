use sdl2::rect::Rect;

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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

// Link type for child nodes
type Link<T> = Option<Box<T>>;

trait Refer<T> {
	fn refer<'a>(&'a self) -> &'a T;
}

impl<T> Refer<T> for Link<T> {
	fn refer<'a>(&'a self) -> &'a T {
		self.as_ref().unwrap()
	}
}

#[derive(Clone, Debug)]
pub struct BVHNode {
	children: (Link<BVHNode>, Link<BVHNode>),
	obj: Link<CollisionObject>,
	area: Rect,
}

impl BVHNode {
	fn new(children: (Link<BVHNode>, Link<BVHNode>), obj: Link<CollisionObject>, area: Rect) -> BVHNode {
		let mut node = BVHNode{children: children, obj: obj, area: area};
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
			self.calculateArea();
		}

		else {
			if self.children.0.refer().area.union(new_obj.rect).area() < self.children.1.refer().area.union(new_obj.rect).area() {
				self.children.0.as_mut().unwrap().insert(new_obj);
			}
			else {
				self.children.1.as_mut().unwrap().insert(new_obj);
			}
		}
	}

	fn remove(&mut self, parent: Link<BVHNode>) {
		
	}
}

#[cfg(test)]
mod test {
	use super::CollisionObject;
	use super::CollisionObjectType;
	use super::check_collision;

	#[test]
	fn testCollide() {
		let c1 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 10, 20);
		let c2 = CollisionObject::new(CollisionObjectType::Hazard, 28, 20, 10, 20);

		assert_eq!(check_collision(&c1, &c2), true);
	}	
}
