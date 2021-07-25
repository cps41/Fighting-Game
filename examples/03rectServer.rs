// Using code from: https://github.com/nfarnan/cs1666_examples/blob/main/sdl/examples/sdl08_rect_collision.rs

extern crate street_code_fighter as scf;

use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread, time};
use std::net::{SocketAddr, UdpSocket};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize}; 

/////// NETWORKING CODE
fn server_setup() -> UdpSocket{
	// ADDRESSING
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");

    // TBD - THREADS? try_clone: https://doc.servo.org/std/net/struct.UdpSocket.html#method.try_clone
    
    println!("CONNECTED");

    socket
}
fn server_rect(socket: &UdpSocket) -> Rect {
    // LISTENING
    	let mut r = Rect::new(0,0,25,25);

        let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 

        match socket.recv(&mut buffer) {
            Ok(received) => {         
               let client_rect = deserialize::<RectangleValues>(&buffer).expect("cannot crack ze coooode"); // print to console
               r.set_x(client_rect.x());
               r.set_y(client_rect.y());
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
        } // deal with Result that's recieved from the buffer

    r
} // close server fn

// pub fn wait(seconds: u64) {
// 	let time_to_wait = time::Duration::from_secs(seconds);
// 	thread::sleep(time_to_wait);
// }


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
const TITLE: &str = "SERVER - PUPPET";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

pub struct SDL08 {
	core: SDLCore,
}

impl Demo for SDL08 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL08{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let socket = server_setup(); // make connection w/ socket

		// set up initial rectangle, initialization state
		let w = 25;
		let x_pos = (CAM_W/2 - w/2) as i32;
		let y_pos = (CAM_H/2 - w/2) as i32;		
		let mut player_box = Rect::new(x_pos, y_pos, w, w);

		'listening_gameloop: loop {

			// keeping so we can exit
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'listening_gameloop,
					_ => {},
				}
			}

			// background
			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			self.core.wincan.set_draw_color(Color::CYAN);
			self.core.wincan.fill_rect(player_box)?;

			self.core.wincan.present();

			let r = server_rect(&socket); // get the rectangle from client side

			// set the server-side rectangle
			player_box.set_x(r.x());
			player_box.set_y(r.y());
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	runner(TITLE, SDL08::init);
}

/////// SDLCore stuffs///////
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