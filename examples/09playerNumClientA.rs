use std::net::{SocketAddr, UdpSocket};
use std::{io, str}; // For input 
use std::thread;

pub fn main() -> std::io::Result<()> {
  println!("Input messages :)! ");

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

    let socket_clone = socket.try_clone().expect("couldn't clone the socket");

    socket.send(&[8]); // send initial message, b/c why not

    let mut player_number;

    'player_num: loop {
      // RECEIVE INPUT FROM SERVER
      let mut buffer = [0u8; 1]; // a buffer than accepts 100 
      let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");
      player_number = &buffer[0]; // gets the first integer
      println!("Player number: {:?}",player_number); 
      break 'player_num;
    }

    // thread to listen
    thread::spawn(move || loop { // standard thread stuff
      send(&socket_clone);  

    });

    // main content
    'listening: loop {
      receive(&socket);  
    }

   Ok(())
} // close main fn

pub fn send(socket: &UdpSocket) -> std::io::Result<()>{
      // SENDING
      let mut input = String::new();
      io::stdin().read_line(&mut input)?;
      socket.send(input.as_bytes())?; // must send a &[u8]

      Ok(())
}

pub fn receive(socket: &UdpSocket){
      // RECEIVE INPUT FROM SERVER
      let mut buffer = [0u8; 100]; // a buffer than accepts 100 
      match socket.recv(&mut buffer) {
        Ok(received) => {         
                println!("Received: {}", str::from_utf8(&buffer).unwrap().trim_matches(char::from(0)).trim()); // test message from server
              }, 
              Err(e) => println!("recv function failed: {:?}", e),
      } // deal with Result that's recieved from the buffer

} // close receive
