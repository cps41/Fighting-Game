#![allow(non_snake_case)]
use sdl2::rect::Rect;
use std::cell::{self, RefCell};
use std::fmt;
use std::rc::{Rc, Weak};
use std::ops::Deref as Df;
use std::ops::DerefMut as Dfm;
use crate::physics::collisions::*;

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
        // std::mem::swap(self.0.deref().replace(), &mut otherbv.borrow_mut().deref_mut());
        self.getMut().bv = other.getMut().bv.take();
		self.get().bv.as_ref().unwrap().borrow_mut().noderef = Some(Rc::downgrade(&self.0));
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
            new_area = self.borrow().deref().rect.clone(); // area = area of collision object if node has no children but points to object
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

	pub fn borrowbv(&self) -> std::cell::RefCell<CollisionObject> {
		self.0.borrow().bv.as_ref().unwrap().clone()
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

	pub fn collidingWith(& self, other: NodeRef<CollisionObject>, potential: &mut Vec<PotentialCollision>, limit: i32) -> i32 {
		if !self.overlapsWith(other.clone()) || limit == 0 {return 0;}

		if self.get().isLeaf() && other.get().isLeaf() {
			potential.push((self.borrow().deref().clone(), other.borrow().deref().clone()));
			return 1;
		}

		if other.get().isLeaf() || (!self.get().isLeaf() && self.get().area.area() >= other.get().area.area()) {
			let count = self.getLeftChild().collidingWith(self.getRightChild(), potential, limit);

			if limit > count {
				return count + self.getRightChild().collidingWith(other, potential, limit);
			}

			else {return count;}
		}

		else if !self.get().isLeaf() {
			let count = self.collidingWith(self.getLeftChild(), potential, limit);

			if limit > count {
				return count + self.collidingWith(self.getRightChild(), potential, limit);
			}

			else {return count;}
		}

		else {return 0;}
	}

	pub fn getPotentialCollsions(&self, potential: &mut Vec<PotentialCollision>, limit: i32) -> i32{
		if self.get().isLeaf() || limit == 0 {return 0;}
		self.getLeftChild().collidingWith(self.getRightChild(), potential, limit)
	}

	pub fn insert(&self, new_obj: CollisionObject) -> RefCell<CollisionObject> {
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
			self.getRightChild().get().bv.as_ref().unwrap().borrow_mut().noderef = Some(Rc::downgrade(&self.0));
			// println!("Inserted {:?}", self.getRightChild());
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
		// // println!("Removing {:?}", self);
		if let Some(parent) = self.getParent() {
			if is(&parent.getLeftChild().0, &self.0) {
                parent.replace(&parent.getRightChild());
            }
			else {
                parent.replace(&parent.getLeftChild());
            }
		}

		// self.0.borrow_mut().detatch();
	}
}