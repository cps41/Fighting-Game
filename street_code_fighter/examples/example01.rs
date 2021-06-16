extern crate street_code_fighter as scf;
extern crate sdl2;

use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;

use scf::SDLCore;
use scf::Demo;

const TITLE: &str = "Testing Character on Screen";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 5000;

pub struct SDL01 {
	core: SDLCore,
}

impl Demo for SDL01 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL01{ core })
	}

	fn run(&mut self) -> Result<(), String> {

		// creating initial character state and 
		let cs = scf::characters::characterAbstract::CharacterState::new();
		let mut fighter = scf::characters::characterAbstract::Fighter::new(cs);

		// loading textures
		let texture_creator = self.core.wincan.texture_creator();
		let texture = texture_creator.load_texture("src/assets/images/characters/python/fjump-outline.png")?;
	    
	    'gameloop: loop { // game loop
			for event in self.core.event_pump.poll_iter() { //input events
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					Event::KeyDown{keycode: Some(k), ..} => {
						match k {
							Keycode::W => (), // jump
							Keycode::A => { fighter.char_state.direction = scf::input::movement::Direction::Left; // update direction left
											scf::input::movement::walk(&mut fighter); // character walks left
										   },
							Keycode::S => (), // crouch (stretch goal)
							Keycode::D => { fighter.char_state.direction = scf::input::movement::Direction::Right; // update direction right 
											scf::input::movement::walk(&mut fighter); // character walks right
										   },
							Keycode::Space => (), 
							_ => {},
						}
					}
					_ => {},
				}
			}

        // update player states
        // update_fighter(&mut fighter);

        // render canvas
        Self::render(&mut self.core.wincan, Color::RGB(222,222,222), &texture, &fighter);

		}
		Ok(())
	}


	fn render(canvas: &mut WindowCanvas,
			  color: Color,
			  texture: &Texture,
			  fighter: &scf::characters::characterAbstract::Fighter,
			  ) -> Result<(), String> {

			// color
		    canvas.set_draw_color(color);
		    canvas.clear();

		    // set canvas height
    		let (width, height) = canvas.output_size()?;


		    let (frame_width, frame_height) = fighter.char_state.sprite.size();

		    let current_frame = Rect::new(
		        fighter.char_state.sprite.x() + frame_width as i32 * fighter.char_state.current_frame,
		        fighter.char_state.sprite.y(), // should always be 0, since y should remain consistent
		        frame_width,
		        frame_height,
		    );

		    // (0, 0) coordinate = center of the scren
		    let screen_position = fighter.char_state.position + Point::new(width as i32 / 2, height as i32 / 2);
		    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
		    canvas.copy(texture, current_frame, screen_rect)?;

		    canvas.present();

		    Ok(())
	}
}

fn main() {
	scf::runner(TITLE, SDL01::init);
}