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

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");

    // LISTENING
    'listening: loop { // listening loop

        let mut buffer = [0u8; 100]; // a buffer than accepts 100 
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

        client_addresses.insert(src_addr);
        // println!("{:?}", client_addresses); // printing array

        // Print text from client sent buffer on server
        let pretty_text = str::from_utf8(&buffer).unwrap().trim_matches(char::from(0)).trim();

        let joint = format!("{:?} > {}", 
                  src_addr, 
                  pretty_text);

        println!("{}", joint);

        // send to appropriate address
        for client_address in &client_addresses {
            // println!("client_address: {:?}", &client_address);
            if &src_addr != client_address { // DUPLEX, only send to other server
                socket.send_to(joint.as_bytes(), client_address).expect("couldn't send message"); 
            }
        }
        // note: use ctrl+c to exit
    } // close listening loop

    Ok(())
} // close main fn