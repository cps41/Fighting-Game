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
    'listening: loop { // listening loop

        let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 

        match socket.recv(&mut buffer) {
            Ok(received) => {         
               let state: CharacterState = deserialize::<CharacterState>(&buffer).expect("cannot crack ze coooode");
               let message: SystemTime = state.time();
               println!("Should print current time: {:?}", message);
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
        } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
     } // close listening loop

    Ok(())
} // close main fn

#[derive(Serialize, Deserialize, Debug)] 
pub struct CharacterState {
   pub time: SystemTime,  
}
impl CharacterState {
  pub fn time(&self) -> SystemTime{
    self.time
  }
}