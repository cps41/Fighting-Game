extern crate sdl2;

use sdl2::image::{self, LoadTexture}; // InitFlag,
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
// use std::fs;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::time::{Instant, Duration}; // needed for FPS
use std::thread;
use std::env;

pub mod characters; // for characterAbstract
pub mod view; // for core
pub mod input; // for inputHandler and movement
pub mod animation;
pub mod networking;
pub mod physics;

//use crate::view::core; // need for SDLCore and TextureManager
//use crate::view::core::Demo; // need for SDLCore's Demo
// use crate::view::loads;

const TITLE: &str = "Street Code Fighter";
const TIMEOUT: u64 = 5000;
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const FRAME_RATE: f64 = 1.0/60.0;


pub fn run_game() -> Result<(), String>{
    let frame_time = Duration::from_secs_f64(FRAME_RATE);
    
    let mut game_window = {
        match view::core::SDLCore::init(TITLE, false, CAM_W, CAM_H){
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        }
    };

    // Creating initial character state
    let fighter = characters::characterAbstract::CharacterState::new();

    let mut fighter = characters::characterAbstract::Fighter::new(fighter);

    let mut hazard = physics::hazard::Hazard::new();

    let texture_creator = game_window.wincan.texture_creator();

    //////////////////////////
    // FUNCTIONING
    // EDIT: Modularize. Challenge: figuring out how to deal with texture's + hashmap lifetime
    // create HashMap of all textures
    let mut python_textures = HashMap::new();

    let idle = texture_creator.load_texture("src/assets/images/characters/python/idle-outline.png")?;
    let walk = texture_creator.load_texture("src/assets/images/characters/python/walk-outline.png")?;
    let jump = texture_creator.load_texture("src/assets/images/characters/python/jump-outline.png")?;
    let fjump = texture_creator.load_texture("src/assets/images/characters/python/fjump-outline.png")?;
    let lpunch = texture_creator.load_texture("src/assets/images/characters/python/lpunch-outline.png")?;
    let lkick = texture_creator.load_texture("src/assets/images/characters/python/lkick-outline.png")?;
    let hkick = texture_creator.load_texture("src/assets/images/characters/python/hkick-outline.png")?;
    let block = texture_creator.load_texture("src/assets/images/characters/python/block-outline.png")?;
    let hazard_texture = texture_creator.load_texture("src/assets/images/hazards/stalactite100x100.png")?;

    python_textures.insert(animation::sprites::State::Idle, idle);
    python_textures.insert(animation::sprites::State::Walk, walk);
    python_textures.insert(animation::sprites::State::Jump, jump);
    python_textures.insert(animation::sprites::State::FJump, fjump);
    python_textures.insert(animation::sprites::State::LPunch, lpunch);
    python_textures.insert(animation::sprites::State::LKick, lkick);
    python_textures.insert(animation::sprites::State::HKick, hkick);
    python_textures.insert(animation::sprites::State::Block, block);

    ///////////////////////
    // NOT YET FUNCTIONING
    // Self::load_textures(&texture_creator, &mut fighter);
    ////////

    let texture = {
        match python_textures.get(&fighter.char_state.state) {
            Some(text) => text,
            _=> panic!("No texture found for the state! Oh nos."),
        }
    };
    game_window.render(Color::RGB(222,222,222), &texture, &fighter, &hazard, &hazard_texture);


//################################################-GAME-LOOP###############################################
    'gameloop: loop{
        let loop_time = Instant::now();

//################################################-GET-INPUT-##############################################

        for event in game_window.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                //_ => { input::inputHandler::keyboard_input(&event, &mut fighter); }
                _=> {},
            }
        }

        let player_input: HashSet<Keycode> = game_window.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        
//##############################################-PROCESS-EVENTS-###########################################
        input::inputHandler::keyboard_input(&player_input, &mut fighter);
        fighter.char_state.advance_frame();
        input::movement::move_char(&mut fighter);

        //##########-PROCESS-COLLISIONS-HERE-##########

        hazard.sprite.offset(0, 15);
//##################################################-RENDER-###############################################
       
        // get the proper texture within the game
        let texture = {
            match python_textures.get(&fighter.char_state.state) {
                Some(text) => text,
                _=> panic!("No texture found for the state! Oh nos."),
            }
        };

        // render canvas
        game_window.render(Color::RGB(222,222,222), &texture, &fighter, &hazard, &hazard_texture);
//##################################################-SLEEP-#############################################        
        thread::sleep(frame_time - loop_time.elapsed().clamp(Duration::new(0, 0), frame_time));
    }

    Ok(())
}







pub fn run_server() -> Result<(), String>{
    networking::chatServer::server_start();
    Ok(())
}

/*
 // run credits
 pub fn run_credits() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("Fighter Game Team", CAM_W, CAM_H)
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated();

    let canvas = canvas.present_vsync();

    let mut canvas = canvas
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    //init and fill a vector with our textures
    let paths = fs::read_dir("./src/assets/images/credits").unwrap();
    let mut textures: Vec<sdl2::render::Texture> = Vec::new();

    for path in paths {
            textures.push(texture_creator.load_texture(path.unwrap().path().display().to_string())?);
    }

    canvas.set_draw_color(Color::RGBA(0, 128, 128, 255));
    canvas.clear();

    /* loop to display each texture, making sure they fit within the
        window and are positioned at the center before displaying them */
    for t in textures {
        let mut img_h = t.query().height;
        let mut img_w = t.query().width;

        while img_h > CAM_H || img_w > CAM_W {
            img_h = img_h/2;
            img_w = img_w/2;
        }

        let center_x = (CAM_W/2) - (img_w/2);
        let center_y = (CAM_H/2) - (img_h/2);
        let display_area = Rect::new(center_x as i32, center_y as i32, img_w, img_h);

        canvas.copy(&t, None, display_area)?;
        canvas.present();

        thread::sleep(Duration::from_millis(TIMEOUT));

        canvas.clear();
    }

    Ok(())
}
*/

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && "server".eq(&args[1]){
        run_server()?;
    }else{
        run_game()?;
        //networking::chatClient::server_connect();
    }

    // run_credits()?;

    Ok(())
}
