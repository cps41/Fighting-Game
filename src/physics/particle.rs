use crate::physics::vecmath::PhysVec;

pub struct Particle {
    position: PhysVec,
    velocity: PhysVec,
    acceleration: PhysVec,
    damping: f32,
    inverse_mass: f32,
    force_accumulator: PhysVec,
}

impl Particle {
    pub fn new(position: PhysVec, velocity: PhysVec, acceleration: PhysVec, damping: f32, mass: f32, force_accumulator: PhysVec) -> Self {
        let inverse_mass = 1f32/mass;
        Particle {
            position,
            velocity,
            acceleration,
            damping,
            inverse_mass,
            force_accumulator,
        }
    }
    /*
        updated x = a + v*t + (1/2)*x*t^2
        like in Physics 1!
        the acceleration will be negligible though because of our frame rate so we nix it
        x += v*t
    */
    pub fn update_position(&mut self, time: f32) {
        self.position.addScalarProduct(&mut self.velocity, time); // x += v*t
    }
    /*
        Integrater to move the particle forward in time via the Newton-Euler method.
        Approximation of integral.
    */
    pub fn integrate(&mut self, duration: f32) {
        if duration <= 0f32 { return }

        // update linear position
        self.update_position(duration);
        // calculate acceleration
        let mut acceleration = self.acceleration.clone();
        acceleration.addScalarProduct(&self.force_accumulator, self.inverse_mass); // a += F/m
        // update linear velocity based on new acceleration
        self.velocity.addScalarProduct(&acceleration, duration);
        // account for drag
        let drag = self.damping.powf(duration);
        self.velocity.replace(&self.velocity.add(&PhysVec::new(drag, drag)));
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
        self.force_accumulator.replace(&self.force_accumulator.add(&force));
    }
}