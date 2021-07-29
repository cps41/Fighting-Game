use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
use std::collections::HashSet;
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};

use crate::physics;
use crate::animation;
use crate::characters;


#[derive(Serialize, Deserialize, Debug)]
pub struct GameState{
    pub p1_position: physics::particle::Particle,
    pub p1_state: animation::sprites::State,
    pub p1_frame: i32,
    pub p2_position: physics::particle::Particle,
    pub p2_state: animation::sprites::State,
    pub p2_frame: i32,
    pub hazard: physics::hazard::HazardVar,
}

impl GameState{
    pub fn new(
        p1: &characters::characterAbstract::Fighter,
        p2: &characters::characterAbstract::Fighter,
        hazard: &physics::hazard::Hazard,
        ) -> GameState{
        GameState {
            p1_position:    p1.char_state.position(),
            p1_state:       p1.char_state.state.clone(),
            p1_frame:       p1.char_state.current_frame,
            p2_position:    p2.char_state.position(),
            p2_state:       p2.char_state.state.clone(),
            p2_frame:       p2.char_state.current_frame,
            hazard:         physics::hazard::HazardVar::new(hazard),
        }
    }
}

pub fn receive_input(socket: &UdpSocket,
				  client_addresses: &HashMap<SocketAddr, u8>,
				  input_1: &mut HashSet<u8>,
				  input_2: &mut HashSet<u8>,
				  message_1: &mut bool,
				  message_2: &mut bool,
				  ){
	let mut buffer = [0u8; 100];

	let (number_of_bytes, src_addr) = {
		match socket.recv_from(&mut buffer){
			Ok(t) => t,
			Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
				return;
			}
			Err(e) => panic!("recv_from function failed: {:?}", e),
		}
	};

    let received_input = deserialize::<HashSet<u8>>(&buffer).expect("Couldn't interpret data");
   
    if client_addresses.get(&src_addr).unwrap().eq(&1) && !*message_1{
    	for keys in received_input.iter(){
    		input_1.insert(*keys);
    	}        
        *message_1 = true;
        println!("Received Input from Player 1");
    }else if client_addresses.get(&src_addr).unwrap().eq(&2) && !*message_2{
		for keys in received_input.iter(){
			input_2.insert(*keys);
		}        
        *message_2 = true;
        println!("Received Input from Player 2");
    }
}

pub fn send_input(socket: &UdpSocket, inputs: &HashSet<u8>,){
	let envelope = serialize(inputs);
    match envelope{
        Ok(encoded_message) =>{ let message = encoded_message.as_slice();
                                socket.send(message);},
        Err(e) => panic!("Send Failed: {:?}", e),
    }
}

pub fn send_game_state( socket: &UdpSocket,
                        client_addresses: &HashMap<SocketAddr, u8>,
                        state: &GameState){
    let envelope = serialize(state);
    match envelope{
        Ok(encoded_message) =>{ let message = encoded_message.as_slice();
                                for address in client_addresses.keys(){
                                    match socket.send_to(message, address){
                                        Ok(t) => {}//println!("Sent Properly"),
                                        Err(e) => panic!("Couldn't Send: {:?}", e),
                                    }
                                }
        },
        Err(e) => panic!("Encoding Failed: {:?}", e),
    }
}

pub fn receive_game_state(socket: &UdpSocket) -> GameState{
    let mut buffer = [0u8; 150];
    let mut number_of_bytes;
   
    match socket.recv(&mut buffer){
        Ok(t) => {number_of_bytes = t;},
        //Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
        Err(e) => panic!("recv function failed: {:?}", e),
    }

    let state = deserialize::<GameState>(&buffer).expect("cannot crack ze coode");
    //println!("Data Received");
    state    
}

pub fn ready_to_read(socket: &UdpSocket) -> bool{
    let mut buffer = [0u8; 100];
    match socket.peek(&mut buffer){
        Ok(t) => t,
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            //println!("not ready to peak");
            return false;
        }
        Err(e) =>{ 
            panic!("peek function failed: {:?}", e); 
            return false;
        }
    };
    return true
}