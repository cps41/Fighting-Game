use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;

pub fn main() -> Result<(), String>{

    // ADDRESSING
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    // BINDING
    let socket = UdpSocket::bind(&server_addresses[..]).expect("couldn't bind to address");

    // LISTENING
    'listening: loop { // listening loop

        let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 

        match socket.recv(&mut buffer) {
            Ok(received) => { 
                // println!("received {} bytes {:?}", received, &buffer[..received]); // test to print bytes and buffer
                let system_time = SystemTime::now(); // get system time
                let ugly_text = str::from_utf8(&buffer).unwrap(); // get the text from buffer
                let pretty_text = ugly_text.trim_matches(char::from(0)); // cut the trailing 0s
                println!("{:?}: {:?}", system_time, pretty_text); // print to console
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
        } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
    } // close listening loop

    Ok(())
} // close main fn