pub mod core;

pub mod globals {
    pub const TITLE: &str = "Street Code Fighter";
    pub const TIMEOUT: u64 = 5000;
    pub const CAM_W: u32 = 1280;
    pub const CAM_H: u32 = 720;
    pub const SPRITE_W: u32 = 210;
    pub const SPRITE_H: u32 = 300;
    pub const FRAME_RATE: f64 = 1.0/60.0;
    pub const GRAVITY: f32 = 9.81;
    pub const FRICTION: f32 = 10f32;
}