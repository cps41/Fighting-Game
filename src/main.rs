extern crate sdl2;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::time::Duration;
use std::thread;
use std::fs;

const TIMEOUT: u64 = 5000;
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

mod physics;

pub fn run_credits() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("Fighter Game Team", CAM_W, CAM_H)
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated();

    let canvas = canvas.present_vsync();

    let mut canvas = canvas
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    //init and fill a vector with our textures
    let paths = fs::read_dir("./src/assets/images/credits").unwrap();
    let mut textures: Vec<sdl2::render::Texture> = Vec::new();

    for path in paths {
            textures.push(texture_creator.load_texture(path.unwrap().path().display().to_string())?);
    }

    canvas.set_draw_color(Color::RGBA(0, 128, 128, 255));
	canvas.clear();

    /* loop to display each texture, making sure they fit within the 
        window and are positioned at the center before displaying them */
    for t in textures {
        let mut img_h = t.query().height;
        let mut img_w = t.query().width;

        while img_h > CAM_H || img_w > CAM_W {
            img_h = img_h/2;
            img_w = img_w/2;
        }

        let center_x = (CAM_W/2) - (img_w/2);
        let center_y = (CAM_H/2) - (img_h/2);
        let display_area = Rect::new(center_x as i32, center_y as i32, img_w, img_h);

        canvas.copy(&t, None, display_area)?;
        canvas.present();

        thread::sleep(Duration::from_millis(TIMEOUT));

        canvas.clear();
    }

    Ok(())
}

fn main() -> Result<(), String> {
    
    run_credits()?;
    
    Ok(())
}
