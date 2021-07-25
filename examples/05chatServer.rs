use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime,UNIX_EPOCH};
use std::str;
use std::thread;

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

        let mut buffer = [0u8; 2048]; // a buffer than accepts 2048 

        match socket.recv(&mut buffer) {
            Ok(received) => {         
                // Test 1: print bytes recieved and array
                // println!("received {} bytes {:?}", received, &buffer[..received]); // test to print bytes and buffer
                
                // Test 2: print current system time and text from client sent buffer 
                let system_time = SystemTime::now(); // get system time
                let ugly_text = str::from_utf8(&buffer).unwrap(); // get the text from buffer
                let pretty_text = ugly_text.trim_matches(char::from(0)); // cut the trailing 0s
                println!("{:?}: {:?}", system_time, pretty_text); // print to console

                // send to chatB
                socket.send_to(pretty_text.as_bytes(), "127.0.0.1:1668").expect("couldn't send message to client"); 
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
        } // deal with Result that's recieved from the buffer

        // Test 3: Send array back to the client
        // Note: Need to figure out if we need to send socket address or how to get that

        // note: use ctrl+c to exit
    } // close listening loop

    Ok(())
} // close main fn