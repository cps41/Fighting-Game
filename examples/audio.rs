extern crate street_code_fighter as scf;

use std::{thread, time};

fn main() {
	scf::audio::audio::play(); // play audio

	thread::sleep(time::Duration::from_secs(10)); // wait for 10 seconds

	scf::audio::audio::stop(); // stop playing audio
}


