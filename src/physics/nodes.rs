#![allow(non_snake_case)]
use sdl2::rect::Rect;
use std::cell::{self, RefCell};
use std::fmt;
use std::rc::{Rc, Weak};
use std::ops::Deref as Df;
use std::ops::DerefMut as Dfm;
use crate::physics::collisions::*;
use crate::physics::vecmath::PhysVec;
use crate::physics::particle::Particle;

// #[derive(Debug)]
pub struct NodeRef<T>(pub Rc<RefCell<Node<T>>>);

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;
pub type BoxRef<T> = Option<Box<T>>;

impl<T> PartialEq for NodeRef<T> {
	fn eq(&self, other: &NodeRef<T>) -> bool {
		is(&self.0, &other.0)
	}
}


pub fn is<T>(a: &Rc<T>, b: &Rc<T>) -> bool {
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
        f.debug_struct("NodeRef")
		.field("\nparent", &self.0.borrow().parent)
		.field("\nleft", &self.0.borrow().left)
		.field("\nright", &self.0.borrow().right)
		.field("\nbv", &self.0.borrow().bv)
		.finish()
    }
}

pub trait Refer<T> {
	fn get(&self) -> std::cell::Ref<Node<T>>;
    fn getMut(&self) -> std::cell::RefMut<Node<T>>;
}

impl<T> Refer<T> for NodeRef<T> {
	fn get(&self) -> std::cell::Ref<Node<T>> {
		self.0.as_ref().borrow()
	}
	fn getMut(&self) -> std::cell::RefMut<Node<T>> {
		self.0.as_ref().borrow_mut()
	}
}

/// Wraps a `std::cell::Ref` for a node’s data.
pub struct Ref<'a, T: 'a> {
    _ref: cell::Ref<'a, Node<T>>
}

/// Wraps a `std::cell::RefMut` for a node’s data.
pub struct RefMut<'a, T: 'a> {
    _ref: cell::RefMut<'a, Node<T>>
}

trait Deref<T> {
    fn deref(&self) -> std::cell::Ref<T>;
}

impl<'a, T> Deref<T> for Ref<'a, T> {
    fn deref(&self) -> std::cell::Ref<T> { self._ref.bv.as_ref().unwrap().borrow() }
}

trait DerefMut<T> {
    fn deref_mut(&self) -> std::cell::RefMut<T>;
}

// impl<'a, T> DerefMut<T> for RefMut<'a, T> {
//     fn deref_mut(&self) -> std::cell::RefMut<T> { self._ref.bv.as_mut().unwrap().borrow_mut() }
// }

trait Unbox<T> {
	fn unbox<'a> (&'a self) -> &'a mut T;
}

impl NodeRef<CollisionObject> {
	pub fn new(bv: CollisionObject) -> Self {
		let node = NodeRef(Rc::new(RefCell::new(
			Node{
				parent: None,
				left: None,
				right: None,
				bv: boxUp(bv),
				area: Rect::new(0,0,0,0)
			}
		)));

		node.calculateArea();
		node
	}

    pub fn replace(&self, other: &NodeRef<CollisionObject>) {
		// println!("\nReplacing {:?}", other);
		if true {
			let l = other.0.borrow().left.clone();
			let r = other.0.borrow().right.clone();
			let bv = other.0.borrow().bv.clone();
			let mut pm = self.getMut();
			pm.left = l;
			pm.right = r;
			pm.bv = bv;

			if pm.isLeaf() {
				pm.bv.as_ref().unwrap().borrow_mut().noderef = Some(Rc::downgrade(&self.0));
			}
			else {
				pm.left.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.0));
				pm.right.as_ref().unwrap().borrow_mut().parent = Some(Rc::downgrade(&self.0));
			}
		}
		// println!("After replace: \n{:?}", self);
        self.calculateArea();
    }

	pub fn getParent(&self) -> Option<NodeRef<CollisionObject>> {
		match &self.get().parent {
       		Some(p) => Some(NodeRef(p.upgrade().unwrap())),
			None => None
		}
	}

    pub fn hasParent(&self) -> bool {
        match &self.getParent() {
            Some(p) => true,
            None => false
        }
    }

	pub fn calculateArea(&self) {
        let new_area: Rect;
		// println!("Area of {:?}", self);
		if self.get().isLeaf() {
            new_area = self.borrow().deref().rect.clone(); // area = area of collision object if node has no children but points to object
		}

		else {
            new_area = self.getRightChild().get().area.union(self.getLeftChild().get().area); // area = smallest bounding box around both children if node has two children
        }
        self.getMut().area = new_area;

        if self.hasParent() {self.getParent().unwrap().calculateArea()}
	}

	pub fn borrow(&self) -> Ref<CollisionObject> {
		Ref {_ref: self.0.borrow()}
	}

	pub fn borrowbv(&self) -> std::cell::RefCell<CollisionObject> {
		self.0.borrow().bv.as_ref().unwrap().clone()
	}

	pub fn borrow_mut(&self) -> RefMut<CollisionObject> {
		RefMut {_ref: self.0.borrow_mut()}
	}

	pub fn getLeftChild(&self) -> NodeRef<CollisionObject> {
		NodeRef(self.get().left.as_ref().unwrap().clone())
	}

	pub fn getLeftRef(&self) -> NodeRef<CollisionObject> {
		if self.getLeftChild().get().isLeaf() {
			self.getLeftChild().get().bv.as_ref().unwrap().borrow().getNodeRef().unwrap()
		}
		else {
			self.getLeftChild()
		}
	}

	pub fn getRightChild(&self) -> NodeRef<CollisionObject> {
		NodeRef(self.get().right.as_ref().unwrap().clone())
	}

	pub fn getRightRef(&self) -> NodeRef<CollisionObject> {
		if self.getRightChild().get().isLeaf() {
			self.getRightChild().get().bv.as_ref().unwrap().borrow().getNodeRef().unwrap()
		}
		else {
			self.getRightChild()
		}
	}

	pub fn getPotentialCollisions(&self, potential: &mut Vec<ParticleContact>, limit: i32) -> i32{
		if self.get().isLeaf() || limit == 0 {return 0;}
		self.getLeftRef().collidingWith(&self.getRightRef(), potential, limit)
	}

	pub fn collidingWith(&self, other: &NodeRef<CollisionObject>, potential: &mut Vec<ParticleContact>, limit: i32) -> i32 {
		// println!("self:\n {:?}, \nother:\n {:?}", self, other);
		// return if there's no overlap
		let intersection = self.get().area.intersection(other.get().area.clone());
		if intersection.is_none() || limit == 0 {0}

		// collision if both are leaves
		else if self.get().isLeaf() && other.get().isLeaf() {
			// println!("\n//////self:\n {:?}, \n///////other:\n {:?}", self, other);
			let a = self.get().bv.as_ref().unwrap().clone();
			let b = other.get().bv.as_ref().unwrap().clone();
			let types = (a.borrow().obj_type, b.borrow().obj_type);
			match types {
				// (CollisionObjectType::Platform, _) | (_, CollisionObjectType::Platform) => (),
				_ => {
					let interpenetration = PhysVec::new(intersection.unwrap().width() as f32, intersection.unwrap().height() as f32);
					let dif = a.borrow().particle.borrow().position.sub(&b.borrow().particle.borrow().position);
					let collision_normal = dif.normalize();
					// println!("\nmagnitude: {}, normal: {:?}, interpenetration: {:?}", dif.magnitude(), collision_normal, interpenetration);
					potential.push(ParticleContact::new(a, b, collision_normal, 1.0, interpenetration));
				},
			}
			1
		}

		// either descend into node that is not a leaf or the node that is larger
		else if other.get().isLeaf() || (!self.get().isLeaf() && self.get().area.area() >= other.get().area.area()) {
			let mut count = self.getLeftRef().collidingWith(&other, potential, limit);
			count += self.getRightRef().collidingWith(&other, potential, limit);
			count += self.getPotentialCollisions(potential, limit);
			count += other.getPotentialCollisions(potential, limit);
			count
		}

		else {
			let mut count = self.collidingWith(&other.getLeftRef(), potential, limit);
			count += self.collidingWith(&other.getRightRef(), potential, limit);
			count += other.getPotentialCollisions(potential, limit);
			count += self.getPotentialCollisions(potential, limit);
			count
		}
	}

	pub fn insert(&self, new_obj: CollisionObject) -> RefCell<CollisionObject> {
		// println!("\nInserting at {:?}\n", self);
        let leaf = {self.get().isLeaf()};
		if leaf {
            if leaf { // to deal w lifetime stuff
            let mut sm = self.getMut();
			sm.left = Some(Rc::new(RefCell::new(Node::new(Some(Rc::downgrade(&self.0)), sm.bv.take().unwrap().into_inner()))));
			sm.right = Some(Rc::new(RefCell::new(Node::new(Some(Rc::downgrade(&self.0)), new_obj))));
            sm.bv.take();
            }
            self.calculateArea();
            // (Rc::downgrade(&Rc::new(self.getLeftChild())), Rc::downgrade(&Rc::new(self.getRightChild())))
			self.getRightChild().get().bv.as_ref().unwrap().borrow_mut().noderef = Some(Rc::downgrade(self.get().right.as_ref().unwrap()));
			self.getLeftChild().get().bv.as_ref().unwrap().borrow_mut().noderef = Some(Rc::downgrade(self.get().left.as_ref().unwrap()));
			// println!("Inserted\n {:?}", self.getRightChild());
            self.getRightChild().borrowbv()
		}

		else {
			let size0 = self.getLeftChild().get().area.area();
			let size1 = self.getRightChild().get().area.area();
			if size0 <= size1 {
				return self.getLeftChild().insert(new_obj);
			}
			else {
				return self.getRightChild().insert(new_obj);
			}
		}
	}

	pub fn remove(&self) {
		if let Some(parent) = self.getParent() {
			let mut left = parent.getLeftChild();
			let mut right = parent.getRightChild();
			let is_left = is(&left.0, &self.0);
			if is_left {
				// println!("replacing with right");
                parent.replace(&parent.getRightChild());
            }
			else {
				// println!("replacing with left");
				parent.replace(&parent.getLeftChild());
            }
		}
		self.getMut().detatch();
	}
}
