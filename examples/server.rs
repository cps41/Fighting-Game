use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use std::str;

pub fn main() {

    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");

    // loop { // listening loop


    //  } // close listening loop

    let mut buf = [0; 10];
    match socket.recv(&mut buf) {
        Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
        Err(e) => println!("recv function failed: {:?}", e),
    }

} // close main fn
