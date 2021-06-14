use sdl2::rect::{Point, Rect};
mod animation;

// EDIT: simplify, if desired
// EDIT: consider updating integers to i32
#[derive(Default)]
struct Fighter {
	// name: Character,
	// state: CharacterState, 
    x_pos: f32, // role into CharacterState
    y_pos: f32, // role into CharacterState
    weight: u16,
    gravity: f32,
    max_fall_speed: u16,
    max_ground_speed: u16,
    run_speed: u16,
    max_air_speed: u16,
    aerial_transition_speed: u16,
    crawl_speed: u16,
    dodge_speed: u16,
    friction: f32,
    static_grip: u16,
    pivot_grip: u16,
    air_resistance: f32,
    air_control: u16,
    jumps: u16,
    jump_height: u16,
    short_hop_height: u16,
    air_jump_height: u16,
    heavy_land_lag: u16,
    wavedash_lag: u16,
    fastfall_multiplier: f32,
    hitstun_elasticity: f32,
    shield_size: u16,
}

// EDIT: add into code here

// Enums 
// defines optional Characters
pub enum Characters {
	Python,
	// Stretch goal: add more
}

// EDIT: make functions public
// EDIT: update getters to function type
// EDIT: for setters, consider updating to start with "set_" and removed "_mut"
// EDIT: should add a new() function to characterAbstract.rs, make this a f(x)
// EDIT: update 'Person' to 'Fighter'
impl Person {
    // Getters
    fn x_pos(&self) -> &String {
        &self.x_pos
    }
    fn y_pos(&self) -> &String {
        &self.y_pos
    }
    fn weight(&self) -> &String {
        &self.weight
    }
    fn gravity(&self) -> &String {
        &self.gravity
    }
    fn max_fall_speed(&self) -> &String {
        &self.max_fall_speed
    }
    fn max_ground_speed(&self) -> &String {
        &self.max_ground_speed
    }
    fn run_speed(&self) -> &String {
        &self.run_speed
    }
    fn max_air_speed(&self) -> &String {
        &self.max_air_speed
    }
    fn aerial_transition_speed(&self) -> &String {
        &self.aerial_transition_speed
    }
    fn crawl_speed(&self) -> &String {
        &self.crawl_speed
    }
    fn dodge_speed(&self) -> &String {
        &self.dodge_speed
    }
    fn friction(&self) -> &String {
        &self.friction
    }
    fn static_grip(&self) -> &String {
        &self.static_grip
    }
    fn pivot_grip(&self) -> &String {
        &self.pivot_grip
    }
    fn air_resistance(&self) -> &String {
        &self.air_resistance
    }
    fn air_control(&self) -> &String {
        &self.air_control
    }
    fn jumps(&self) -> &String {
        &self.jumps
    }
    fn jump_height(&self) -> &String {
        &self.jump_height
    }
    fn short_hop_height(&self) -> &String {
        &self.short_hop_height
    }
    fn air_jump_height(&self) -> &String {
        &self.air_jump_height
    }
    fn heavy_land_lag(&self) -> &String {
        &self.heavy_land_lag
    }
    fn wavedash_lag(&self) -> &String {
        &self.wavedash_lag
    }
    fn fastfall_multiplier(&self) -> &String {
        &self.fastfall_multiplier
    }
    fn hitstun_elasticity(&self) -> &String {
        &self.hitstun_elasticity
    }
    fn shield_size(&self) -> &String {
        &self.shield_size
    }

    // Setters
    fn x_pos_mut(&mut self) -> &mut String {
        &mut self.x_pos
    }
    fn y_pos_mut(&mut self) -> &mut String {
        &mut self.y_pos
    }
    fn weight_mut(&mut self) -> &mut String {
        &mut self.weight
    }
    fn gravity_mut(&mut self) -> &mut String {
        &mut self.gravity
    }
    fn max_fall_speed_mut(&mut self) -> &mut String {
        &mut self.max_fall_speed
    }
    fn max_ground_speed_mut(&mut self) -> &mut String {
        &mut self.max_ground_speed
    }
    fn run_speed_mut(&mut self) -> &mut String {
        &mut self.run_speed
    }
    fn max_air_speed_mut(&mut self) -> &mut String {
        &mut self.max_air_speed
    }
    fn aerial_transition_speed_mut(&mut self) -> &mut String {
        &mut self.aerial_transition_speed
    }
    fn crawl_speed_mut(&mut self) -> &mut String {
        &mut self.crawl_speed
    }
    fn dodge_speed_mut(&mut self) -> &mut String {
        &mut self.dodge_speed
    }
    fn friction_mut(&mut self) -> &mut String {
        &mut self.friction
    }
    fn static_grip_mut(&mut self) -> &mut String {
        &mut self.static_grip
    }
    fn pivot_grip_mut(&mut self) -> &mut String {
        &mut self.pivot_grip
    }
    fn air_resistance_mut(&mut self) -> &mut String {
        &mut self.air_resistance
    }
    fn air_control_mut(&mut self) -> &mut String {
        &mut self.air_control
    }
    fn jumps_mut(&mut self) -> &mut String {
        &mut self.jumps
    }
    fn jump_height_mut(&mut self) -> &mut String {
        &mut self.jump_height
    }
    fn short_hop_height_mut(&mut self) -> &mut String {
        &mut self.short_hop_height
    }
    fn air_jump_height_mut(&mut self) -> &mut String {
        &mut self.air_jump_height
    }
    fn heavy_land_lag_mut(&mut self) -> &mut String {
        &mut self.heavy_land_lag
    }
    fn wavedash_lag_mut(&mut self) -> &mut String {
        &mut self.wavedash_lag
    }
    fn fastfall_multiplier_mut(&mut self) -> &mut String {
        &mut self.fastfall_multiplier
    }
    fn hitstun_elasticity_mut(&mut self) -> &mut String {
        &mut self.hitstun_elasticity
    }
    fn shield_size_mut(&mut self) -> &mut String {
        &mut self.shield_size
    }
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
	pub fn new() -> CharacterState {
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

