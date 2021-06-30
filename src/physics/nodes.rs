#![allow(non_snake_case)]
use sdl2::rect::Rect;
use std::cell::{self, RefCell};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use crate::physics::collisions::{Node, CollisionObject, Area, PotentialCollision};

pub struct NodeRef<T>(Rc<RefCell<Node<T>>>);

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;
pub type BoxRef<T> = Option<Box<T>>;

fn boxUp<T>(data: T) -> BoxRef<T>{
	Some(Box::new(data))
}

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
        fmt::Debug::fmt(&*self.0.borrow(), f)
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

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &T { &self._ref.bv.as_deref().unwrap() }
}

// impl<'a, T> Deref for RefMut<'a, T> {
//     type Target = T;
//     fn deref(&self) -> &T { &self._ref.bv.as_deref_mut().unwrap() }
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

    pub fn replace(&self, bv: &mut Option<Box<CollisionObject>>) {
        std::mem::swap(&mut self.0.borrow_mut().deref_mut().bv, bv);
        self.getMut().left.take();
        self.getMut().right.take();
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
		if self.get().isLeaf() {
            new_area = self.borrow().rect.clone(); // area = area of collision object if node has no children but points to object
		}

		else {
            new_area = self.getRightChild().get().area.union(self.getLeftChild().get().area); // area = smallest bounding box around both children if node has two children
        }
        self.getMut().area = new_area;

        if self.hasParent() {self.getParent().unwrap().calculateArea()}
	}

	pub fn overlapsWith(&self, other: NodeRef<CollisionObject>) -> bool {
		self.get().area.has_intersection(other.get().area.clone())
	}

	pub fn borrow(&self) -> Ref<CollisionObject> {
		Ref {_ref: self.0.borrow()}
	}

	pub fn borrow_mut(&self) -> RefMut<CollisionObject> {
		RefMut {_ref: self.0.borrow_mut()}
	}

	pub fn getLeftChild(&self) -> NodeRef<CollisionObject> {
		NodeRef(self.get().left.as_ref().unwrap().clone())
	}

	pub fn getRightChild(&self) -> NodeRef<CollisionObject> {
		NodeRef(self.get().right.as_ref().unwrap().clone())
	}

	pub fn collidingWith(& self, other: NodeRef<CollisionObject>, potential: &mut Option<PotentialCollision>, limit: i32) -> i32 {
		if !self.overlapsWith(other.clone()) || limit == 0 {return 0;}

		if self.get().isLeaf() && other.get().isLeaf() {
			potential.as_mut().unwrap().0 = self.borrow().deref().clone();
			potential.as_mut().unwrap().1 = other.borrow().deref().clone();
			return 1;
		}

		if other.get().isLeaf() || (!self.get().isLeaf() && self.get().area.area() >= other.get().area.area()) {
			let count = self.getLeftChild().collidingWith(self.getRightChild(), potential, limit);

			if limit > count {
				return count + self.getRightChild().collidingWith(other, potential, limit);
			}

			else {return count;}
		}

		else {
			let count = self.collidingWith(self.getLeftChild(), potential, limit);

			if limit > count {
				return count + self.collidingWith(self.getLeftChild(), potential, limit);
			}

			else {return count;}
		}
	}

	pub fn getPotentialCollsions(&self, potential: &mut Option<PotentialCollision>, limit: i32) -> i32{
		if self.get().isLeaf() || limit == 0 {return 0;}
		self.getLeftChild().collidingWith(self.getRightChild(), potential, limit)
	}

	pub fn insert(&self, new_obj: CollisionObject) -> (NodeRef<CollisionObject>, NodeRef<CollisionObject>){
        let leaf = {self.get().isLeaf()};
		if leaf {
            let mut sm = self.getMut();
			sm.left = Some(Rc::new(RefCell::new(Node::new(Some(Rc::downgrade(&self.0)), sm.bv.as_deref().unwrap().clone()))));
			sm.right = Some(Rc::new(RefCell::new(Node::new(Some(Rc::downgrade(&self.0)), new_obj))));
            sm.bv.take();
		}

		else {
			let size0 = self.getLeftChild().get().area.area();
			let size1 = self.getRightChild().get().area.area();
			if size0 <= size1 {
				self.getLeftChild().insert(new_obj);
			}
			else {
				self.getRightChild().insert(new_obj);
			}
		}
		self.calculateArea();
        (self.getLeftChild().clone(), self.getRightChild().clone())
	}

	pub fn remove(&mut self) {
		if let Some(parent) = self.getParent() {
			if is(&parent.getLeftChild().0, &self.0) {
                parent.replace(&mut parent.getRightChild().0.borrow_mut().deref_mut().bv);
            }
			else {
                parent.replace(&mut parent.getLeftChild().0.borrow_mut().deref_mut().bv);
            }
		}

		// self.0.borrow_mut().detatch();
	}
}