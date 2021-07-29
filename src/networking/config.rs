use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
use std::collections::HashSet;
use bincode::{serialize, deserialize}; 
use serde_derive::{Serialize, Deserialize};

pub fn client_setup() -> (UdpSocket, u8){
    // ADDRESSING
    let client_addresses: [SocketAddr; 2] = [
        SocketAddr::from(([127, 0, 0, 1], 1667)),
        SocketAddr::from(([127, 0, 0, 1], 1668)),
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

    let mut player_number = {
      let mut buffer = [0u8; 100]; // a buffer than accepts 100 
      let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
      *(&buffer[0]) // gets the first integer
    };
    println!("Player number: {:?}",player_number);
    
    (socket, player_number)
}

pub fn server_setup() -> UdpSocket{
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

pub fn client_connect(socket: &UdpSocket, 
                  client_addresses: &mut HashMap<SocketAddr,u8>,
                  player_count: u8) -> u8 {
    let mut buffer = [0u8; 100]; // a buffer than accepts 100
    let (number_of_bytes, src_addr) = {
        match socket.recv_from(&mut buffer){
            Ok(t) => t,
            Err(e) => panic!("recv_from function failed: {:?}",e),
        }
    };

    // Client IPs and player #
    if !client_addresses.contains_key(&src_addr) { // for first time
        println!("First time connection to: {:?} > {:?}", src_addr, &buffer[0]); // test to print IP and initial info sent 
        client_addresses.insert(src_addr, player_count); // add to set
        socket.send_to(&[player_count], src_addr); // send player # 
        return player_count + 1; // increment player #
    } 

    return player_count;
}