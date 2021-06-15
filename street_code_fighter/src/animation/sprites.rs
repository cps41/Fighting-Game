use crate::characters; // to reference Character

use sdl2::rect::Rect;

// EDIT: update based on moves to characterAbstract

// constants based on current sprite sheets 150ppi
const W: u32 = 210;
const H: u32 = 300;
const Y: i32 = 0;

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

// Functions to get current file name as string, to use to generate textures
pub fn get_state_filename(c: &characters::characterAbstract::Fighter) -> &'static str {
	match c.name {
		characters::characterAbstract::Characters::Python =>
			match c.char_state.state {
				State::Idle => { return "src/assets/images/characters/python/idle.png"; },
				State::Walk => { return "src/assets/images/characters/python/walk.png"; },
				State::Jump => { return "src/assets/images/characters/python/jump.png"; },
				State::FJump => { return "src/assets/images/characters/python/fjump.png"; },
				State::LPunch => { return "src/assets/images/characters/python/lpunch.png"; },
				State::LKick => { return "src/assets/images/characters/python/lkick.png"; },
				State::HKick => { return "src/assets/images/characters/python/hkick.png"; },
				State::Block => { return "src/assets/images/characters/python/block.png"; },
			},
	}
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
				State::Idle => { return 5; },
				State::Walk => { return 6; },
				State::Jump => { return 6; },
				State::FJump => { return 7; },
				State::LPunch => { return 3; },
				State::LKick => { return 3; },
				State::HKick => { return 5; },
				State::Block => { return 1; },
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
