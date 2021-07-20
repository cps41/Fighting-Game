use std::net::{SocketAddr, UdpSocket};
use std::{io, str}; // For input 

pub fn main() -> std::io::Result<()> {

	// ADDRESSING
	let client_addresses: [SocketAddr; 1] = [
	    SocketAddr::from(([127, 0, 0, 1], 1668)),
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
    'receiving: loop { // receiving client side
    	// Send input
    	// println!("input: ");
     // 	let mut input = String::new();
     // 	io::stdin().read_line(&mut input)?;
     // 	socket.send(input.as_bytes())?;	// must send a &[u8]

      // Recieving input from server
      let mut buffer = [0u8; 2048]; // a buffer than accepts 4096 
      match socket.recv(&mut buffer) {
            Ok(received) => {         
                let ugly_text = str::from_utf8(&buffer).unwrap(); // get the text from buffer
                let pretty_text = ugly_text.trim_matches(char::from(0)); // cut the trailing 0s
                println!("{:?}", pretty_text); // print to console
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
      } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
    } // close receiving loop

    Ok(())
} // close main fn