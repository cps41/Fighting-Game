use crate::physics::vecmath::PhysVec;

pub struct Particle {
    position: PhysVec,
    velocity: PhysVec,
    acceleration: PhysVec,
    damping: f32,
    inverse_mass: f32,
}

impl Particle {
    pub fn new(position: PhysVec, velocity: PhysVec, acceleration: PhysVec, damping: f32, mass: f32) -> Self {
        let inverse_mass = 1f32/mass;
        Particle {
            position,
            velocity,
            acceleration,
            damping,
            inverse_mass
        }
    }
    /*
        updated x = a + v*t + (1/2)*x*t^2
        like in Physics 1!
    */
    pub fn update_position(&mut self, time: f32) {
        self.position.add(
            self.velocity.dot_product(time).add(
                self.acceleration.dot_product(
                    time*time*0.5
        )));
    }
    /*
        Integrater to move the particle forward in time via the Newton-Euler method.
        Approximation of integral.
    */
    pub fn integrate(&mut self, duration: f32) {
        if duration <= 0f32 { return }


    }
}