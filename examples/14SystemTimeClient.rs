use std::net::{SocketAddr, UdpSocket};
use std::{io, str}; // For input 

use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize}; 

extern crate street_code_fighter as scf;
use scf::networking::packet::Packet;
use scf::animation;
use scf::input;
use std::time::{SystemTime,UNIX_EPOCH};
use std::{thread, time}; // sleep for funsies

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
    for i in 0..3 { // for testing
      let current_time: SystemTime = SystemTime::now();

      let envelope = serialize(&CharacterState {
          time: current_time,
      }); // creates a Vec

      match envelope {
         Ok(encoded_message) => {
            let message = encoded_message.as_slice(); // changes from Vec to &[u8]
            socket.send(message);
         },
         Err(e) => panic!("oh nos! No message"),
      }
      
      let seconds = time::Duration::from_secs(3);
      thread::sleep(seconds);// for testing
   } // test loop 
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