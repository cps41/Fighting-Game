use crate::characters; // to reference Character

use sdl2::rect::Rect;
use serde_derive::{Serialize, Deserialize}; // NOT YET OFFICIALLY AUTHORIZED
// EDIT: update based on moves to characterAbstract

// constants based on current sprite sheets 150ppi
const W: u32 = 210;
const H: u32 = 300;
const Y: i32 = 0;

// enumeration of the various states
#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
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

// Gets the rectangle to use for positioning view of sprite
pub fn get_rectangle(f: u32) -> Rect { // current frame
	let x = W*f; // + 0
	return Rect::new(x as i32, Y, W, H);
}

// Gets the numbers of frames per move
pub fn get_frame_cnt(c: &characters::characterAbstract::CharacterState) -> i32 {
	// TODO: ensure every character has same # of animations/state
	match c.state {
		//Frames per Animation, Sprites per animation
		State::Idle 	=> { return 30; },	//5
		State::Walk 	=> { return 30; },	//6
		State::Jump 	=> { return 30; },	//6
		State::FJump 	=> { return 42; },	//7
		State::LPunch 	=> { return 17; },	//3
		State::LKick 	=> { return 25; },	//3
		State::HKick 	=> { return 35; },	//5
		State::Block 	=> { return 1; },	//1
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
