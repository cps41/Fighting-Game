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
use rand::prelude::*;
use std::collections::VecDeque;

pub mod characters; // for characterAbstract
pub mod view; // for core
pub mod input; // for inputHandler and movement
pub mod animation;
pub mod networking;
pub mod physics;
pub mod audio;

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

    let platform = Rect::new(100, 560, CAM_W-200, 30);
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
    let win = texture_creator.load_texture("src/assets/images/end/win.png")?;
    let lose = texture_creator.load_texture("src/assets/images/end/lose.png")?;

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

    let mut end_message = None;

    ///////////////////////
    // NOT YET FUNCTIONING
    // Self::load_textures(&texture_creator, &mut fighter);
    ////////

    // get random # (for music)
    let mut rng = rand::thread_rng();
    let random_num: f64 = rng.gen(); // generates a float between 0 and 1
    println!("{}", random_num);

    // music 
    let clips = audio::handler::Clips::new();

        // randomize between the 3x combat audio tracks
    if random_num < 0.4 { 
        sdl2::mixer::Channel::all().play(&clips.combat1, -1); // -1 means repeat forever
    } else if random_num < 0.7 {
        sdl2::mixer::Channel::all().play(&clips.combat2, -1); // -1 means repeat forever  
    } else {
        sdl2::mixer::Channel::all().play(&clips.combat3, -1); // -1 means repeat forever
    }


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
            &hazard, &hazard_texture, None, &healthbar_left, &healthbar_right,
            &healthbar_fill_left, &healthbar_fill_right)?;


    let collisions = BVHierarchy::new(CollisionObject::new_from(CollisionObjectType::Platform, platform.clone(),
        Rc::new(RefCell::new(Particle::new(
            PhysVec::new(((CAM_W-platform.width())/2) as f32, 560f32), 0.5, 2000000000.0, 0, 0)))));

    collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_l, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_L.0 as f32, WALL_L.1 as f32), 0.5, 20000000000.0, 0, 0)))));
    collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_r, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_R.0 as f32, WALL_R.1 as f32), 0.5, 20000000000.0, 0, 0)))));
    collisions.insert(CollisionObject::new_from(CollisionObjectType::Platform, arch, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(ARCH.0 as f32, ARCH.1 as f32), 0.5, 20000000000.0, 0, 0)))));



//################################################-GAME-LOOP###############################################
    'gameloop: loop {
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

        let player_input = input::inputHandler::convert_input(&player_input);

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
        // println!("\nCollisions head BEFORE: \n{:#?}\n", collisions.head);
        // println!("\n\nupdating...");
		// println!("\nFighter 1\n {:?}\n", fighter.char_state.get_node());
		// println!("\nFighter 2\n {:?}\n", fighter2.char_state.get_node());
		// println!("\nHazard\n {:?}\n", hazard.hitbox);
        let hazard_reset = collisions.resolve_collisions();
        // println!("\nCollisions head AFTER: \n{:#?}\n", collisions.head);
        fighter.char_state.particle.borrow_mut().integrate(FRAME_RATE as f32);
        fighter2.char_state.particle.borrow_mut().integrate(FRAME_RATE as f32);
        hazard.particle.borrow_mut().integrate(FRAME_RATE as f32);

        //move hazard
        hazard.update_position();
        if hazard_reset {
           hazard.reset();
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

        end_message = {
            // check if game should continue
            if fighter.char_state.health() <= 0 {
                Some(&lose)
            }
            else if fighter2.char_state.health() <= 0 {
                Some(&win)
            }
            else {
                None
            }
        };
        // render canvas
        game_window.render(&background, &texture, &fighter, &texture2, &fighter2, 
            &hazard, &hazard_texture, end_message, &healthbar_left, &healthbar_right,
            &healthbar_fill_left, &healthbar_fill_right)?;
        
        if end_message.is_some() {
            break 'gameloop;
        }
    //##################################################-SLEEP-############################################

        thread::sleep(frame_time - loop_time.elapsed().clamp(Duration::new(0, 0), frame_time));
    }

    'endloop: loop {
    //################################################-GET-INPUT-##########################################
        //check if play quits
        for event in game_window.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'endloop,
                //_ => { input::inputHandler::keyboard_input(&event, &mut fighter); }
                _=> {},
            }
        }
        // render canvas
        game_window.render(&background, &texture, &fighter, &texture2, &fighter2, 
            &hazard, &hazard_texture, end_message, &healthbar_left, &healthbar_right,
            &healthbar_fill_left, &healthbar_fill_right)?;
    }

    Ok(())
}

pub fn run_server() -> Result<(), String>{
    let frame_time = Duration::from_secs_f64(FRAME_RATE);

    let socket = networking::config::server_setup();
    socket.set_read_timeout(None).expect("set_read_timeout call failed");

    let mut client_addresses = HashMap::new();
    let mut player_count: u8 = 1;

    'connecting: loop{
        player_count = networking::config::client_connect(&socket, &mut client_addresses, player_count);
        if player_count == 3 {
            println!("Two players found!");
            break 'connecting;
        }
    }

    // Creating initial character state
    let fighter1 = characters::characterAbstract::CharacterState::new();
    let fighter2 = characters::characterAbstract::CharacterState::new();

    let mut fighter1 = characters::characterAbstract::Fighter::new(fighter1);
    let mut fighter2 = characters::characterAbstract::Fighter::new(fighter2);
    //this is just to make fighter2 spawn a little to the right of fighter
    fighter2.char_state.particle.borrow_mut().position.replace(&PhysVec::new(300.0, 0.0));
    fighter2.name = characters::characterAbstract::Characters::Java;

    let mut hazard = physics::hazard::Hazard::new();

    let platform = Rect::new(100, 560, CAM_W-200, 30);
    let wall_l = Rect::new(WALL_L.0, WALL_L.1, WALL_SIZE.0, WALL_SIZE.1);
    let wall_r = Rect::new(WALL_R.0, WALL_R.1, WALL_SIZE.0, WALL_SIZE.1);
    let arch = Rect::new(ARCH.0, ARCH.1, ARCH_SIZE.0, ARCH_SIZE.1);


    let collisions = BVHierarchy::new(CollisionObject::new_from(CollisionObjectType::Platform, platform.clone(),
        Rc::new(RefCell::new(Particle::new(
            PhysVec::new(((CAM_W-platform.width())/2) as f32, 560f32), 0.5, 2000000000.0, 0, 0)))));

    collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_l, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_L.0 as f32, WALL_L.1 as f32), 0.5, 20000000000.0, 0, 0)))));
    collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_r, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_R.0 as f32, WALL_R.1 as f32), 0.5, 20000000000.0, 0, 0)))));
    collisions.insert(CollisionObject::new_from(CollisionObjectType::Platform, arch, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(ARCH.0 as f32, ARCH.1 as f32), 0.5, 20000000000.0, 0, 0)))));


    for address in client_addresses.keys(){
        socket.send_to(&[0], address).expect("message not sent");
    }

    socket.set_nonblocking(true).unwrap();

  //################################################-GAME-LOOP###############################################
    'gameloop: loop{
        let readout_time = Instant::now();
    //################################################-GET-INPUT-##########################################
        let mut input_1: HashSet<u8> = HashSet::new();
        let mut input_2: HashSet<u8> = HashSet::new();
        let mut message_1 = false;
        let mut message_2 = false;        
        
        'peeking: loop{
            if networking::transmit::ready_to_read(&socket){break;}
        }

        let receive_time = Instant::now();
        'receiving: loop{
            networking::transmit::receive_input(&socket, &client_addresses, &mut input_1, 
                &mut input_2, &mut message_1, &mut message_2, &readout_time);
        
            if receive_time.elapsed().as_millis() >= Duration::from_secs_f64(FRAME_RATE*2.0).as_millis() 
                || message_1 && message_2 { break; }
        }


    //##############################################-PROCESS-EVENTS-#######################################
        //process player movement
        input::inputHandler::keyboard_input(&input_1, &mut fighter1);
        input::inputHandler::keyboard_input(&input_2, &mut fighter2);


        //select frame to be rendered
        fighter1.char_state.advance_frame();
        fighter2.char_state.advance_frame();

        //move character based on current frame
        input::movement::move_char(&mut fighter1);
        input::movement::move_char(&mut fighter2);

        fighter1.char_state.update_bounding_boxes(&collisions);
        fighter2.char_state.update_bounding_boxes(&collisions);
        hazard.update_bounding_box(&collisions);
        

        let hazard_reset = collisions.resolve_collisions();
        fighter1.char_state.particle.borrow_mut().integrate(FRAME_RATE as f32);
        fighter2.char_state.particle.borrow_mut().integrate(FRAME_RATE as f32);

        //move hazard
        hazard.update_position();
        if hazard_reset {
           hazard.reset();
       }
    //#############################################-SEND-GAMESTATE-#######################################
        
        let current_frame = networking::transmit::GameState::new(&fighter1, &fighter2, &hazard);
        networking::transmit::send_game_state(&socket, &client_addresses, &current_frame);    
    }
    Ok(())
}

pub fn run_client() -> Result<(), String>{
    let frame_time = Duration::from_secs_f64(FRAME_RATE);

    let (socket, player_number) = networking::config::client_setup();
    socket.set_read_timeout(None).expect("set_read_timeout call failed");


    let mut game_window = {
        match view::core::SDLCore::init(TITLE, false, CAM_W, CAM_H){
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        }
    };

    // Creating initial character state
    let fighter1 = characters::characterAbstract::CharacterState::new();
    let fighter2 = characters::characterAbstract::CharacterState::new();

    let mut fighter1 = characters::characterAbstract::Fighter::new(fighter1);
    let mut fighter2 = characters::characterAbstract::Fighter::new(fighter2);
    //this is just to make fighter2 spawn a little to the right of fighter
    fighter2.char_state.particle.borrow_mut().position.replace(&PhysVec::new(300.0, 0.0));
    fighter2.name = characters::characterAbstract::Characters::Java;

    let mut hazard = physics::hazard::Hazard::new();

    let texture_creator = game_window.wincan.texture_creator();

    let platform = Rect::new(100, 560, CAM_W-200, 30);
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
    let win = texture_creator.load_texture("src/assets/images/end/win.png")?;
    let lose = texture_creator.load_texture("src/assets/images/end/lose.png")?;

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

    // get random # (for music)
    let mut rng = rand::thread_rng();
    let random_num: f64 = rng.gen(); // generates a float between 0 and 1
    println!("{}", random_num);

    // music 
    let clips = audio::handler::Clips::new();

        // randomize between the 3x combat audio tracks
    if random_num < 0.4 { 
        sdl2::mixer::Channel::all().play(&clips.combat1, -1); // -1 means repeat forever
    } else if random_num < 0.7 {
        sdl2::mixer::Channel::all().play(&clips.combat2, -1); // -1 means repeat forever  
    } else {
        sdl2::mixer::Channel::all().play(&clips.combat3, -1); // -1 means repeat forever
    }


    //load window before game starts with starting texture
    let texture = {
        match python_textures.get(&fighter1.char_state.state) {
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

    game_window.render(&background, &texture, &fighter1, &texture2, &fighter2, 
            &hazard, &hazard_texture, None, &healthbar_left, &healthbar_right,
            &healthbar_fill_left, &healthbar_fill_right)?;


    let collisions = BVHierarchy::new(CollisionObject::new_from(CollisionObjectType::Platform, platform.clone(),
        Rc::new(RefCell::new(Particle::new(
            PhysVec::new(((CAM_W-platform.width())/2) as f32, 560f32), 0.5, 2000000000.0, 0, 0)))));

    collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_l, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_L.0 as f32, WALL_L.1 as f32), 0.5, 20000000000.0, 0, 0)))));
    collisions.insert(CollisionObject::new_from(CollisionObjectType::Wall, wall_r, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(WALL_R.0 as f32, WALL_R.1 as f32), 0.5, 20000000000.0, 0, 0)))));
    collisions.insert(CollisionObject::new_from(CollisionObjectType::Platform, arch, 
        Rc::new(RefCell::new(Particle::new(PhysVec::new(ARCH.0 as f32, ARCH.1 as f32), 0.5, 20000000000.0, 0, 0)))));



    let mut input_buffer: VecDeque<networking::transmit::GameState> = VecDeque::new();

    for i in 0 .. 6{
        input_buffer.push_back(networking::transmit::GameState::new(&fighter1, &fighter2, &hazard));
    }

    println!("Waiting for other player...");
    let mut buffer = [0u8; 800];
    let (number_of_bytes) = socket.recv(&mut buffer).expect("Didn't receive data");
    println!("Starting Game");
    socket.set_nonblocking(true).unwrap();

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

        let player_input = input::inputHandler::convert_input(&player_input);

    //##############################################-PROCESS-EVENTS-#######################################
        let player_input = networking::transmit::InputStruct::new(player_input);
        let readout_time = Instant::now();


        networking::transmit::send_input(&socket, &player_input);

        let state = input_buffer.pop_front().unwrap();
        
        fighter1.char_state.set_state(state.p1_state);
        fighter1.char_state.current_frame = state.p1_frame;
        fighter1.char_state.particle.replace(state.p1_position);

        fighter2.char_state.set_state(state.p2_state);
        fighter2.char_state.current_frame = state.p2_frame;
        fighter2.char_state.particle.replace(state.p2_position);

        hazard.from_packet(&state.hazard);    
    //##################################################-RENDER-###########################################

        // get the proper texture within the game
        let texture = {
            match python_textures.get(&fighter1.char_state.state) {
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

        let end_message = {
            // check if game should continue
            if fighter1.char_state.health() <= 0 {
                Some(&lose)
            }
            else if fighter2.char_state.health() <= 0 {
                Some(&win)
            }
            else {
                None
            }
        };

        // render canvas
        game_window.render(&background, &texture, &fighter1, &texture2, &fighter2, 
            &hazard, &hazard_texture, end_message, &healthbar_left, &healthbar_right,
            &healthbar_fill_left, &healthbar_fill_right)?;
    //##################################################-SLEEP-############################################
            let mut next_state = networking::transmit::GameState::new(&fighter1, &fighter2, &hazard);
            let receive_time = Instant::now();
           
            'reading: loop{
                if networking::transmit::receive_game_state(&socket, &mut next_state, &readout_time) { 
                    break;
                }else if(receive_time.elapsed().as_millis() > Duration::from_secs_f64(FRAME_RATE*2.0).as_millis()){
                    break;
                }
            }

            input_buffer.push_back(next_state);

        

        thread::sleep(frame_time - loop_time.elapsed().clamp(Duration::new(0, 0), frame_time));
    }
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
    }else if args.len() > 1 && "client".eq(&args[1]){
        run_client()?;
        //networking::chatClient::server_connect();
    }else{
        run_game()?;
    }

    // run_credits()?;

    Ok(())
}
