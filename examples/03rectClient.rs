// Using code from: https://github.com/nfarnan/cs1666_examples/blob/main/sdl/examples/sdl08_rect_collision.rs

extern crate street_code_fighter as scf;

use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::net::{SocketAddr, UdpSocket};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize}; 

/////// NETWORKING CODE
fn client_setup() -> UdpSocket{
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

    socket
}
fn client_rect(player_box: Rect, socket: &UdpSocket){
	let rectangle = RectangleValues::new(player_box.x(), player_box.y());

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

// Creating new Rectangle struct, since Rect isn't serialized (for testing purposes :)
#[derive(Serialize, Deserialize, Debug)] 
pub struct RectangleValues {
    pub x: i32,
    pub y: i32, 
}

impl RectangleValues{ 
	pub fn new(x: i32, y: i32) -> RectangleValues {
		RectangleValues {x,y}
	}
	pub fn x(&self) -> i32{
		self.x
	}
	pub fn y(&self) -> i32{
		self.y
	}	
}

/////// CODE from "SDL08 Rect Collisions" Example
/////// Note: it's virtually the same, but sends information to client 
const TITLE: &str = "CLIENT - CONTROLLER";
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

	fn run(&mut self) -> Result<(), String> {
		let socket = client_setup(); // set up connection with server 

		let w = 25;

		let x_pos = (CAM_W/2 - w/2) as i32;
		let y_pos = (CAM_H/2 - w/2) as i32;		
		let mut player_box = Rect::new(x_pos, y_pos, w, w);

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

			// Try to move horizontally
			player_box.set_x(player_box.x() + x_vel);

			// // Try to move vertically
			player_box.set_y(player_box.y() + y_vel);

			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			self.core.wincan.set_draw_color(Color::CYAN);
			self.core.wincan.fill_rect(player_box)?;

			client_rect(player_box, &socket); // Send data on where the rectangle is to server
			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	runner(TITLE, SDL08::init);
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
	fn run(&mut self) -> Result<(), String>;
}

pub fn runner<F, D>(desc: &str, initter: F)
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
			match d.run() {
				Err(e) => println!("\n\t\tEncountered error while running: {}", e),
				Ok(_) => println!("DONE\nExiting cleanly"),
			};
		},
	};
}