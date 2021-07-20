use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;
use std::thread;
use std::collections::HashSet;

pub fn main() -> Result<(), String>{
    // ADDRESSING
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    let mut client_addresses = HashSet::new(); // store addresses
    let mut player_count: u8 = 1;

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");

    // LISTENING
    'listening: loop { // listening loop

        let mut buffer = [0u8; 100]; // a buffer than accepts 100 
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

        // Client IPs and player #
        if !client_addresses.contains(&src_addr) { // for first time
        	client_addresses.insert(src_addr); // add to set
        	socket.send_to(&[player_count], src_addr); // send player # 
        	player_count += 1; // increment player #
        	println!("{:?} > {:?}", src_addr, &buffer[0]); // test to print IP and initial info sent 
    	} else { // after first time
	        // Print text from client sent buffer on server
	        let pretty_text = str::from_utf8(&buffer).unwrap().trim_matches(char::from(0)).trim();
	        let joint = format!("{:?} > {}", 
	                  src_addr, 
	                  pretty_text);
	        println!("{}", joint);

	        // send to all addresses
       		for client_address in &client_addresses {
                socket.send_to(joint.as_bytes(), client_address).expect("couldn't send message"); 
        	} // end sending for loop
    	} // close else

    } // close listening loop

    Ok(())
} // close main fn