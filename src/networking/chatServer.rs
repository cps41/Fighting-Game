use std::net::UdpSocket;
 
pub fn server_start() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
 
    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;
 
        let buf = &mut buf[..amt];
        socket.send_to(buf, &src)?;

        //if str::from_utf8(&buf).eq("exit") {break};
    }
}