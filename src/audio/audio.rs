extern crate sdl2;
// use std::env;
use sdl2::TimerSubsystem;
use std::path::Path;
use sdl2::mixer::open_audio;
use sdl2::mixer::close_audio;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

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

// time of files, idk how not to hardcore...
const opening_secs: u32 = 13;
const hit_secs: u32 = 1;
const ko_secs: u32 = 2;
const combat_secs: u32 = 3;

pub fn play(music_file: &Path, timer: &mut TimerSubsystem, seconds: u32) -> Result<(), String> {
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;

    let music = sdl2::mixer::Music::from_file(music_file)?;

    println!("play => {:?}", music.play(1));

    // timer.delay(seconds*1000); // to do, not sure how to not hardcode delay in

    Ok(())
}

pub fn stop() { // kind of useless
	println!("stop music");
	sdl2::mixer::Music::halt();
}

/// EASY PLAY
pub fn opening(timer: &mut TimerSubsystem)  {
	let opening = Path::new(&opening_file);
	play(opening, timer, opening_secs).expect("couldn't play opening");
}
pub fn hit(timer: &mut TimerSubsystem)  {
	let hit = Path::new(&hit_file);
	play(hit, timer, hit_secs).expect("couldn't play opening");
}
pub fn ko(timer: &mut TimerSubsystem)  {
	let ko = Path::new(&ko_file);
	play(ko, timer, ko_secs).expect("couldn't play opening");
}
pub fn combat(timer: &mut TimerSubsystem)  {
	let combat = Path::new(&combat_file);
	play(combat, timer, combat_secs).expect("couldn't play opening");
}
