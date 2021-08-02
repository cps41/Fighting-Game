extern crate sdl2;

use sdl2::keyboard::TextInputUtil;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::pixels::Color;
use std::collections::HashMap;
use sdl2::video::WindowContext;
use sdl2::rect::Point;

use crate::characters;
use crate::animation;
use crate::input::movement::Direction;
use crate::physics;

use super::globals::*;

pub struct SDLCore{
	sdl_cxt: sdl2::Sdl,
	pub wincan: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,
	pub audio: sdl2::AudioSubsystem,
	pub timer: sdl2::TimerSubsystem,
}

impl SDLCore{
	pub fn init(
		title: &str,
		vsync: bool,
		width: u32,
		height: u32,
	) -> Result<SDLCore, String>{
		let sdl_cxt = sdl2::init()?;
		let video_subsys = sdl_cxt.video()?;

	// adding for audio	
    let audio = sdl_cxt.audio()?;
    let mut timer = sdl_cxt.timer()?;

		let window = video_subsys.window(title, width, height).build().map_err(|e| e.to_string())?;
		let wincan = window.into_canvas().accelerated();

		let wincan = if vsync {
			wincan.present_vsync()
		}else{
			wincan
		};

		let mut wincan = wincan.build().map_err(|e| e.to_string())?;

		let event_pump = sdl_cxt.event_pump()?;

		wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
		wincan.clear();
		wincan.present();

		Ok(SDLCore{
			sdl_cxt,
			wincan,
			event_pump,
			audio,
			timer,
		})
	}

	pub fn render(&mut self,
				background: &Texture,
				texture: &Texture,
				fighter: &characters::characterAbstract::Fighter,
				texture2: &Texture,
				fighter2: &characters::characterAbstract::Fighter,
				hazard: &physics::hazard::Hazard,
				hazard_texture: &Texture,
				end: Option<&Texture>,
				healthbar_left: &Texture,
				healthbar_right: &Texture,
				healthbar_fill_left: &Texture,
				healthbar_fill_right: &Texture,
				) -> Result<(), String>{

		// set canvas height
		let (width, height) = self.wincan.output_size()?;

		// background
		self.wincan.copy(background, None, None)?;
		self.wincan.set_draw_color(Color::YELLOW);
		let wall_l = Rect::new(WALL_L.0, WALL_L.1, WALL_SIZE.0, WALL_SIZE.1);
		let wall_r = Rect::new(WALL_R.0, WALL_R.1, WALL_SIZE.0, WALL_SIZE.1);
		let arch = Rect::new(ARCH.0, ARCH.1, ARCH_SIZE.0, ARCH_SIZE.1);
		self.wincan.draw_rects(&[Rect::new(100, 560, CAM_W-200, 30), wall_l, wall_r, arch])?;
		//self.wincan.clear();

		// fill health bars
		if fighter.char_state.health() > 0 {
			self.wincan.copy(healthbar_fill_left, 
				Rect::new(0,0, 300-(270-fighter.char_state.health() as u32), 40), 
				Rect::new(3,10, 300-(270-fighter.char_state.health() as u32), 40))?;
		}
		if fighter2.char_state.health() > 0 {
			self.wincan.copy(healthbar_fill_right, 
				Rect::new(270-fighter2.char_state.health(),0, 300-(270-fighter2.char_state.health() as u32), 40), 
				Rect::new(CAM_W as i32-(300-(270-fighter2.char_state.health()))-3,10, 300-(270-fighter2.char_state.health() as u32), 40))?;
		}
		self.wincan.copy(healthbar_left, None, Rect::new(3,10, 300, 40))?;
		self.wincan.copy(healthbar_right, None, Rect::new(CAM_W as i32-300-3,10, 300, 40))?;

		let (frame_width, frame_height) = fighter.char_state.sprite.size();

		//get curent chararcter state
        let current_frame = Rect::new(
        	//determins which sprite to get, using current_frame as offset on sprite sheet
            fighter.char_state.sprite.x() + frame_width as i32 * fighter.char_state.current_frame,
            fighter.char_state.sprite.y(), // should always be 0, since y should remain consistent
            frame_width,
            frame_height,
        );

		let current_frame2 = Rect::new(
        	//determins which sprite to get, using current_frame as offset on sprite sheet
            fighter2.char_state.sprite.x() + frame_width as i32 * fighter2.char_state.current_frame,
            fighter2.char_state.sprite.y(), // should always be 0, since y should remain consistent
            frame_width,
            frame_height,
        );

		let hazard_frame = Rect::new(0, 0, 100, 100);

        // (0, 0) cordinate = center of the scren
		// make new rect and screen pos //

        let screen_position = fighter.char_state.particle.borrow().to_point() + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
		let screen_position2 = fighter2.char_state.particle.borrow().to_point() + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect2 = Rect::from_center(screen_position2, frame_width, frame_height);


		// hazard rectangle & position
		let hazard_screen_position = hazard.position;
		let hazard_screen_rectangle = Rect::new(hazard.sprite.x, hazard.sprite.y, 50, 50);

		// copy textures
        if let Direction::Left = fighter.char_state.direction() {
			self.wincan.copy(texture, current_frame, screen_rect)?;
		}
		else {
			self.wincan.copy(texture, current_frame, screen_rect)?;
		}
        if let Direction::Left = fighter2.char_state.direction() {
			self.wincan.copy_ex(texture2, current_frame2, screen_rect2, 0.0, None, true, false)?;
		}
		else {
			self.wincan.copy_ex(texture2, current_frame2, screen_rect2, 0.0, None, true, false)?;
		}
		// self.wincan.copy_ex(texture2, current_frame2, screen_rect2, 0.0, None, true, false)?;
		self.wincan.copy(hazard_texture, hazard_frame, hazard_screen_rectangle)?;
		self.wincan.set_draw_color(Color::RED);
		self.wincan.draw_rects(&[fighter.char_state.get_bb(), fighter2.char_state.get_bb(), hazard.get_bb()])?;
		if end.is_some() {
			self.wincan.copy(end.unwrap(), 
				Rect::new((700-415)/2,(300-155)/2,415, 155), 
				Rect::new((CAM_W as i32-415)/2, (CAM_H as i32-155)/2, 415, 155))?;
		}
        self.wincan.present();

        /*
        println!("Frame count is: {}    Frame Per State is: {}    Current Sprite is: {}    State is: {:?}",
        fighter.char_state.frame_count, fighter.char_state.frames_per_state,
        fighter.char_state.current_frame, fighter.char_state.state);
		*/


        Ok(())
	} // closing render fun
/*
    // NOT FUNCTIONING YET
    fn load_textures(texture_creator: &'t TextureCreator<WindowContext>,
                     f: &mut characters::characterAbstract::Fighter) {

            // let idle = texture_creator.load_texture("src/assets/images/characters/python/idle-outline.png");

            // match idle {
            //     Ok(i) =>  { f.add_texture(animation::sprites::State::Idle, i); },
            //     Err(e) => { panic!("Nooo"); },
            // }

    } // close load_textures
*/

}
