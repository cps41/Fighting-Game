use std::net::{SocketAddr, UdpSocket, ToSocketAddrs};
use std::{io, str}; // For input 
use std::env;
use regex::Regex;

pub fn main() -> std::io::Result<()> {

let ip_regex = Regex::new(r"^(?P<ip>[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+):(?P<port>[0-9]+)$").unwrap();

let args: Vec<String> = env::args().collect();
let ip_arg = &args[1];
let mut ip_input = String::new();

let mut client_addresses: SocketAddr; 

'ip: loop {
  if ip_regex.is_match(ip_arg) {
    client_addresses = SocketAddr::from(ip_arg.to_socket_addrs().unwrap().next().unwrap());
    break 'ip;
  } else {
      println!("Please submit a valid IP (#.#.#.#:#): ");
      io::stdin().read_line(&mut ip_input)?;
      ip_input.pop(); // pop off the "\n" that's appended
      if ip_regex.is_match(&ip_input) {
        client_addresses = SocketAddr::from(ip_input.to_socket_addrs().unwrap().next().unwrap());
        break 'ip;
      }
  }
} // loop
  
    // SERVER ADDRESS
    let server_addresses: [SocketAddr; 1] = [
        SocketAddr::from(([127, 0, 0, 1], 1666)),
        // can add backup IPs
    ];

    // BINDING & CONNECTING
    let mut socket: UdpSocket;
    socket = UdpSocket::bind(&client_addresses).expect("couldn't bind to address");
    socket.connect(&server_addresses[..]).expect("connect function failed");

    // SENDING
	// socket.send(&[1,6,6,6]).expect("couldn't send message"); // Test1: Send array message
    'sending: loop { // sending client side
    	// Test 2: Send input
    	println!("input: ");
     	let mut input = String::new();
     	io::stdin().read_line(&mut input)?;
     	socket.send(input.as_bytes())?;	// must send a &[u8]

      // Test 3: Recieving input from server
      let mut buffer = [0u8; 4096]; // a buffer than accepts 4096 
      match socket.recv(&mut buffer) {
            Ok(received) => {         
                // Test 3: print bytes recieved and array from server
                println!("received {} bytes {:?}", received, &buffer[..received]); // test to print bytes and buffer
            }, 
            Err(e) => println!("recv function failed: {:?}", e),
      } // deal with Result that's recieved from the buffer

        // note: use ctrl+c to exit
    } // close sending loop

    Ok(())
} // close main fn