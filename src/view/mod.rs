pub mod core;

pub mod globals {
    pub const TITLE: &str = "Street Code Fighter";
    pub const TIMEOUT: u64 = 5000;
    pub const CAM_W: u32 = 1280;
    pub const CAM_H: u32 = 720;
    pub const W_OFFSET: i32 = CAM_W as i32/2-SPRITE_W as i32/2;
    pub const H_OFFSET: i32 = CAM_H as i32/2-100;
    pub const SPRITE_W: u32 = 80;
    pub const SPRITE_H: u32 = 210;
    pub const PLATFORM_TOP: f32 = 560.0;
    pub const FRAME_RATE: f64 = 1.0/60.0;
    pub const GRAVITY: f32 = 9.81;
    pub const FRICTION: f32 = 10f32;
}
