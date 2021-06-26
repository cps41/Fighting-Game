use std::net::{SocketAddr, UdpSocket};
use std::{io, str}; // For input 

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
	// socket.send(&[1,6,6,6]).expect("couldn't send message"); // Test1: Send array message
    'sending: loop { // sending client side
    	// Test 2: Send input
    	println!("input: ");
     	let mut input = String::new();
     	io::stdin().read_line(&mut input)?;
     	socket.send(input.as_bytes())?;	// must send a &[u8]
        // note: use ctrl+c to exit
      } // close sending loop

    Ok(())
} // close main fn