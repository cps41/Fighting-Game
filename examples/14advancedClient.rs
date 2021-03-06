use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::net::{SocketAddr, UdpSocket};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};
use std::thread;
use std::io;
use std::collections::VecDeque;
use std::time::{Instant, Duration};


//const TITLE: &str = "CLIENT - CYAN - PLAYER 1";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;
const FRAME_RATE: f64 = 1.0/10.0;

fn main() {
    let (socket, player_number) = client_setup(); // set up connection with server
    socket.set_read_timeout(None).expect("set_read_timeout call failed");

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

    println!("Waiting for other player...");
    let mut buffer = [0u8; 100];
    let (number_of_bytes) = socket.recv(&mut buffer).expect("Didn't receive data");
    println!("Starting Game");

    //socket.set_nonblocking(true).unwrap();
    run(&mut game_window, &socket, player_number);
}

fn run(core: &mut SDLCore, 
       socket: &UdpSocket, 
       player_number: u8,
      ) -> Result<(), String> {
    
    let frame_time = Duration::from_secs_f64(FRAME_RATE);


    let w = 25;
    let x_pos = (CAM_W/2 - w/2) as i32;
    let y_pos = (CAM_H/2 - w/2) as i32;     
    
    let mut p1_box = Rect::new(x_pos, y_pos, w, w);
    let mut p2_box = Rect::new(x_pos, y_pos, w, w);

    let mut p1_x_vel = 0;
    let mut p1_y_vel = 0;
    let mut p2_x_vel = 0;
    let mut p2_y_vel = 0;


    let mut input_buffer: VecDeque<GameState> = VecDeque::new();

    for i in 0 .. 6{
        input_buffer.push_back(GameState::new(0,0,0,0,0,0,0,0));
    }



    core.wincan.set_draw_color(Color::BLACK);
    core.wincan.clear();
    core.wincan.set_draw_color(Color::CYAN);
    core.wincan.fill_rect(p1_box);
    core.wincan.set_draw_color(Color::RED);
    core.wincan.fill_rect(p2_box);
    core.wincan.present();

    'gameloop: loop{
        let loop_time = Instant::now();
        // keeping so we can exit
        for event in core.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                _ => {},
            }
        }
        
        let mut keystate: HashSet<Keycode> = core.event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

        //convert inputs to serializable option
        let input = InputValues::from_keystate(&keystate);
        
        //thread::sleep_ms(3000);

        //send inputs to be processed by the server
        send(&socket, &input);

        //receive the current game state from the server
        let state = input_buffer.pop_front().unwrap();
            
        p1_box.set_x(state.p1_x_pos());
        p1_box.set_y(state.p1_y_pos());
        p1_x_vel = state.p1_x_vel();
        p1_y_vel = state.p1_y_vel();

        p2_box.set_x(state.p2_x_pos());
        p2_box.set_y(state.p2_y_pos());
        p2_x_vel = state.p2_x_vel();
        p2_y_vel = state.p2_y_vel();

        core.wincan.set_draw_color(Color::BLACK);
        core.wincan.clear();
        core.wincan.set_draw_color(Color::CYAN);
        core.wincan.fill_rect(p1_box)?;
        core.wincan.set_draw_color(Color::RED);
        core.wincan.fill_rect(p2_box)?;
        core.wincan.present();

        input_buffer.push_back(receive(socket));
        thread::sleep(frame_time - loop_time.elapsed().clamp(Duration::new(0,0), frame_time));
    }

    // Out of game loop, return Ok
    Ok(())
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
    //println!("Sending Data");
    let envelope = serialize(inputs);
    match envelope{
        Ok(encoded_message) =>{ let message = encoded_message.as_slice();
                                socket.send(message);},
        Err(e) => panic!("Send Failed: {:?}", e),
    }
    //println!("Data Sent");
}

pub fn receive(socket: &UdpSocket) -> GameState{
    //println!("Receiving Data");
    let mut buffer = [0u8; 100];
    let mut number_of_bytes;

    loop{    
        match socket.recv(&mut buffer){
            Ok(t) => {number_of_bytes = t; break;},
            //Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
            Err(e) => panic!("recv function failed: {:?}", e),
        }
    }

    let state = deserialize::<GameState>(&buffer).expect("cannot crack ze coode");
    //println!("Data Received");
    state
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState{
    pub p1_x_pos:   i32,
    pub p1_y_pos:   i32,
    pub p1_x_vel:   i32,
    pub p1_y_vel:   i32,    
    pub p2_x_pos:   i32,
    pub p2_y_pos:   i32, 
    pub p2_x_vel:   i32,
    pub p2_y_vel:   i32,
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
                    p2_y_vel,
                }
    }


    pub fn copy(&mut self, other: &GameState){
        self.p1_x_pos   =   other.p1_x_pos();
        self.p1_y_pos   =   other.p1_y_pos();
        self.p1_x_vel   =   other.p1_x_vel();
        self.p1_y_vel   =   other.p1_y_vel();
        self.p2_x_pos   =   other.p2_x_pos();
        self.p2_y_pos   =   other.p2_y_pos();
        self.p2_x_vel   =   other.p2_x_vel();
        self.p2_y_vel   =   other.p2_y_vel();
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
    pub fn from_keystate(keystate: &HashSet<Keycode>) -> InputValues {    
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

        InputValues{w,s,a,d,}
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
        //wincan.present();

        Ok(SDLCore{
            sdl_cxt,
            wincan,
            event_pump,
        })
    }
}