extern crate street_code_fighter as scf;
use street_code_fighter::view::globals::{TITLE, CAM_W, CAM_H};

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String>  {
	// have to initialize audio in SDL2!!
    let mut game_window = {
        match scf::view::core::SDLCore::init(TITLE, false, CAM_W, CAM_H){
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        }
    };

    // play audio
	scf::audio::audio::ko(&mut game_window.timer);
	scf::audio::audio::hit(&mut game_window.timer);
	scf::audio::audio::combat(&mut game_window.timer);
	scf::audio::audio::opening(&mut game_window.timer);

	Ok(())
}


