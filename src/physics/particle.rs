use crate::physics::vecmath::PhysVec;
use crate::view::globals::*;
use sdl2::rect::Point;

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: PhysVec,
    pub velocity: PhysVec,
    pub acceleration: PhysVec,
    pub damping: f32,
    pub inverse_mass: f32,
    pub force_accumulator: PhysVec,
    pub health: i32,
}

impl Particle {
    pub fn new(position: PhysVec, damping: f32, mass: f32, health: i32) -> Self {
        let zero = PhysVec::new(0f32, 0f32);
        let inverse_mass = 1f32/mass;
        Particle {
            position,
            velocity: zero.clone(),
            acceleration: zero.clone(),
            damping,
            inverse_mass,
            force_accumulator: zero.clone(),
            health: health,
        }
    }

    // Create Point struct out of position coordinates
    pub fn to_point(&self) -> Point {
        let (x,y) = self.position.raw();
        Point::new(x as i32, y as i32)
    }

    pub fn reset_y(&mut self) {
        // self.position.y = 88.0;
        self.velocity.y = 0.0;
        self.acceleration.y = 0.0;
        self.force_accumulator.y = 0.0;
    }

    pub fn reset_x(&mut self) {
        // self.position.y = 88.0;
        self.velocity.x = 0.0;
        self.acceleration.x = 0.0;
        self.force_accumulator.x = 0.0;
    }

    pub fn update_health(&mut self, damage: i32) {
        self.health -= damage;
        self.health.clamp(0, 270);
    }

    /*
        updated x = a + v*t + (1/2)*x*t^2
        like in Physics 1!
        the acceleration will be negligible though because of our frame rate so we nix it
        x += v*t
    */
    pub fn update_position(&mut self, time: f32) {
        self.position.add_scaled_product(&mut self.velocity, time); // x += v*t
    }
    /*
        Integrater to move the particle forward in time via the Newton-Euler method.
        Approximation of integral.
    */
    pub fn integrate(&mut self, duration: f32) {
        let old = self.clone();
		let w_offset = CAM_W as f32/2f32;
		let h_offset = CAM_H as f32/2f32;
        if duration <= 0f32 { return }

        // update linear position
        self.update_position(duration);
        // clamp position
		self.position.x = self.position.x.clamp(-w_offset+SPRITE_W as f32/2.0, w_offset-SPRITE_W as f32/2.0);
		// self.position.y = self.position.y.clamp(-1000.0, h_offset-SPRITE_H as f32/2.0);
        // calculate acceleration
        self.acceleration.add_scaled_product(&self.force_accumulator, self.inverse_mass); // a += F/m
        // update linear velocity based on new acceleration
        self.velocity.add_scaled_product(&self.acceleration, duration);
        // account for drag
        let drag = self.damping.powf(duration);
        self.velocity.dot_replace(drag);
        // clamp velocity
		self.velocity.x = self.velocity.x.clamp(-1000.0, 1000.0);
		self.velocity.y = self.velocity.y.clamp(-2500.0, 1000.0);

        // println!("\nintegrated from {:?}\n to {:?}", old, self);
        // reset force accumulator
        self.clear_forces();
    }
    // Clear all forces applied to the particle
    pub fn clear_forces(&mut self) {
        self.force_accumulator.x = 0f32;
        self.force_accumulator.y = 0f32;
    }
    // Add force to the accumulator
    pub fn add_force(&mut self, force: &PhysVec) {
        self.force_accumulator.add_vec(force);
    }
    // Add force to the accumulator
    pub fn add_force_comps(&mut self, x: f32, y: f32) {
        self.force_accumulator.add_vec(&PhysVec::new(x, y));
    }
}
