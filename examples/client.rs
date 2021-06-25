use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use std::{io, str};

pub fn main() -> std::io::Result<()> {

	let client_addresses: [SocketAddr; 1] = [
	    SocketAddr::from(([127, 0, 0, 1], 1667)),
	    // can add backup IPs
	];

    let mut socket = UdpSocket::bind(&client_addresses[..]).expect("couldn't bind to address");

    socket.connect("127.0.0.1:1666").expect("connect function failed");

	socket.send(&[1,6,6,6]).expect("couldn't send message"); // Test message

  	// loop { // sending chat side

 	// }

 	Ok(())
}
