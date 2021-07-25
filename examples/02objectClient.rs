use std::net::{SocketAddr, UdpSocket};
use std::{io, str}; // For input 

use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize}; 

extern crate street_code_fighter as scf;
use scf::networking::packet::Packet;
use scf::animation;
use scf::input;
// use sdl2::rect::{Point, Rect};

pub fn main() -> std::io::Result<()> {

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

    // SENDING
    // 'sending: loop { // sending client side
          let envelope = serialize(&CharacterState {
              // position: Point::new(0,0),
              state: animation::sprites::State::Idle,
              frames_per_state: 5,
              current_frame: 0, 
              // sprite: Rect::new(0, 0, 210, 300),
              auto_repeat: true,
              next_state: animation::sprites::State::Idle,
              direction: input::movement::Direction::Up,
          }); // creates a Vec

          match envelope {
    	       Ok(encoded_message) => {
                let message = encoded_message.as_slice(); // changes from Vec to &[u8]
                socket.send(message);
             },
             Err(e) => panic!("oh nos! No message"),
          }
        // note: use ctrl+c to exit
      // } // close sending loop

    Ok(())
} // close main fn


#[derive(Serialize, Deserialize)]
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

// impl CharacterState {
//   pub fn frames_per_state(&self) -> i32{
//     self.frames_per_state
//   }
// }