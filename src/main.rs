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
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::time::{Instant, Duration}; // needed for FPS
use std::thread;
use std::env;
use physics::collisions::*;
use physics::vecmath::*;
use physics::particle::*;
use view::globals::*;

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
    let fighter2 = characters::characterAbstract::CharacterState::new();

    let mut fighter = characters::characterAbstract::Fighter::new(fighter);
    let mut fighter2 = characters::characterAbstract::Fighter::new(fighter2);
    //this is just to make fighter2 spawn a little to the right of fighter
    fighter2.char_state.particle.borrow_mut().position.replace(&PhysVec::new(300.0, 0.0));
    fighter2.name = characters::characterAbstract::Characters::Java;

    let mut hazard = physics::hazard::Hazard::new();

    let texture_creator = game_window.wincan.texture_creator();

    let platform = Rect::new(50, 560, CAM_W-100, 30);
    let wall_l = Rect::new(WALL_L.0, WALL_L.1, WALL_SIZE.0, WALL_SIZE.1);
    let wall_r = Rect::new(WALL_R.0, WALL_R.1, WALL_SIZE.0, WALL_SIZE.1);
    let arch = Rect::new(ARCH.0, ARCH.1, ARCH_SIZE.0, ARCH_SIZE.1);


    //////////////////////////
    // FUNCTIONING
    // EDIT: Modularize. Challenge: figuring out how to deal with texture's + hashmap lifetime
    // create HashMap of all textures
    let mut python_textures = HashMap::new();
    let mut java_textures = HashMap::new();

    let idle = texture_creator.load_texture("src/assets/images/characters/python/idle.png")?;
    let walk = texture_creator.load_texture("src/assets/images/characters/python/walk.png")?;
    let jump = texture_creator.load_texture("src/assets/images/characters/python/jump.png")?;
    let fjump = texture_creator.load_texture("src/assets/images/characters/python/fjump.png")?;
    let lpunch = texture_creator.load_texture("src/assets/images/characters/python/lpunch.png")?;
    let lkick = texture_creator.load_texture("src/assets/images/characters/python/lkick.png")?;
    let hkick = texture_creator.load_texture("src/assets/images/characters/python/hkick.png")?;
    let block = texture_creator.load_texture("src/assets/images/characters/python/block.png")?;
    let hazard_texture = texture_creator.load_texture("src/assets/images/hazards/stalactite100x100.png")?;
    let background = texture_creator.load_texture("src/assets/images/background/small_background.png")?;
    let healthbar_left = texture_creator.load_texture("src/assets/images/healthbar/healthbar_left.png")?;
    let healthbar_right = texture_creator.load_texture("src/assets/images/healthbar/healthbar_right.png")?;
    let healthbar_fill_left = texture_creator.load_texture("src/assets/images/healthbar/healthbar_fill_left.png")?;
    let healthbar_fill_right = texture_creator.load_texture("src/assets/images/healthbar/healthbar_fill_right.png")?;

    let java_idle = texture_creator.load_texture("src/assets/images/characters/java/idle.png")?;
    let java_walk = texture_creator.load_texture("src/assets/images/characters/java/walk.png")?;
    let java_jump = texture_creator.load_texture("src/assets/images/characters/java/jump.png")?;
    let java_fjump = texture_creator.load_texture("src/assets/images/characters/java/fjump.png")?;
    let java_lpunch = texture_creator.load_texture("src/assets/images/characters/java/lpunch.png")?;
    let java_lkick = texture_creator.load_texture("src/assets/images/characters/java/lkick.png")?;
    let java_hkick = texture_creator.load_texture("src/assets/images/characters/java/hkick.png")?;
    let java_block = texture_creator.load_texture("src/assets/images/characters/java/block.png")?;

    python_textures.insert(animation::sprites::State::Idle, idle);
    python_textures.insert(animation::sprites::State::Walk, walk);
    python_textures.insert(animation::sprites::State::Jump, jump);
    python_textures.insert(animation::sprites::State::FJump, fjump);
    python_textures.insert(animation::sprites::State::LPunch, lpunch);
    python_textures.insert(animation::sprites::State::LKick, lkick);
    python_textures.insert(animation::sprites::State::HKick, hkick);
    python_textures.insert(animation::sprites::State::Block, block);

    java_textures.insert(animation::sprites::State::Idle, java_idle);
    java_textures.insert(animation::sprites::State::Walk, java_walk);
    java_textures.insert(animation::sprites::State::Jump, java_jump);
    java_textures.insert(animation::sprites::State::FJump, java_fjump);
    java_textures.insert(animation::sprites::State::LPunch, java_lpunch);
    java_textures.insert(animation::sprites::State::LKick, java_lkick);
    java_textures.insert(animation::sprites::State::HKick, java_hkick);
    java_textures.insert(animation::sprites::State::Block, java_block);

    ///////////////////////
    // NOT YET FUNCTIONING
    // Self::load_textures(&texture_creator, &mut fighter);
    ////////

    //load window before game starts with starting texture
    let texture = {
        match python_textures.get(&fighter.char_state.state) {
            Some(text) => text,
            _=> panic!("No texture found for the state! Oh nos."),
        }
    };

    let texture2 = {
        match java_textures.get(&fighter2.char_state.state) {
            Some(text) => text,
            _=> panic!("No texture found for the state! Oh nos."),
        }
    };

    game_window.render(&background, &texture, &fighter, &texture2, &fighter2, 
            &hazard, &hazard_texture, &platform, &healthbar_left, &healthbar_right,
            &healthbar_fill_left, &healthbar_fill_right)?;


    let collisions = BVHierarchy::new(CollisionObject::new_from(CollisionObjectType::Platform, platform.clone(),
        Rc::new(RefCell::new(Particle::new(
            PhysVec::new((CAM_W-50-platform.width()) as f32, 560f32), 0.5, 2000000000.0, 0)))));

    // collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_l, 
    //     Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_L.0 as f32, WALL_L.1 as f32), 0.5, 20000000000.0, 0)))));
    // collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_r, 
    //     Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_R.0 as f32, WALL_R.1 as f32), 0.5, 20000000000.0, 0)))));
    collisions.insert(CollisionObject::new_from(CollisionObjectType::Platform, arch, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(ARCH.0 as f32, ARCH.1 as f32), 0.5, 20000000000.0, 0)))));



//################################################-GAME-LOOP###############################################
    'gameloop: loop{
        let loop_time = Instant::now();
    //################################################-GET-INPUT-##########################################
        //ceck if play quits
        for event in game_window.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                //_ => { input::inputHandler::keyboard_input(&event, &mut fighter); }
                _=> {},
            }
        }


        //gather player input
        let player_input: HashSet<Keycode> = game_window.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

    //##############################################-PROCESS-EVENTS-#######################################
        //process player movement
        input::inputHandler::keyboard_input(&player_input, &mut fighter);

        //select frame to be rendered
        fighter.char_state.advance_frame();
        fighter2.char_state.advance_frame();

        //move character based on current frame
        input::movement::move_char(&mut fighter);
        input::movement::move_char(&mut fighter2);

        fighter.char_state.update_bounding_boxes(&collisions);
        fighter2.char_state.update_bounding_boxes(&collisions);
        hazard.update_bounding_box(&collisions);
        let hazard_reset = collisions.resolve_collisions();
        // println!("\nCollisions head: \n{:?}\n", collisions.head);
        fighter.char_state.particle.borrow_mut().integrate(FRAME_RATE as f32);
        fighter2.char_state.particle.borrow_mut().integrate(FRAME_RATE as f32);

        //move hazard
        if hazard.sprite.y() < 600 && hazard.fell == false {
           hazard.sprite.offset(0, 7);
           //println!("{}", hazard.sprite.y())
       }
    //    if hazard.sprite.y() >= 600 || hazard_reset {
        if hazard_reset {
           hazard.reset();
           hazard.fell = false;
       }
    //##################################################-RENDER-###########################################

        // get the proper texture within the game
        let texture = {
            match python_textures.get(&fighter.char_state.state) {
                Some(text) => text,
                _=> panic!("No texture found for the state! Oh nos."),
            }
        };
        let texture2 = {
            match java_textures.get(&fighter2.char_state.state) {
                Some(text) => text,
                _=> panic!("No texture found for the state! Oh nos."),
            }
        };

        // render canvas
        game_window.render(&background, &texture, &fighter, &texture2, &fighter2, 
            &hazard, &hazard_texture, &platform, &healthbar_left, &healthbar_right,
            &healthbar_fill_left, &healthbar_fill_right)?;
    //##################################################-SLEEP-############################################

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
