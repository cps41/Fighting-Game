// abstracted packet

use std::net::SocketAddr;

pub struct Packet {
    address: SocketAddr, // address of source
    payload: Box<[u8]>,
}

impl Packet {
	pub fn new(address: SocketAddr, payload: Vec<u8>) -> Packet { // serializing returns Vec
		Packet {
			address: address,
			payload: payload.into_boxed_slice(), // slices vectors to boxes
		}
	}

	// getters
    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

} // close Packet impl