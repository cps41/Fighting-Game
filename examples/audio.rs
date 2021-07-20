extern crate street_code_fighter as scf;
use street_code_fighter::view::globals::{TITLE, CAM_W, CAM_H};

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String>  {
	// have to initialize audio in SDL2!!
    let mut game_window = {
        match scf::view::core::SDLCore::init(TITLE, false, 200, 200){
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        }
    };

    let clips = scf::audio::handler::Clips::new();

 	// sdl2::mixer::Channel::all().play(&clips.opening, 1);
 	// game_window.timer.delay(13_000); // wait for opening to end
 	// sdl2::mixer::Channel::all().halt(); // halt the opening music, before restarting (otherwise all will play)

 	sdl2::mixer::Channel::all().play(&clips.combat1, -1); // -1 means repeat forever

	let mut count = 0;

    'gameloop: loop{
        for event in game_window.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                //_ => { input::inputHandler::keyboard_input(&event, &mut fighter); }
                _=> {},
            }
        } // for

        // if count == 5000000 { // this is just testing logic
        // 	println!("hit");
        // 	sdl2::mixer::Channel::all().play(&clips.hit, 1);
        // }
        // if count == 10000000 { // just for ze testing 
        // 	println!("ko");
        // 	sdl2::mixer::Channel::all().play(&clips.ko, 1);
        // 	count = 0;
        // }

        count = count + 1; 
    } // gameloop

	Ok(())
}


