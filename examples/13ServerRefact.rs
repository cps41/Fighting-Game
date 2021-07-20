extern crate sdl2;
extern crate street_code_fighter;

use crate::street_code_fighter::*;
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
use std::path::Path;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use std::time::{Instant, Duration}; // needed for FPS
use std::thread;
use std::env;

use street_code_fighter::physics::collisions::*;
use street_code_fighter::physics::vecmath::*;
use street_code_fighter::physics::particle::*;
use street_code_fighter::input::*;
use street_code_fighter::animation::*;
use street_code_fighter::networking::*;
use street_code_fighter::physics::*;

use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize}; 
use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};

//pub mod characters; // for characterAbstract
//pub mod view; // for core
//pub mod input; // for inputHandler and movement
//pub mod animation;
//pub mod networking;
//pub mod physics;

//use crate::view::core; // need for SDLCore and TextureManager
//use crate::view::core::Demo; // need for SDLCore's Demo
// use crate::view::loads;

#[derive(Serialize, Deserialize, Debug)] 
pub struct CharacterState {
    pub position: RefCell<Particle>,
    pub state: animation::sprites::State,
    pub frames_per_state: i32,
    pub current_frame: i32,
    pub frame_count: i32,
    pub direction: input::movement::Direction,
}
impl CharacterState {
    pub fn new(position: RefCell<Particle>, 
        state: animation::sprites::State, 
        frames_per_state: i32,
        current_frame: i32,
        frame_count: i32,
        direction: input::movement::Direction) -> CharacterState {
        CharacterState {position,state,frames_per_state,current_frame,frame_count,direction}
    }
    pub fn position(&self) -> RefCell<Particle>{
        return self.position.clone();
    }
    pub fn state(&self) -> animation::sprites::State{
        self.state
    }
    pub fn frames_per_state(&self) -> i32{
        self.frames_per_state
    }
    pub fn current_frame(&self) -> i32{
        self.current_frame
    }
    pub fn frame_count(&self) -> i32{
        self.frame_count
    }
    pub fn direction(&self) -> input::movement::Direction{
        self.direction
    }
}

#[derive(Serialize, Deserialize, Debug)] 
pub struct CharStates {
    pub state1: CharacterState,
    pub state2: CharacterState,
}
impl CharStates {
    pub fn new(state1: CharacterState, state2: CharacterState) -> CharStates {
        CharStates {state1,state2}
    }
}



fn server_setup() -> UdpSocket{
    // ADDRESSING
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");
    
    println!("CONNECTED");

    socket
}

// first connect
fn client_connect(socket: &UdpSocket, 
                  client_addresses: &mut HashMap<SocketAddr,u8>,
                  player_count: u8) -> u8 {
    let mut buffer = [0u8; 100]; // a buffer than accepts 100
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

    // Client IPs and player #
    if !client_addresses.contains_key(&src_addr) { // for first time
        println!("First time connection to: {:?} > {:?}", src_addr, &buffer[0]); // test to print IP and initial info sent 
        client_addresses.insert(src_addr, player_count); // add to set
        socket.send_to(&[player_count], src_addr); // send player # 
        return player_count + 1; // increment player #
    } 

    return player_count;
}

fn server_rect(socket: &UdpSocket, 
               client_addresses: &HashMap<SocketAddr,u8>,
               r: &mut characters::characterAbstract::Fighter, 
               r2: &mut characters::characterAbstract::Fighter) {
    let mut buffer = [0u8; 100]; // a buffer than accepts 4096 
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

    let client_rect = deserialize::<CharacterState>(&buffer).expect("cannot crack ze coooode"); // print to console
    if client_addresses.get(&src_addr).unwrap().eq(&1) {
        r.char_state.particle = client_rect.position();
        r.char_state.state = client_rect.state();
        r.char_state.frames_per_state = client_rect.frames_per_state();
        r.char_state.current_frame = client_rect.current_frame();
        r.char_state.frame_count = client_rect.frame_count();
        r.char_state.direction = client_rect.direction();
    } else {   
        r2.char_state.particle = client_rect.position();
        r2.char_state.state = client_rect.state();
        r2.char_state.frames_per_state = client_rect.frames_per_state();
        r2.char_state.current_frame = client_rect.current_frame();
        r2.char_state.frame_count = client_rect.frame_count();
        r2.char_state.direction = client_rect.direction(); 
    }
    // // send to all addresses
    for client_address in client_addresses.keys() {
         if &src_addr != client_address { // DUPLEX, only send to other server
            socket.send_to(serialize(&client_rect).unwrap().as_slice(), client_address).expect("couldn't send message"); 
        }
    } // end sending for loop
} // close server fn

/* pub fn wait(seconds: u64) {
 let time_to_wait = time::Duration::from_secs(seconds);
 thread::sleep(time_to_wait);
} */



const TITLE: &str = "Street Code Fighter - Server";
const TIMEOUT: u64 = 5000;
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const FRAME_RATE: f64 = 1.0/90.0;


pub fn run_game(socket: &UdpSocket, client_addresses: &HashMap<SocketAddr,u8>) -> Result<(), String>{
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

    let platform = Rect::new(40, 620, CAM_W-80, CAM_H-680);


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
    
    game_window.render(&background, &texture, &fighter, &texture2, &fighter2, &hazard, &hazard_texture);


    let collisions = BVHierarchy::new(CollisionObject::new_from(CollisionObjectType::Platform, platform.clone(),
        RefCell::new(Particle::new(
            PhysVec::new(platform.x as f32, platform.y as f32), 0.5, 2000000000.0))));

        // collisions.insert(CollisionObject::new_from(CollisionObjectType::HurtBox, hazard.sprite.clone(),
        //     RefCell::new(Particle::new(
        //         PhysVec::new(hazard.position.x as f32, hazard.position.y as f32), 0.5, 200.0))));


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

    //##############################################-PROCESS-EVENTS-#######################################

        //select frame to be rendered
        fighter.char_state.advance_frame();
        fighter2.char_state.advance_frame();
        
        //move character based on current frame
        //input::movement::move_char(&mut fighter);
        //input::movement::move_char(&mut fighter2);

        fighter.char_state.update_bounding_boxes(&collisions);
        fighter2.char_state.update_bounding_boxes(&collisions);
        collisions.resolve_collisions();
        // println!("\nCollisions head: \n{:?}", collisions.head);

        //move hazard
        if hazard.sprite.y() < 600 && hazard.fell == false {
           hazard.sprite.offset(0, 7);
           //println!("{}", hazard.sprite.y())
       }
       if hazard.sprite.y() >= 600 {
           hazard.reset();
       }

       server_rect(&socket, 
        &client_addresses, 
        &mut fighter, 
        &mut fighter2);
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
        game_window.render(&background, &texture, &fighter, &texture2, &fighter2, &hazard, &hazard_texture);
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
        let socket = server_setup(); // make connection w/ socket
        let mut client_addresses = HashMap::new(); // store addresses
        let mut player_count: u8 = 1;

        // connecting before game loop!
        'connecting: loop {
            player_count = client_connect(&socket, &mut client_addresses, player_count);
            // increments connection +1
            if player_count == 3 { // if 3, two players are found
                println!("Two players found!");
                break 'connecting;
            }
        }

        run_game(&socket, &client_addresses)?;
        //networking::chatClient::server_connect();
    }

    // run_credits()?;

    Ok(())
}
