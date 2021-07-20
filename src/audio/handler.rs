extern crate sdl2;
use sdl2::TimerSubsystem;
use std::path::Path;
use sdl2::mixer::{Chunk,open_audio,close_audio,InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

// things we need for sdl2::mixer::open_audio
const frequency: i32 = 44_100;
const format: u16 = AUDIO_S16LSB;
const channels: i32 = DEFAULT_CHANNELS; // stereo
const chunk_size: i32 = 256; // can be 1_024, but setting low for low latency

// file names
const opening_file: &str = "src/assets/audio/songs/opening-test.mp3";
const hit_file: &str = "src/assets/audio/sfx/hit-test.mp3";
const ko_file: &str = "src/assets/audio/sfx/ko-test.mp3";
const combat_file: &str = "src/assets/audio/sfx/combat-test.mp3";

pub struct Clips  {
	pub opening: Chunk,
	pub combat: Chunk,
	pub hit: Chunk,
	pub ko: Chunk,
}

impl Clips {
	pub fn new() -> Clips {
		sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
	    sdl2::mixer::allocate_channels(5);
   
   		// paths
      	let opening = Path::new(&opening_file);
   	    let combat = Path::new(&combat_file);
   	    let hit = Path::new(&hit_file);
   	    let ko = Path::new(&ko_file);

   	    // create music chunks
	    let opening_music = sdl2::mixer::Chunk::from_file(opening).unwrap();
	    let combat_music = sdl2::mixer::Chunk::from_file(combat).unwrap();
	    let hit_music = sdl2::mixer::Chunk::from_file(hit).unwrap();
	    let ko_music = sdl2::mixer::Chunk::from_file(ko).unwrap();

	    // create clips object
		Clips {
		 opening: opening_music,
		 combat: combat_music,
		 hit: hit_music,
		 ko: ko_music,
		}
	} // close new fn

}