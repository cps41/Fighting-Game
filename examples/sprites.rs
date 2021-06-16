extern crate street_code_fighter as scf;

// useful libraries
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const PLAYER_MOVEMENT_SPEED: i32 = 1;

/// Returns the row of the spritesheet corresponding to the given direction
fn direction_spritesheet_row(direction: &scf::input::movement::Direction) -> i32 {
    match direction {
        Left => 3,
        Right => 0,
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    fighter: &scf::characters::characterAbstract::Fighter,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = fighter.char_state.sprite.size();
    let current_frame = Rect::new(
        fighter.char_state.sprite.x() + frame_width as i32 * fighter.char_state.current_frame,
        fighter.char_state.sprite.y() + frame_height as  i32 * direction_spritesheet_row(&fighter.char_state.direction),
        frame_width,
        frame_height,
    );

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = fighter.char_state.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

// Update player a fixed amount based on their speed.
fn update_fighter(fighter: &mut scf::characters::characterAbstract::Fighter) {
    match &fighter.char_state.direction {
        Left => {
            fighter.char_state.position = fighter.char_state.position.offset(-fighter.speed, 0);
        },
        Right => {
            fighter.char_state.position = fighter.char_state.position.offset(fighter.speed, 0);
        },
    }

    // Only continue to animate if the player is moving
    if fighter.speed != 0 {
		fighter.char_state.set_current_frame(fighter.char_state.current_frame + 1); // update frame
        fighter.char_state.current_frame = fighter.char_state.current_frame(); // should modulo to correct frame
    }
}

fn main() -> Result<(), String> {
	
	// CANVAS
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
	
    let window = video_subsystem.window("Testing Sprite Functions", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

	let cs = scf::characters::characterAbstract::CharacterState::new();
	let mut fighter = scf::characters::characterAbstract::Fighter::new(cs);

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(scf::animation::sprites::get_state_filename(&fighter))?;


    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
					fighter.speed = PLAYER_MOVEMENT_SPEED;
                    fighter.char_state.direction = scf::input::movement::Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
					fighter.speed = PLAYER_MOVEMENT_SPEED;
                    fighter.char_state.direction = scf::input::movement::Direction::Right;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    fighter.speed = 0;
                },
                _ => {}
            }
        }

        // Update
        update_fighter(&mut fighter);

        // Render
        render(&mut canvas, Color::RGB(222,222,222), &texture, &fighter)?;

    }

    Ok(())
}