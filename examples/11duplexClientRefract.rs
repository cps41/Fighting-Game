use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::net::{SocketAddr, UdpSocket};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize}; 

//const TITLE: &str = "CLIENT - CYAN - PLAYER 1";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

fn main() {
    let (socket, player_number) = client_setup(); // set up connection with server
    
    let TITLE: &str ={
        if player_number == 1 {
            "CLIENT - CYAN - PLAYER 1"
        }else{
            "CLIENT - RED - PLAYER 2"
        }};


    let mut game_window = {
        match SDLCore::init(TITLE, true, CAM_W, CAM_H){
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        }
    };
    
    run(&mut game_window, &socket, player_number);
}

fn run(core: &mut SDLCore, 
       socket: &UdpSocket, 
       player_number: u8,
      ) -> Result<(), String> {
    
    let w = 25;
    let x_pos = (CAM_W/2 - w/2) as i32;
    let y_pos = (CAM_H/2 - w/2) as i32;     
    
    let mut p1_box = Rect::new(x_pos, y_pos, w, w);
    let mut p2_box = Rect::new(x_pos, y_pos, w, w);

    let mut p1_x_vel = 0;
    let mut p1_y_vel = 0;
    let mut p2_x_vel = 0;
    let mut p2_y_vel = 0;

    core.wincan.set_draw_color(Color::BLACK);
    core.wincan.clear();
    core.wincan.set_draw_color(Color::CYAN);
    core.wincan.fill_rect(p1_box);
    core.wincan.set_draw_color(Color::RED);
    core.wincan.fill_rect(p2_box);
    core.wincan.present();

    let mut buffer = [0u8; 100];
    let (number_of_bytes) = socket.recv(&mut buffer).expect("Didn't receive data");

    'gameloop: loop{
        // keeping so we can exit
        for event in core.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                _ => {},
            }
        }

        //receive inputs
        let keystate: HashSet<Keycode> = core.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        //convert inputs to serializable option
        let input = InputValues::new(&keystate);
        
        //send inputs to be processed by the server
        send(&socket, &input);
        
        /*
        //before movement is processed, set the previous state of the boxes to be filled black
        core.wincan.set_draw_color(Color::BLACK);
        core.wincan.fill_rect(p1_box);
        core.wincan.fill_rect(p2_box);
        */

        //this is the predictive aspect.  Assume what is happening on the server is
        //also happening on yours and show it
        if player_number == 1{
            calc_vel(&input, &mut p1_x_vel, &mut p1_y_vel);
            p1_box.set_x(p1_box.x() + p1_x_vel);
            p1_box.set_y(p1_box.y() + p1_y_vel);
        } else if player_number == 2 {
            calc_vel(&input, &mut p2_x_vel, &mut p2_y_vel);
            p2_box.set_x(p2_box.x() + p2_x_vel);
            p2_box.set_y(p2_box.y() + p2_y_vel);
        }

        core.wincan.set_draw_color(Color::BLACK);
        core.wincan.clear();
        core.wincan.set_draw_color(Color::CYAN);
        core.wincan.fill_rect(p1_box)?;
        core.wincan.set_draw_color(Color::RED);
        core.wincan.fill_rect(p2_box)?;
        core.wincan.present();

        //receive the current game state from the server
        let state = receive(socket);

        //if anything is wrong with box 1, fix it
        if p1_box.x() != state.p1_x_pos() || p1_box.y() != state.p1_y_pos() 
           || p1_x_vel != state.p1_x_vel() || p1_y_vel != state.p1_y_vel(){
            
            core.wincan.set_draw_color(Color::BLACK);
            core.wincan.fill_rect(p1_box)?;
            
            p1_box.set_x(state.p1_x_pos());
            p1_box.set_y(state.p1_y_pos());
            p1_x_vel = state.p1_x_vel();
            p1_y_vel = state.p1_y_vel();

            core.wincan.set_draw_color(Color::CYAN);
            core.wincan.fill_rect(p1_box)?;
            core.wincan.present();
        }

        //if anything is wrong with box 2, fix it
        if p2_box.x() != state.p2_x_pos() || p2_box.y() != state.p2_y_pos() 
           || p2_x_vel != state.p2_x_vel() || p2_y_vel != state.p2_y_vel(){        
            
            core.wincan.set_draw_color(Color::BLACK);
            core.wincan.fill_rect(p2_box)?;

            p2_box.set_x(state.p2_x_pos());
            p2_box.set_y(state.p2_y_pos());
            p2_x_vel = state.p2_x_vel();
            p2_y_vel = state.p2_y_vel();

            core.wincan.set_draw_color(Color::BLACK);
            core.wincan.fill_rect(p2_box)?;
            core.wincan.present();
        }                
    }

    // Out of game loop, return Ok
    Ok(())
}

fn calc_vel(input: &InputValues, x_vel: &mut i32, y_vel: &mut i32){
    let mut x_deltav = 0;
    let mut y_deltav = 0;
    
    if input.w(){
        y_deltav -= ACCEL_RATE;
    }
    
    if input.a(){
        x_deltav -= ACCEL_RATE;
    }
    
    if input.s() {
        y_deltav += ACCEL_RATE;
    }
    
    if input.d() {
        x_deltav += ACCEL_RATE;
    }

    x_deltav = resist(*x_vel, x_deltav);
    y_deltav = resist(*y_vel, y_deltav);
    
    *x_vel = (*x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
    *y_vel = (*y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
}

fn resist(vel: i32, deltav: i32) -> i32 {
    if deltav == 0 {
        if vel > 0 {
            -1
        }
        else if vel < 0 {
            1
        }
        else {
            deltav
        }
    }
    else {
        deltav
    }
}

fn client_setup() -> (UdpSocket, u8){
    // ADDRESSING
    let client_addresses: [SocketAddr; 2] = [
        SocketAddr::from(([127, 0, 0, 1], 1667)),
        SocketAddr::from(([127, 0, 0, 1], 1668)),
        // can add backup client IPs
    ];

    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup server IPs
    ];

    // BINDING & CONNECTING
    let mut socket = UdpSocket::bind(&client_addresses[..]).expect("couldn't bind to address");
    socket.connect(&server_addresses[..]).expect("couldn't bind to address");

    println!("CONNECTED");
    
    socket.send(&[9]); // send initial message, b/c why not

    let mut player_number = {
      let mut buffer = [0u8; 100]; // a buffer than accepts 100 
      let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
      *(&buffer[0]) // gets the first integer
    };
    println!("Player number: {:?}",player_number);
    
    (socket, player_number)
}

pub fn send(socket: &UdpSocket, inputs: &InputValues,){
    println!("Sending Data");
    let envelope = serialize(inputs);
    match envelope{
        Ok(encoded_message) =>{ let message = encoded_message.as_slice();
                                socket.send(message);},
        Err(e) => panic!("No message"),
    }
    println!("Data Sent");
}

pub fn receive(socket: &UdpSocket) -> GameState{
    println!("Receiving Data");
    let mut buffer = [0u8; 100];
    let (number_of_bytes) = socket.recv(&mut buffer).expect("Didn't receive data");

    let state = deserialize::<GameState>(&buffer).expect("cannot crack ze coode");
    println!("Data Received");
    state
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState{
    pub p1_x_pos: i32,
    pub p1_y_pos: i32,
    pub p1_x_vel: i32,
    pub p1_y_vel: i32,    
    pub p2_x_pos: i32,
    pub p2_y_pos: i32, 
    pub p2_x_vel: i32,
    pub p2_y_vel: i32, 
}

impl GameState{
    pub fn new (p1_x_pos: i32, 
                p1_y_pos: i32, 
                p1_x_vel: i32, 
                p1_y_vel: i32, 
                p2_x_pos: i32, 
                p2_y_pos: i32, 
                p2_x_vel: i32, 
                p2_y_vel: i32) -> GameState {
        GameState { p1_x_pos,
                    p1_y_pos,
                    p1_x_vel,
                    p1_y_vel,
                    p2_x_pos,
                    p2_y_pos,
                    p2_x_vel,
                    p2_y_vel}
    }

    pub fn copy(&mut self, other: &GameState){
        self.p1_x_pos = other.p1_x_pos();
        self.p1_y_pos = other.p1_y_pos();
        self.p1_x_vel = other.p1_x_vel();
        self.p1_y_vel = other.p1_y_vel();
        self.p2_x_pos = other.p2_x_pos();
        self.p2_y_pos = other.p2_y_pos();
        self.p2_x_vel = other.p2_x_vel();
        self.p2_y_vel = other.p2_y_vel();
    }

    pub fn p1_x_pos(&self) -> i32{
        self.p1_x_pos
    }

    pub fn p1_y_pos(&self) -> i32{
        self.p1_y_pos
    }

    pub fn p1_y_vel(&self) -> i32{
        self.p1_y_vel
    }

    pub fn p1_x_vel(&self) -> i32{
        self.p1_x_vel
    }
    pub fn p2_x_pos(&self) -> i32{
        self.p2_x_pos
    }

    pub fn p2_y_pos(&self) -> i32{
        self.p2_y_pos
    }

    pub fn p2_y_vel(&self) -> i32{
        self.p2_y_vel
    }

    pub fn p2_x_vel(&self) -> i32{
        self.p2_x_vel
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputValues{
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
}

impl InputValues{
    pub fn new(keystate: &HashSet<Keycode>) -> InputValues {    
        let w = if keystate.contains(&Keycode::W) {
            true
        }else{
            false
        };
        
        let s = if keystate.contains(&Keycode::S) {
            true
        }else{
            false
        };

        let a = if keystate.contains(&Keycode::A) {
            true
        }else{
            false
        };

        let d = if keystate.contains(&Keycode::D) {
            true
        }else{
            false
        };

        InputValues{w,s,a,d}
    }

    pub fn copy(&mut self, other: InputValues){
        self.w = other.w();
        self.s = other.s();
        self.a = other.a();
        self.d = other.d();
    }

    pub fn w(&self) -> bool{
        self.w
    }

    pub fn s(&self) -> bool{
        self.s
    }

    pub fn a(&self) -> bool{
        self.a
    }

    pub fn d(&self) -> bool{
        self.d
    }
}

pub struct SDLCore{
    sdl_cxt: sdl2::Sdl,
    pub wincan: sdl2::render::WindowCanvas,
    pub event_pump: sdl2::EventPump,
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
        })
    }
}
