use sdl2::rect::{Point, Rect};

// constants based on current sprite sheets 150ppi
const W: u32 = 210;
const H: u32 = 300;
const Y: i32 = 0;

// Enums 
// defines optional Characters
pub enum Characters {
	Python,
	// Stretch goal: add more
}

// enumeration of the various states
pub enum State {
    Idle,
    Walk,
	Jump,
	FJump,
	LPunch,
	LKick,
	HKick,
	Block,
	// Stretch goal: add more
}

// Structs 
// defines the current state of the character
pub struct CharacterState {
	pub character: Characters,
	pub position: Point,
    pub state: State,
	// pub texture: Texture<'a>,
	pub frames_per_state: i32,
	pub current_frame: i32, 
	pub sprite: Rect,
	pub auto_repeat: bool,
	pub next_state: State,	
}

// Implementations
impl CharacterState {
	// initialize
	pub fn new( ) -> CharacterState {
		// current default values
		// Stretch goals: expand to not use default values
		CharacterState {
			character: Characters::Python,
			position: Point::new(0,0),
			state: State::Idle,
//			texture: Texture<'a>,
			frames_per_state: 5,
			current_frame: 0, 
			sprite: Rect::new(0, 0, W, H),
			auto_repeat: true,
			next_state: State::Idle,
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
	pub fn character(&self) 	-> &Characters 	{ &self.character}
	pub fn position(&self)  	-> &Point 		{ &self.position } 
	pub fn state(&self)     	-> &State 		{ &self.state }
	pub fn frames_per_state(&self) -> i32 		{ self.frames_per_state } // for testing
	pub fn current_frame(&self) -> i32 			{ self.current_frame } 
	pub fn sprite(&self) 		-> &Rect 		{ &self.sprite }
	pub fn auto_repeat(&self)	-> bool 		{ self.auto_repeat }
	pub fn next_state(&self) 	-> &State 		{ &self.next_state }
	pub fn x(&self)				-> i32			{ self.position.x() }
	pub fn y(&self)				-> i32			{ self.position.y() }
//	pub fn texture(&self)		-> &Texture		{ &self.texture }
	
	// settters (use to update)
	pub fn set_character(&mut self, c: Characters)	{ self.character = c; }
	pub fn set_position(&mut self, p: Point)		{ self.position = p; }
	pub fn set_state(&mut self, s: State)			{ self.state = s; 
													  self.frames_per_state = get_frame_cnt(self); }
	pub fn set_current_frame(&mut self, i: i32)		{ self.current_frame = self.current_frame + (i % self.frames_per_state); } // need to stay within # of frames
	pub fn set_sprite(&mut self, r: Rect)			{ self.sprite = r; }
	pub fn set_auto_repeat(&mut self, b: bool)		{ self.auto_repeat = b; }
	pub fn set_next_state(&mut self, s: State)		{ self.next_state = s; }
		
}

// Functions to get current file name as string, to use to generate textures
pub fn get_state_filename(s: CharacterState) -> &'static str {
	match s.character {
		Characters::Python =>
			match s.state {
				State::Idle => { return "assets/images/characters/python/idle.png"; },
				State::Walk => { return "assets/images/characters/python/walk.png"; },
				State::Jump => { return "assets/images/characters/python/jump.png"; },
				State::FJump => { return "assets/images/charcters/python/fjump.png"; },
				State::LPunch => { return "assets/images/characters/python/lpunch.png"; },
				State::LKick => { return "assets/images/characters/python/lkick.png"; },
				State::HKick => { return "assets/images/characters/python/hkick.png"; },
				State::Block => { return "assets/images/characters/python/block.png"; },
			},
	}
}

// Gets the rectangle to use for positioning view of sprite
pub fn get_rectangle(f: u32) -> Rect { // current frame
	let x = W*f; // + 0
	return Rect::new(x as i32, Y, W, H);
}

// Gets the numbers of frames per move
pub fn get_frame_cnt(s: &CharacterState) -> i32 {
	match s.character {
		Characters::Python =>
			match s.state {
				State::Idle => { return 5; },
				State::Walk => { return 6; },
				State::Jump => { return 6; },
				State::FJump => { return 7; },
				State::LPunch => { return 3; },
				State::LKick => { return 3; },
				State::HKick => { return 5; },
				State::Block => { return 1; },
			},
	}
}

// get character texture
/* pub fn get_texture(s: CharacterState) -> &Texture {
		match s.character {
		Characters::Python =>
			match s.state {
				State::Idle => { return ; },
				State::Walk => { return ; },
				State::Jump => { return ; },
				State::FJump => { return ; },
				State::LPunch => { return ; },
				State::LKick => { return ; },
				State::HKick => { return ; },
				State::Block => { return ; },
			},
	}
}*/
