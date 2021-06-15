use crate::animation; // to reference sprite State
use crate::input; // use to reference Direction

use sdl2::rect::{Point, Rect};

// Enums 
// defines optional Characters
pub enum Characters {
	Python,
	// Stretch goal: add more
}

// Structs 
// defines the current state of the character
pub struct CharacterState {
	pub position: Point,
    pub state: animation::sprites::State,
	// pub texture: Texture<'a>,
	pub frames_per_state: i32,
	pub current_frame: i32, 
	pub sprite: Rect,
	pub auto_repeat: bool,
	pub direction: input::movement::Direction,
	pub next_state: animation::sprites::State,	
}

// EDIT: update based on States (in sprites)
// EDIT: simplify, if desired
// EDIT: consider updating integers to i32
// EDIT: make fields public (with `pub`)
pub struct Fighter {
	pub name: Characters,
	pub char_state: CharacterState, 
	pub speed: i32,
}

// EDIT: make functions public
// EDIT: update getters to function type
// EDIT: for setters, consider updating to start with "set_" and removed "_mut"
// EDIT: should add a new() function to characterAbstract.rs, make this a f(x)
// EDIT: update 'Person' to 'Fighter'
impl Fighter {
	pub fn new(c: CharacterState) -> Fighter {
		Fighter {
			name: Characters::Python,
			char_state: c,
			speed: 0,
		}
	} 
}



// Implementations
impl CharacterState {
	// initialize
	pub fn new() -> CharacterState {
		// current default values
		// Stretch goals: expand to not use default values
		CharacterState {
			position: Point::new(0,0),
			state: animation::sprites::State::Idle,
//			texture: Texture<'a>,
			frames_per_state: 5,
			current_frame: 0, 
			sprite: Rect::new(0, 0, 210, 300),
			auto_repeat: true,
			next_state: animation::sprites::State::Idle,
			direction: input::movement::Direction::Left,
		}
	}
	
	// update Point position
	pub fn update_position(&mut self, vel: i32, x_bounds: (i32, i32)){
		let x = (self.position.x() + vel).clamp(x_bounds.0, x_bounds.1);
		let current_y = self.position.y();
		self.position = Point::new(x, current_y);
	} 
	
	// convenience f(x)	
	// getters
	pub fn position(&self)  	-> &Point 						{ &self.position } 
	pub fn state(&self)     	-> &animation::sprites::State 	{ &self.state }
	pub fn frames_per_state(&self) -> i32 						{ self.frames_per_state } // for testing
	pub fn current_frame(&self) -> i32 							{ self.current_frame } 
	pub fn sprite(&self) 		-> &Rect 						{ &self.sprite }
	pub fn auto_repeat(&self)	-> bool 						{ self.auto_repeat }
	pub fn next_state(&self) 	-> &animation::sprites::State 	{ &self.next_state }
	pub fn x(&self)				-> i32							{ self.position.x() }
	pub fn y(&self)				-> i32							{ self.position.y() }
	pub fn direction(&self)		-> &input::movement::Direction	{ &self.direction }
//	pub fn texture(&self)		-> &Texture		{ &self.texture }

	
	// settters (use to update)
	pub fn set_position(&mut self, p: Point)						{ self.position = p; }
	pub fn set_state(&mut self, s: animation::sprites::State)		{ self.state = s; 
																	  self.frames_per_state = animation::sprites::get_frame_cnt(self); }
	pub fn set_current_frame(&mut self, i: i32)						{ self.current_frame = self.current_frame + (i % self.frames_per_state); } // need to stay within # of frames
	pub fn set_sprite(&mut self, r: Rect)							{ self.sprite = r; }
	pub fn set_auto_repeat(&mut self, b: bool)						{ self.auto_repeat = b; }
	pub fn set_next_state(&mut self, s: animation::sprites::State)	{ self.next_state = s; }
	pub fn set_direction(&mut self, d: input::movement::Direction)	{ self.direction = d; }
		
}

