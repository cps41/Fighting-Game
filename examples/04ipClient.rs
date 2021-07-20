use std::net::{SocketAddr, UdpSocket};
use std::{io, str}; // For input 

use bincode::{serialize, deserialize};
use serde_derive::{Serialize, Deserialize}; 

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
    socket.connect(&server_addresses[..]).expect("connect function failed");

    // SENDING
    let envelope = serialize(&client_addresses); // creates a Vec

    match envelope {
      Ok(encoded_message) => {
        let message = encoded_message.as_slice(); // changes from Vec to &[u8]
        socket.send(message);
      },
        Err(e) => panic!("oh nos! No message"),
    }

    Ok(())
} // close main fn