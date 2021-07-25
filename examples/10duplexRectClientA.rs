// Using code from: https://github.com/nfarnan/cs1666_examples/blob/main/sdl/examples/sdl08_rect_collision.rs

use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::net::{SocketAddr, UdpSocket};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize}; 

/////// NETWORKING CODE
fn client_setup() -> (UdpSocket, u8) {
	// ADDRESSING
	let client_addresses: [SocketAddr; 1] = [
	    SocketAddr::from(([127, 0, 0, 1], 1667)),
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

    let mut player_number;

    'player_num: loop {
      // RECEIVE INPUT FROM SERVER
      let mut buffer = [0u8; 100]; // a buffer than accepts 100 
      let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
      player_number = *(&buffer[0]); // gets the first integer
      println!("Player number: {:?}",player_number); 
      break 'player_num;
    }

    println!("RETURNING PLAYER NUMBER");

    (socket, player_number)
} // close client_setup

fn client_rect(socket: &UdpSocket, player_box: Rect, player_number: u8){
    let socket_clone = socket.try_clone().expect("couldn't clone the socket");

	//send player box information
      send(&socket_clone, &player_box, player_number);  

} // close client rect

pub fn send(socket: &UdpSocket, player_box: &Rect, player_number: u8){
	let rectangle;
	if player_number == 1{
		rectangle = RectangleValues::new(player_box.x(), player_box.y(),0,0);
	} else { // player #2
		rectangle = RectangleValues::new(0,0,player_box.x(), player_box.y());
	}

    // SENDING
  	let envelope = serialize(&rectangle); // creates a Vec

  	match envelope {
       Ok(encoded_message) => {
        let message = encoded_message.as_slice(); // changes from Vec to &[u8]
        socket.send(message);
     	},
     Err(e) => panic!("oh nos! No message"),
  	}
}
pub fn receive(socket: &UdpSocket, enemy: &mut Rect){
    let mut buffer = [0u8; 100]; // a buffer than accepts 4096 
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

    let client_rect = deserialize::<RectangleValues>(&buffer).expect("cannot crack ze coooode"); // print to console
    enemy.set_x(client_rect.x1());
    enemy.set_y(client_rect.y1()); 
}

// Creating new Rectangle struct, since Rect isn't serialized (for testing purposes :)
#[derive(Serialize, Deserialize, Debug)] 
pub struct RectangleValues {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32, 
}

impl RectangleValues{ 
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> RectangleValues {
        RectangleValues {x1,y1,x2,y2}
    }
    pub fn x1(&self) -> i32{
        self.x1
    }
    pub fn y1(&self) -> i32{
        self.y1
    }  
    pub fn x2(&self) -> i32{
        self.x2
    }
    pub fn y2(&self) -> i32{
        self.y2
    }    
}

/////// CODE from "SDL08 Rect Collisions" Example
/////// Note: it's virtually the same, but sends information to client 
const TITLE: &str = "CLIENT - CYAN - PLAYER 1";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

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

pub struct SDL08 {
	core: SDLCore,
}

impl Demo for SDL08 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL08{ core })
	}

	fn run(&mut self, socket: &UdpSocket, player_number: u8) -> Result<(), String> {
		let w = 25;

		let x_pos = (CAM_W/2 - w/2) as i32;
		let y_pos = (CAM_H/2 - w/2) as i32;		
		let mut player_box = Rect::new(x_pos, y_pos, w, w);
		let mut player2_box = Rect::new(x_pos, y_pos, w, w);

		let mut x_vel = 0;
		let mut y_vel = 0;

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

			let mut x_deltav = 0;
			let mut y_deltav = 0;
			if keystate.contains(&Keycode::W) {
				y_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::A) {
				x_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::S) {
				y_deltav += ACCEL_RATE;
			}
			if keystate.contains(&Keycode::D) {
				x_deltav += ACCEL_RATE;
			}

			// Slow down to 0 vel if no input and non-zero velocity
			x_deltav = resist(x_vel, x_deltav);
			y_deltav = resist(y_vel, y_deltav);

			// Don't exceed speed limit
			x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
			y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

			if player_number == 1 {
				player_box.set_x(player_box.x() + x_vel); // horizontal movement
				player_box.set_y(player_box.y() + y_vel); // vertical movement
				client_rect(&socket, player_box, player_number); // Send data on where the rectangle is to server
				receive(&socket, &mut player2_box);
			} else { // player_number == 2
				player2_box.set_x(player2_box.x() + x_vel); // horizontal movement
				player2_box.set_y(player2_box.y() + y_vel); // vertical movement
				client_rect(&socket, player2_box, player_number); // Send data on where the rectangle is to server
				receive(&socket, &mut player_box);
			}


			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			self.core.wincan.set_draw_color(Color::CYAN);
			self.core.wincan.fill_rect(player_box)?;

			self.core.wincan.set_draw_color(Color::RED);
			self.core.wincan.fill_rect(player2_box)?;

			
			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	let (socket, player_number) = client_setup(); // set up connection with server 
	runner(TITLE, SDL08::init, &socket, player_number);
}

/////// SDLCore stuffs/////////////////
pub struct SDLCore {
	sdl_cxt: sdl2::Sdl,
	pub wincan: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,
	pub cam: Rect,
}

impl SDLCore {
	pub fn init(
		title: &str,
		vsync: bool,
		width: u32,
		height: u32,
	) -> Result<SDLCore, String>
	{
		let sdl_cxt = sdl2::init()?;
		let video_subsys = sdl_cxt.video()?;

		let window = video_subsys.window(title, width, height)
			.build()
			.map_err(|e| e.to_string())?;

		let wincan = window.into_canvas().accelerated();

		// Check if we should lock to vsync
		let wincan = if vsync {
			wincan.present_vsync()
		}
		else {
			wincan
		};
		
		let wincan = wincan.build()
			.map_err(|e| e.to_string())?;

		let event_pump = sdl_cxt.event_pump()?;

		let cam = Rect::new(0, 0, width, height);

		Ok(SDLCore{
			sdl_cxt,
			wincan,
			event_pump,
			cam,
		})
	}
}

pub trait Demo {
	fn init() -> Result<Self, String> where Self: Sized;
	fn run(&mut self, socket: &UdpSocket, player_number: u8) -> Result<(), String>;
}

pub fn runner<F, D>(desc: &str, initter: F, socket: &UdpSocket, player_number: u8)
	where
		F: Fn() -> Result<D, String>,
		D: Demo,
{
	println!("\nRunning {}:", desc);
	print!("\tInitting...");
	match initter() {
		Err(e) => println!("\n\t\tFailed to init: {}", e),
		Ok(mut d) => {
			println!("DONE");

			print!("\tRunning...");
			match d.run(socket, player_number) {
				Err(e) => println!("\n\t\tEncountered error while running: {}", e),
				Ok(_) => println!("DONE\nExiting cleanly"),
			};
		},
	};
}