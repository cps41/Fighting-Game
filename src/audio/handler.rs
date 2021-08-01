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
const hit_file: &str = "src/assets/audio/sfx/hit-sound.mp3";
const ko_file: &str = "src/assets/audio/sfx/ko-sound.mp3";
const combat_file1: &str = "src/assets/audio/sfx/combat1.mp3";
const combat_file2: &str = "src/assets/audio/sfx/combat2.mp3";
const combat_file3: &str = "src/assets/audio/sfx/combat3.mp3";

pub struct Clips  {
	pub opening: Chunk,
	pub combat1: Chunk,
	pub combat2: Chunk,
	pub combat3: Chunk,
	pub hit: Chunk,
	pub ko: Chunk,
}

impl Clips {
	pub fn new() -> Clips {
		sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
	    sdl2::mixer::allocate_channels(10);
   
   		// paths
      	let opening = Path::new(&opening_file);
   	    let combat1 = Path::new(&combat_file1);
   	    let combat2 = Path::new(&combat_file2);
   	    let combat3 = Path::new(&combat_file3);
   	    let hit = Path::new(&hit_file);
   	    let ko = Path::new(&ko_file);

   	    // create music chunks
	    let opening_music = sdl2::mixer::Chunk::from_file(opening).unwrap();
	    let combat_music1 = sdl2::mixer::Chunk::from_file(combat1).unwrap();
	    let combat_music2 = sdl2::mixer::Chunk::from_file(combat2).unwrap();
	    let combat_music3 = sdl2::mixer::Chunk::from_file(combat3).unwrap();
	    let hit_music = sdl2::mixer::Chunk::from_file(hit).unwrap();
	    let ko_music = sdl2::mixer::Chunk::from_file(ko).unwrap();

	    // create clips object
		Clips {
		 opening: opening_music,
		 combat1: combat_music1,
		 combat2: combat_music2,
		 combat3: combat_music3,
		 hit: hit_music,
		 ko: ko_music,
		}
	} // close new fn

}