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
    pub fn new(position: PhysVec, damping: f32, mass: f32) -> Self {
        let zero = PhysVec::new(0f32, 0f32);
        let inverse_mass = 1f32/mass;
        Particle {
            position,
            velocity: zero.clone(),
            acceleration: zero.clone(),
            damping,
            inverse_mass,
            force_accumulator: zero.clone(),
        }
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
        if duration <= 0f32 { return }

        // update linear position
        self.update_position(duration);
        println!("position: {:?}", self.position);
        // calculate acceleration
        self.acceleration.add_scaled_product(&self.force_accumulator, self.inverse_mass); // a += F/m
        println!("acceleration: {:?}", self.acceleration);
        // update linear velocity based on new acceleration
        self.velocity.add_scaled_product(&self.acceleration, duration);
        println!("velocity: {:?}", self.velocity);
        // account for drag
        let drag = self.damping.powf(duration);
        println!("drag: {:?}", drag);
        self.velocity.dot_replace(drag);
        println!("velocity again: {:?}", self.velocity);
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
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn testInit() {
        let zero = PhysVec::new(0f32, 0f32);
        let p = Particle::new(zero.clone(), 1f32, 5f32);

        assert_eq!(p.position, zero);
        assert_eq!(p.acceleration, zero);
        assert_eq!(p.velocity, zero);
        assert_eq!(p.force_accumulator, zero);
        assert_eq!(p.damping, 1f32);
        assert_eq!(p.inverse_mass, 0.2);
    }

    #[test]
    pub fn testAddForce() {
        let zero = PhysVec::new(0f32, 0f32);
        let mut p = Particle::new(zero.clone(), 1f32, 5f32);
        let force1 = PhysVec::new(5f32, 7f32);
        let force2 = PhysVec::new(2f32, 2f32);
        p.add_force(&force1);

        assert_eq!(p.force_accumulator, force1);

        p.add_force(&force2);

        assert_eq!(p.force_accumulator, PhysVec::new(7f32, 9f32));
    }

    #[test]
    pub fn testClearForce() {
        let zero = PhysVec::new(0f32, 0f32);
        let mut p = Particle::new(zero.clone(), 1f32, 5f32);
        let force1 = PhysVec::new(5f32, 7f32);
        let force2 = PhysVec::new(2f32, 2f32);
        p.add_force(&force1);
        p.add_force(&force2);
        p.clear_forces();

        assert_eq!(p.force_accumulator, zero);
    }

    #[test]
    pub fn testIntegrate() {
        let zero = PhysVec::new(0f32, 0f32);
        let one = PhysVec::new(1f32, 1f32);
        let mut p = Particle::new(zero.clone(), 0.5f32, 2f32);
        let force1 = PhysVec::new(5f32, 7f32);
        let force2 = PhysVec::new(2f32, 2f32);
        p.velocity.replace(&one);
        p.add_force(&force1);
        p.add_force(&force2);
        p.integrate(1f32);

        assert_eq!(p.position, one);
        assert_eq!(p.acceleration, PhysVec::new(3.5, 4.5));
        assert_eq!(p.velocity, PhysVec::new(2.25, 2.75));
        assert_eq!(p.force_accumulator, zero);
    }
}