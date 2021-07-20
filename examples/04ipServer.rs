use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;
use std::thread;

use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize}; 

pub fn main() -> Result<(), String>{

    // ADDRESSING
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");

    // THREADS? Not sure if we need?
    // thread::spawn(move || { // standard thread stuff
    //     // some work here

    // });

    // LISTENING
    'listening: loop { // listening loop

        let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 

        match socket.recv(&mut buffer) {
            Ok(received) => {         
               // Test 4: print attribute to the console
               let client_socket_address: SocketAddr = deserialize::<SocketAddr>(&buffer).expect("cannot crack ze coooode");
               println!("Should client socket: {:?}", client_socket_address);
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
        } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
    } // close listening loop

    Ok(())
} // close main fn