use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
use std::collections::HashSet;
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};
use std::time::{SystemTime,UNIX_EPOCH};
use std::time::{Instant, Duration};


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
    pub time: SystemTime,
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
            time:           SystemTime::now(),
        }
    }

    pub fn copy(&mut self, other: &GameState){
        self.p1_position = other.p1_position.clone();
        self.p1_state = other.p1_state;
        self.p1_frame = other.p1_frame;
        self.p2_position = other.p2_position.clone();
        self.p2_state = other.p2_state;
        self.p2_frame = other.p2_frame;
        self.hazard.from_hazvar(&other.hazard);
        self.time = other.time;
    }

    pub fn update_time(&mut self){
        self.time = SystemTime::now();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputStruct{
    pub inputs: HashSet<u8>,
    pub time:   SystemTime,
}

impl InputStruct{
    pub fn new(keys: HashSet<u8>) -> InputStruct{
        InputStruct{
            inputs:  keys,
            time:    SystemTime::now(),
        }
    }

    pub fn update_time(&mut self){
        self.time = SystemTime::now();
    }
}

pub fn receive_input(socket: &UdpSocket,
				  client_addresses: &HashMap<SocketAddr, u8>,
				  input_1: &mut HashSet<u8>,
				  input_2: &mut HashSet<u8>,
				  message_1: &mut bool,
				  message_2: &mut bool,
                  readout_time: &Instant,
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

    let received_input = deserialize::<InputStruct>(&buffer).expect("Couldn't interpret data");
    
    if received_input.time.elapsed().unwrap() > readout_time.elapsed(){ return; }

    if client_addresses.get(&src_addr).unwrap().eq(&1) && !*message_1{
        for keys in received_input.inputs.iter(){
    		input_1.insert(*keys);
    	}        
        *message_1 = true;
        //println!("Received Input from Player 1");
    }else if client_addresses.get(&src_addr).unwrap().eq(&2) && !*message_2{
		for keys in received_input.inputs.iter(){
			input_2.insert(*keys);
		}        
        *message_2 = true;
        //println!("Received Input from Player 2");
    }
}

pub fn send_input(socket: &UdpSocket, inputs: &InputStruct,){
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

pub fn receive_game_state(  socket: &UdpSocket, 
                            next_state: &mut GameState, 
                            readout_time: &Instant
                         ) -> bool{
    let mut buffer = [0u8; 150];
    //let mut number_of_bytes;
   
    match socket.recv(&mut buffer){
        Ok(t) => {
            let state = deserialize::<GameState>(&buffer).expect("cannot crack ze coode");
            if state.time.elapsed().unwrap() > readout_time.elapsed(){
                return false;
            }
            next_state.copy(&state);
            return true
        },
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {return false},
        Err(e) => panic!("recv function failed: {:?}", e),
    }

    false
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