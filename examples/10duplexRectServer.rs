use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;
use std::{thread, time};
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/////// NETWORKING CODE
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
               r: &mut Rect, 
               r2: &mut Rect) {
    let mut buffer = [0u8; 100]; // a buffer than accepts 4096 
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

    let client_rect = deserialize::<RectangleValues>(&buffer).expect("cannot crack ze coooode"); // print to console
    if client_addresses.get(&src_addr).unwrap().eq(&1) {
        r.set_x(client_rect.x1());
        r.set_y(client_rect.y1()); 
    } else {   
        r2.set_x(client_rect.x2());
        r2.set_y(client_rect.y2());   
    }
    // // send to all addresses
    for client_address in client_addresses.keys() {
         if &src_addr != client_address { // DUPLEX, only send to other server
            socket.send_to(serialize(&client_rect).unwrap().as_slice(), client_address).expect("couldn't send message"); 
        }
    } // end sending for loop
} // close server fn

pub fn wait(seconds: u64) {
 let time_to_wait = time::Duration::from_secs(seconds);
 thread::sleep(time_to_wait);
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
const TITLE: &str = "SERVER - AUTHORITATIVE";
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

    fn run(&mut self,socket: &UdpSocket, client_addresses: &HashMap<SocketAddr,u8>) -> Result<(), String> {
        // set up initial rectangle, initialization state
        let w = 25;
        let x_pos = (CAM_W/2 - w/2) as i32;
        let y_pos = (CAM_H/2 - w/2) as i32;     
        let mut player_box = Rect::new(x_pos, y_pos, w, w);
        let mut player2_box = Rect::new(x_pos, y_pos, w, w);

        'gameloop: loop {

            // keeping so we can exit
            for event in self.core.event_pump.poll_iter() {
                match event {
                    Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                    _ => {},
                }
            }

            // background
            self.core.wincan.set_draw_color(Color::BLACK);
            self.core.wincan.clear();

            self.core.wincan.set_draw_color(Color::CYAN);
            self.core.wincan.fill_rect(player_box)?;

            self.core.wincan.set_draw_color(Color::RED);
            self.core.wincan.fill_rect(player2_box)?;

            self.core.wincan.present();

            server_rect(&socket, 
                        &client_addresses, 
                        &mut player_box, 
                        &mut player2_box); // get the rectangle from client side

            // set the server-side rectangle
            // player_box.set_x(r.x());
            // player_box.set_y(r.y());
            // player2_box.set_x(r2.x());
            // player2_box.set_y(r2.y());
        }

        // Out of game loop, return Ok
        Ok(())
    }
}

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

    runner(TITLE, SDL08::init, &socket, &client_addresses);
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
    fn run(&mut self,socket: &UdpSocket, client_addresses: &HashMap<SocketAddr,u8>) -> Result<(), String>;
}

pub fn runner<F, D>(desc: &str, initter: F, socket: &UdpSocket, client_addresses: &HashMap<SocketAddr,u8>)
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
            match d.run(socket, client_addresses) {
                Err(e) => println!("\n\t\tEncountered error while running: {}", e),
                Ok(_) => println!("DONE\nExiting cleanly"),
            };
        },
    };
}