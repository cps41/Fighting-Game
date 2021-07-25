use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;

use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize}; 

// From our crate
extern crate street_code_fighter as scf;
use scf::networking::packet::Packet;
use scf::animation;
use scf::input;
// use sdl2::rect::{Point, Rect};


pub fn main() -> Result<(), String>{

    // ADDRESSING
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");

    // LISTENING
    // 'listening: loop { // listening loop

        let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 

        match socket.recv(&mut buffer) {
            Ok(received) => {         
               println!("{:?}",  deserialize::<CharacterState>(&buffer).expect("cannot crack ze coooode")); // Test 3: print object to console
               // Test 4: print attribute to the console
               let state: CharacterState = deserialize::<CharacterState>(&buffer).expect("cannot crack ze coooode");
               let message: i32 = state.frames_per_state();
               println!("Should print 5: {}", message);
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
        } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
    // } // close listening loop

    Ok(())
} // close main fn

// Must implement Serialize/Deserialize traits for translation, 
// must implement debug for formatting
#[derive(Serialize, Deserialize, Debug)] 
pub struct CharacterState {
    // pub position: Point,
    pub state: animation::sprites::State,
    pub frames_per_state: i32,
    pub current_frame: i32, 
    // pub sprite: Rect,
    pub auto_repeat: bool,
    pub direction: input::movement::Direction,
    pub next_state: animation::sprites::State,  
}
impl CharacterState {
  pub fn frames_per_state(&self) -> i32{
    self.frames_per_state
  }
}