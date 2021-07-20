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

        let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
        
        println!("Received {} bytes", number_of_bytes); // Print bytes # received and bytes (corresponding to message)
        
        println!("SocketAddr: {:?}", src_addr); // Print socket address 

        // Print current system time and text from client sent buffer 
        let system_time = SystemTime::now(); // get system time
        let ugly_text = str::from_utf8(&buffer).unwrap(); // get the text from buffer
        let pretty_text = ugly_text.trim_matches(char::from(0)); // cut the trailing 0s
        println!("System Time: {:?}; Message: {:?}", system_time, pretty_text); // print to console

        // Send array back to the client
        socket.send_to(("Message received").as_bytes(), src_addr).expect("couldn't send message"); // Send note to client that message was received

        // note: use ctrl+c to exit
    } // close listening loop

    Ok(())
} // close main fn