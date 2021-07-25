use core::cell::RefCell;
use crate::animation; // to reference sprite State
use crate::animation::sprites::State;
use crate::input; // use to reference Direction

use sdl2::rect::{Rect};
use sdl2::render::Texture;
use std::collections::HashMap;
use std::rc::Rc;
use crate::physics::collisions::*;
use crate::physics::vecmath::*;
use crate::physics::nodes::*;
use crate::physics::particle::*;
use crate::view::globals::*;

// Enums
// defines optional Characters
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Characters {
	Python,
	Java,
	// Stretch goal: add more
}

// Structs
// defines the current state of the character
pub struct CharacterState {
	// pub position: RefCell<Particle>,
	pub particle: Rc<RefCell<Particle>>,
    pub state: animation::sprites::State,
	pub frames_per_state: i32,
	pub frame_count:	i32,
	pub current_frame: i32,
	pub sprite: Rect,
	pub auto_repeat: bool,
	pub direction: input::movement::Direction,
	pub next_state: animation::sprites::State,
	pub hitbox: Option<RefCell<CollisionObject>>,
	pub hurtbox: Option<RefCell<CollisionObject>>,
	pub blockbox: Option<RefCell<CollisionObject>>,
}
//self.current_frame = (self.current_frame + 1) % self.frames_per_state; }

// EDIT: consider updating integers to f64
pub struct Fighter<'t> {
	pub name: Characters,
	pub char_state: CharacterState,
	pub health: i32,
	pub speed: i32,
    pub weight: i32,
    pub gravity: f32,
    pub max_fall_speed: i32,
    pub walk_speed: i32,
    pub run_speed: i32,
    pub max_air_speed: i32,
    pub aerial_transition_speed: i32,
    pub crawl_speed: i32,
    pub dodge_speed: i32,
    pub friction: f32,
    pub static_grip: i32,
    pub pivot_grip: i32,
    pub air_resistance: f32,
    pub air_control: i32,
    pub jumps: i32,
    pub jump_height: i32,
    pub short_hop_height: i32,
    pub air_jump_height: i32,
    pub heavy_land_lag: i32,
    pub fastfall_multiplier: f32,
    pub shield_size: i32,
  	pub textures: HashMap<animation::sprites::State, Texture<'t>>,
}

impl <'t> Fighter <'t> {
	pub fn new (c: CharacterState) -> Fighter<'t> {
		Fighter {
			name: Characters::Python,
			health: 270,
			char_state: c,
			speed: 20, // arbitrary #
			weight: 180,
			gravity: -9.8,
			max_fall_speed: 20,
			walk_speed: 500,
			run_speed: 15,
			max_air_speed: 5,
			aerial_transition_speed: 3,
			crawl_speed: 3,
			dodge_speed: 5,
			friction: -0.1,
			static_grip: 20,
			pivot_grip: 25,
			air_resistance: -0.1,
			air_control: 5,
			jumps: 2,
			jump_height: 10,
			short_hop_height: 5,
			air_jump_height: 7,
			heavy_land_lag: 2,
			fastfall_multiplier: 1.25,
			shield_size: 3,
      		textures: HashMap::new(),
		}
	}

	// Getters
    pub fn weight(&self) -> &i32 {&self.weight}
    pub fn gravity(&self) -> &f32 {&self.gravity}
    pub fn max_fall_speed(&self) -> &i32 {&self.max_fall_speed}
	pub fn get_health(&self) -> &i32 {&self.health}
    pub fn walk_speed(&self) -> &i32 {&self.walk_speed}
    pub fn run_speed(&self) -> &i32 {&self.run_speed}
    pub fn max_air_speed(&self) -> &i32 {&self.max_air_speed}
    pub fn aerial_transition_speed(&self) -> &i32 {&self.aerial_transition_speed}
    pub fn crawl_speed(&self) -> &i32 {&self.crawl_speed}
    pub fn dodge_speed(&self) -> &i32 {&self.dodge_speed}
    pub fn friction(&self) -> &f32 {&self.friction}
    pub fn static_grip(&self) -> &i32 {&self.static_grip}
    pub fn pivot_grip(&self) -> &i32 {&self.pivot_grip}
    pub fn air_resistance(&self) -> &f32 {&self.air_resistance}
    pub fn air_control(&self) -> &i32 {&self.air_control}
    pub fn jumps(&self) -> &i32 {&self.jumps}
    pub fn jump_height(&self) -> &i32 {&self.jump_height}
    pub fn short_hop_height(&self) -> &i32 {&self.short_hop_height}
    pub fn air_jump_height(&self) -> &i32 {&self.air_jump_height}
    pub fn heavy_land_lag(&self) -> &i32 {&self.heavy_land_lag}
    pub fn fastfall_multiplier(&self) -> &f32 {&self.fastfall_multiplier}
    pub fn shield_size(&self) -> &i32 {&self.shield_size}

	pub fn textures(&self) -> &Texture<'t> {
		match &self.textures.get(&self.char_state.state) {
			Some(texture) => texture,
			None => panic!("Texture issue in fighter"),
		}
	}

	pub fn add_texture(&mut self, s: animation::sprites::State, t: Texture<'t>) {
            &self.textures.insert(s, t);
	}

	// update Particle position
	pub fn update_position(&mut self, force: &PhysVec) {
		let mut scaled = force.clone();
		scaled.dot_replace(1.0/0.0002645833);
		self.char_state.particle.borrow_mut().add_force(&scaled);
		self.char_state.particle.borrow_mut().integrate(FRAME_RATE as f32);
	}

	pub fn inflict_damage (&mut self, damage: i32) {
		self.health = self.health - damage;
		if self.health < 0 {
			println!("Uh Oh, we're dead");
			self.health = 0;
		}
	}
	pub fn reset_health (&mut self) { self.health = 100; }
	pub fn kill_player_test (&mut self) { self.inflict_damage(100); }


    // Setters
    pub fn set_weight(&mut self) -> &mut i32 {&mut self.weight}
    pub fn set_gravity(&mut self) -> &mut f32 {&mut self.gravity}
	pub fn set_health(&mut self) -> &mut i32 {&mut self.health}
    pub fn set_max_fall_speed(&mut self) -> &mut i32 {&mut self.max_fall_speed}
    pub fn set_walk_speed(&mut self) -> &mut i32 {&mut self.walk_speed}
    pub fn set_run_speed(&mut self) -> &mut i32 {&mut self.run_speed}
    pub fn set_max_air_speed(&mut self) -> &mut i32 {&mut self.max_air_speed}
    pub fn set_aerial_transition_speed(&mut self) -> &mut i32 {&mut self.aerial_transition_speed}
    pub fn set_crawl_speed(&mut self) -> &mut i32 {&mut self.crawl_speed}
    pub fn set_dodge_speed(&mut self) -> &mut i32 {&mut self.dodge_speed}
    pub fn set_friction(&mut self) -> &mut f32 {&mut self.friction}
    pub fn set_static_grip(&mut self) -> &mut i32 {&mut self.static_grip}
    pub fn set_pivot_grip(&mut self) -> &mut i32 {&mut self.pivot_grip}
    pub fn set_air_resistance(&mut self) -> &mut f32 {&mut self.air_resistance}
    pub fn set_air_control(&mut self) -> &mut i32 {&mut self.air_control}
    pub fn set_jumps(&mut self) -> &mut i32 {&mut self.jumps}
    pub fn set_jump_height(&mut self) -> &mut i32 {&mut self.jump_height}
    pub fn set_short_hop_height(&mut self) -> &mut i32 {&mut self.short_hop_height}
    pub fn set_air_jump_height(&mut self) -> &mut i32 {&mut self.air_jump_height}
    pub fn set_heavy_land_lag(&mut self) -> &mut i32 {&mut self.heavy_land_lag}
    pub fn set_fastfall_multiplier(&mut self) -> &mut f32 {&mut self.fastfall_multiplier}
    pub fn set_shield_size(&mut self) -> &mut i32 {&mut self.shield_size}
} // close Fighter impl

// Implementations
impl CharacterState {
	// initialize
	pub fn new() -> CharacterState {
		// current default values
		// Stretch goals: expand to not use default values
		let position = Particle::new(PhysVec::new(0f32,0f32), 0.01, 180f32, 270);
		CharacterState {
			// position: RefCell::new(position.clone()),
			particle: Rc::new(RefCell::new(position.clone())),
			state: animation::sprites::State::Idle,
			frames_per_state: 30,
			current_frame: 0,
			frame_count:	0,
			sprite: Rect::new(0, 0, 210, 300),
			auto_repeat: true,
			next_state: animation::sprites::State::Idle,
			direction: input::movement::Direction::Up,
			hitbox: None,
			hurtbox: None,
			blockbox: None,
		}
	}

    // advancing frames
    pub fn advance_frame(&mut self) {
		self.frame_count = (self.frame_count + 1) % (self.frames_per_state+1);

    	match self.state{
    		animation::sprites::State::Idle =>{
    			if self.frame_count < 7{
    				self.current_frame = 0;
    			}else if self.frame_count < 13 {
    				self.current_frame = 1;
    			}else if self.frame_count < 19 {
    				self.current_frame = 2;
    			}else if self.frame_count < 24 {
    				self.current_frame = 3;
    			}else{
    				self.current_frame = 4;
    			}
    		}
    		animation::sprites::State::Walk =>{
    			if self.frame_count < 6 {
 					self.current_frame = 0;
    			}else if self.frame_count < 11{
 					self.current_frame = 1;
    			}else if self.frame_count < 16{
 					self.current_frame = 2;
    			}else if self.frame_count < 21{
 					self.current_frame = 3;
    			}else if self.frame_count < 26{
 					self.current_frame = 4;
    			}else{
 					self.current_frame = 5;
    			}
    		}
    		animation::sprites::State::Jump =>{
    			if self.frame_count < 6{
    				self.current_frame = 0;
    			}else if self.frame_count < 11 {
    				self.current_frame = 1;
    			}else if self.frame_count < 16 {
    				self.current_frame = 2;
    			}else if self.frame_count < 21 {
    				self.current_frame = 3;
    			}else if self.frame_count < 26{
    				self.current_frame = 4;
    			}else{
    				self.current_frame = 5;
    			}
    		}
    		animation::sprites::State::FJump =>{
    			if self.frame_count < 7 {
 					self.current_frame = 0;
    			}else if self.frame_count < 13{
 					self.current_frame = 1;
    			}else if self.frame_count < 19{
 					self.current_frame = 2;
    			}else if self.frame_count < 25{
 					self.current_frame = 3;
    			}else if self.frame_count < 31{
 					self.current_frame = 4;
    			}else if self.frame_count < 37{
 					self.current_frame = 5;
    			}else{
    				self.current_frame = 6;
    			}
    		}
    		animation::sprites::State::LPunch =>{
    			if self.frame_count < 6 {
    				self.current_frame = 0;
    			}else if self.frame_count < 11 {
    				self.current_frame = 1;
    			}else if self.frame_count <= 17 {
    				self.current_frame = 2;
    			}
    		}
    		animation::sprites::State::LKick =>{
    			if self.frame_count < 8 {
    				self.current_frame = 0;
    			}else if self.frame_count < 14  {
    				self.current_frame = 1;
    			}else{
    				self.current_frame = 2;
    			}
    		}
    		animation::sprites::State::HKick =>{
    			if self.frame_count < 6 {
    				self.current_frame = 0;
    			}else if self.frame_count < 10{
    				self.current_frame = 1;
    			}else if self.frame_count < 14{
    				self.current_frame = 2;
    			}else if self.frame_count < 21{
    				self.current_frame = 3;
    			}else if self.frame_count <= 35{
    				self.current_frame = 4
    			}
    		}
    		animation::sprites::State::Block =>{}
    	}
    	//println!("Frame count is: {}    Frame Per State is: {}    Current Frame is: {}    State is: {:?}",
    	//	self.frame_count, self.frames_per_state, self.current_frame, self.state);


    }
	// convenience f(x)
	// getters
	pub fn position(&self)  	-> Particle 					{ self.particle.borrow().clone() }
	pub fn state(&self)     	-> &animation::sprites::State 	{ &self.state }
	pub fn frames_per_state(&self) -> i32 						{ self.frames_per_state } // for testing
	pub fn current_frame(&self) -> i32 							{ self.current_frame }
	pub fn sprite(&self) 		-> &Rect 						{ &self.sprite }
	pub fn auto_repeat(&self)	-> bool 						{ self.auto_repeat }
	pub fn next_state(&self) 	-> &animation::sprites::State 	{ &self.next_state }
	pub fn x(&self)				-> i32							{ self.particle.borrow().position.x as i32 }
	pub fn y(&self)				-> i32							{ self.particle.borrow().position.y as i32 }
	pub fn health(&self)		-> i32 							{ self.particle.borrow().health}
	pub fn velocity(&self)		-> (f32, f32)					{ self.particle.borrow().velocity.raw() }
	pub fn acceleration(&self)		-> (f32, f32)					{ self.particle.borrow().acceleration.raw() }
	pub fn direction(&self)		-> &input::movement::Direction	{ &self.direction }

	// settters (use to update)
	// pub fn set_position(&mut self, p: PhysVec)						{ self.position.borrow().position.replace(&p); }
	pub fn set_state(&mut self, s: animation::sprites::State)		{ self.state = s;
																	  self.frames_per_state = animation::sprites::get_frame_cnt(self);
																	  // println!("s: {:?}, cf: {}", self.state, self.current_frame);
																	}
	pub fn set_current_frame(&mut self, i: i32)						{ self.current_frame = (self.current_frame + i) % self.frames_per_state; } // need to stay within # of frames
	pub fn reset_frame_count(&mut self)								{ self.frame_count = 0}
	pub fn set_sprite(&mut self, r: Rect)							{ self.sprite = r; }
	pub fn set_auto_repeat(&mut self, b: bool)						{ self.auto_repeat = b; }
	pub fn set_next_state(&mut self, s: animation::sprites::State)	{ self.next_state = s; }
	pub fn set_direction(&mut self, d: input::movement::Direction)	{ self.direction = d; }
	pub fn reset_current_frame(&mut self)							{ self.current_frame = 0;  self.frame_count = 0;}

	pub fn isMoving(&self) -> bool {
		if self.state == animation::sprites::State::Jump || self.state == animation::sprites::State::FJump
		|| self.state == animation::sprites::State::LPunch || self.state == animation::sprites::State::LKick
		|| self.state == animation::sprites::State::HKick {
			true
		} else {
			false
		}
	}
	pub fn remove(link: &mut Option<RefCell<CollisionObject>>) {
		link.take().map(|l| {
			l.borrow().getNodeRef().map(|n| {
				// println!("\nremoving {:?}\n", n);
				n.remove()
			});
		});
	}
	pub fn insert_hit_box(&mut self, bvh: &BVHierarchy) {
		// println!("inserting hit box...");
		CharacterState::remove(&mut self.hitbox);
		let mut vel_particle = self.particle.clone();
		vel_particle.borrow_mut().velocity.x = 50.0;
		vel_particle.borrow_mut().velocity.y = 0.0;
		self.hitbox = Some(bvh.insert(
			CollisionObject {
				obj_type: CollisionObjectType::HitBox, 
				area: SPRITE_W as u32 * SPRITE_H/2,
				rect: Rect::new(self.x()+W_OFFSET+SPRITE_W as i32/2, self.y()+H_OFFSET, SPRITE_W as u32, SPRITE_H/2),
				noderef: None,
				particle: vel_particle,
			}
		));
	}
	pub fn insert_hurt_box(&mut self, bvh: &BVHierarchy) {
		// println!("inserting hurt box...");
		CharacterState::remove(&mut self.hurtbox);
		self.hurtbox = Some(bvh.insert(
			CollisionObject::new(
				CollisionObjectType::HurtBox, self.x()+W_OFFSET, self.y()+H_OFFSET, SPRITE_W, SPRITE_H, self.particle.clone())
		));
	}
	pub fn insert_block_box(&mut self, bvh: &BVHierarchy) {
		// println!("inserting block box...");
		CharacterState::remove(&mut self.blockbox);
		self.blockbox = Some(bvh.insert(
			CollisionObject::new(
				CollisionObjectType::BlockBox, self.x()+W_OFFSET, self.y()+H_OFFSET, SPRITE_W, SPRITE_H, self.particle.clone())
		));
	}
	pub fn update_bounding_boxes(&mut self, bvh: &BVHierarchy) {
		// println!("updating...");
		// println!("\nUpdating Bounding Boxes {:?}", bvh.head);
		match &self.state {
			State::Block => {
				CharacterState::remove(&mut self.hitbox);
				CharacterState::remove(&mut self.hurtbox);
				self.insert_block_box(&bvh);
			},
			State::LPunch | State::HKick | State::LKick => {
				CharacterState::remove(&mut self.blockbox);
				CharacterState::remove(&mut self.hurtbox);
				self.insert_hit_box(&bvh);
			},
			_ => {
				CharacterState::remove(&mut self.hitbox);
				CharacterState::remove(&mut self.blockbox);
				self.insert_hurt_box(&bvh);
			},
		}
		// println!("\nhitbox: {:?}\nblockbox: {:?}\nhurtbox: {:?}\n", self.hitbox, self.blockbox, self.hurtbox);
	}
	pub fn get_bb(&self) -> Rect {
		if self.hurtbox.is_some() {
			self.hurtbox.clone().unwrap().borrow().rect.clone()
		}
		else if self.hitbox.is_some() {
			self.hitbox.clone().unwrap().borrow().rect.clone()
		}
		else if self.blockbox.is_some(){
			self.blockbox.clone().unwrap().borrow().rect.clone()
		}
		else {Rect::new(0,0,0,0)}
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	#[test]
	pub fn testInsert() {
		let mut f = Fighter::new(CharacterState::new());
		let platform = Rect::new(40, 620, CAM_W-80, CAM_H-680);
		let collisions = BVHierarchy::new(CollisionObject::new_from(CollisionObjectType::Platform, platform.clone(),
		RefCell::new(Particle::new(
			PhysVec::new(platform.x as f32, platform.y as f32), 0.5, 2000000000.0))));
		f.char_state.update_bounding_boxes(&collisions);

		assert_eq!(f.char_state.position().position.raw(), (0.0, 0.0));
	}
}
