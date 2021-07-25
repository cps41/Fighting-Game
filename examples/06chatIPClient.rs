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
    'sending: loop { // sending client side
    	// Send input
    	println!("input: ");
     	let mut input = String::new();
     	io::stdin().read_line(&mut input)?;
     	socket.send(input.as_bytes())?;	// must send a &[u8]

      // RECEIVE INPUT FROM CLIENT
      let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 
      match socket.recv(&mut buffer) {
            Ok(received) => {         
                println!("{:?}", (str::from_utf8(&buffer).unwrap()).trim_matches(char::from(0))); // test message from server
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
      } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
    } // close sending loop

    Ok(())
} // close main fn