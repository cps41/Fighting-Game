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
    	println!("input: ");
     	let mut input = String::new();
     	io::stdin().read_line(&mut input)?;
     	socket.send(input.as_bytes())?;	// must send a &[u8]

      // let mut buffer = [0u8; 2048]; // a buffer than accepts 2048 
      // match socket.recv(&mut buffer) {
      //       Ok(received) => {         
      //           // Test 3: print bytes recieved and array from server
      //           println!("received {} bytes {:?}", received, &buffer[..received]); // test to print bytes and buffer
      //       }, 
      //       Err(e) => println!("recv function failed: {:?}", e),
      // } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
    } // close sending loop

    Ok(())
} // close main fn