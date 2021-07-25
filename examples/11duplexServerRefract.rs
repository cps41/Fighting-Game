use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;
use std::{thread, time};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Instant, Duration};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const TITLE: &str = "SERVER - AUTHORITATIVE";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;


fn main() {
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

    for address in client_addresses.keys(){
        socket.send_to(&[0], address).expect("message not sent");
    }


    let mut game_window = {
    	match SDLCore::init(TITLE, true, CAM_W, CAM_H){
    		Ok(t) => t,
    		Err(e) => panic!("{}", e),
    	}
    };

    run(&mut game_window, &socket, &client_addresses);
}

pub fn run(core: &mut SDLCore,
		   socket: &UdpSocket,
		   client_addresses: &HashMap<SocketAddr, u8>,
		  ) -> Result<(), String>{
   
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

    let received_limit = Duration::from_secs(5);

    'gameloop: loop{
        // keeping so we can exit
        for event in core.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                _ => {},
            }
        }

		let mut input_1 = InputValues{w: false, s: false, a: false, d: false};
		let mut input_2 = InputValues{w: false, s: false, a: false, d: false};
		let mut message_1 = false;
        let mut message_2 = false;

		
        let not_received = Instant::now();
        loop{
            receive(&socket, &client_addresses, &mut input_1, 
                    &mut input_2, &mut message_1, &mut message_2);
            println!("message 1 is: {}, message 2 is: {}", message_1, message_2);
            if (message_1 && message_2) || (not_received.elapsed() >= received_limit) {break;}
        }

        /*
        core.wincan.set_draw_color(Color::BLACK);
        core.wincan.fill_rect(p1_box)?;
        core.wincan.fill_rect(p2_box)?;
        */

		calc_vel(&input_1, &mut p1_x_vel, &mut p1_y_vel);
		p1_box.set_x(p1_box.x() + p1_x_vel);
		p1_box.set_y(p1_box.y() + p1_y_vel);
        //println!("Player 1 position is X:{}, Y:{}", p1_box.x(), p1_box.y);

		calc_vel(&input_2, &mut p2_x_vel, &mut p2_y_vel);
		p2_box.set_x(p2_box.x() + p2_x_vel);
		p2_box.set_y(p2_box.y() + p2_y_vel);
        //println!("Player 2 position is X:{}, Y:{}", p2_box.x(), p2_box.y);

        core.wincan.set_draw_color(Color::BLACK);
        core.wincan.clear();
        core.wincan.set_draw_color(Color::CYAN);
        core.wincan.fill_rect(p1_box)?;
        core.wincan.set_draw_color(Color::RED);
        core.wincan.fill_rect(p2_box)?;
        core.wincan.present();

        let state = GameState::new(p1_box.x(), p1_box.y(), p1_x_vel, p1_y_vel,
        					  	   p2_box.x(), p2_box.y(), p2_x_vel, p2_y_vel);

        send(&socket, &client_addresses, &state);   	
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

/*
fn receive(socket: &UdpSocket, 
           client_addresses: &HashMap<SocketAddr,u8>,
           input_1: &mut InputValues,
           input_2: &mut InputValues,
		  ){
    let mut message_1 = false;
    let mut message_2 = false;

    loop{
        println!("Started Receive Loop");
        let mut buffer = [0u8; 100]; // a buffer than accepts 4096 
        
        match socket.peek(&mut buffer){
            Ok(t) => 
            Err(e) =>
        }

        /*
        let (number_of_bytes, src_addr) = {
            match socket.recv_from(&mut buffer){
                Ok((usize, SocketAddr)) => {println!("Received Data"); (usize, SocketAddr)},
                Err(e) => panic!("{}", e)
            };
        };
        */
      
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
        print!("Received Some Data");
        if client_addresses.get(&src_addr).unwrap().eq(&1) && !message_1{
            let received_input = deserialize::<InputValues>(&buffer).expect("cannot crack ze coooode");
            input_1.copy(received_input);
            println!("Received Data from Player 1");
            message_1 = true;
        }else if client_addresses.get(&src_addr).unwrap().eq(&2) && !message_2{ 
            let received_input = deserialize::<InputValues>(&buffer).expect("cannot crack ze coooode");
            input_2.copy(received_input);
            println!("Received Data from Player 2");
            message_2 = true;
        }

        if message_1 && message_2 {break;}
    }
}
*/

fn receive(socket: &UdpSocket, 
           client_addresses: &HashMap<SocketAddr,u8>,
           input_1: &mut InputValues,
           input_2: &mut InputValues,
           message_1: &mut bool,
           message_2: &mut bool,
          ){
    println!("Made it into receive");
    let mut buffer = [0u8; 100]; // a buffer than accepts 4096 
    
    //let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
    
    match socket.peek(&mut buffer){
        Ok(t) => {  
            let (number_of_bytes, src_addr) = 
                socket.recv_from(&mut buffer).expect("Didn't receive data");                    
                    
            if client_addresses.get(&src_addr).unwrap().eq(&1) && !*message_1{
                println!("Received Data from Player 1");
                let received_input = deserialize::<InputValues>(&buffer)
                    .expect("cannot crack ze coooode");
                input_1.copy(received_input);
                *message_1 = true;
                //println!("Received Data from Player 1");
            }else if client_addresses.get(&src_addr).unwrap().eq(&2) && !*message_2{
                println!("Received Data from Player 2");
                let received_input = deserialize::<InputValues>(&buffer)
                    .expect("cannot crack ze coooode");
                input_2.copy(received_input);
                *message_2 = true;
                //println!("Received Data from Player 2");
            }
        },
        Err(e) => {println!("Didn't receive data")},
    };
}

fn send(socket: &UdpSocket,
		client_addresses: &HashMap<SocketAddr, u8>,
		state: &GameState){
	let envelope = serialize(state);
	match envelope{
		Ok(encoded_message) =>{ let message = encoded_message.as_slice();
								for address in client_addresses.keys(){
									socket.send_to(message, address).expect("message not sent");
								}
		},
		Err(e) => panic!("No message"),
	}
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
