use crate::physics::particle; // used to apply gravity

// Structs 
// Keeps track of one force generator and the particle it applies to.
#[derive(Debug)]
pub struct ParticleForceRegistration {
	pub Particle: particle, // Add particle class from Carly
    pub ParticleForceGenerator: fg,
}
pub struct ParticleForceGenerator {
	pub gravity: Vec<ParticleForceRegistration> = Vec::new(),
}

// Holds the list of registrations.
let Registry: Vec<ParticleForceRegistration> = Vec::new();

impl <'t> ParticleForceRegistration <'t> {
    /**
    * Registers the given force generator to apply to the
    * given particle.
    */
	pub fn add(Particle* particle, ParticleForceGenerator *fg) {
        Registry.push(particle, fg);
	} 
    /**
    * Removes the given registered pair from the registry.
    * If the pair is not registered, this method will have
    * no effect.
    */
    pub fn remove(Particle* particle, ParticleForceGenerator *fg) {
        let i = 0;
        for item in Registry {
            if item.particle == particle && item.fg == fg{
                Registry.remove(i);
            } else{
                i = i + 1;
            }
        }
	}
    /**
    * Clears all registrations from the registry. This will
    * not delete the particles or the force generators
    * themselves, just the records of their connection.
    */
    pub fn clear() {
        Registry.clear();
	}
    /**
    * Calls all the force generators to update the forces of
    * their corresponding particles.
    */
    pub fn ParticleForceRegistry::updateForces(real duration) {
        //Registry::iterator i = registrations.begin();
        for item in Registry {
            item.fg.updateForce(item.particle, duration);
        }
    }
} // close ParticleForceRegistration impl

impl <'t> ParticleForceGenerator <'t> {
    /**
    * Registers the given force generator to apply to the
    * given particle.
    */
	pub fn ParticleGravity::updateForce(Particle* particle, real duration) {
        // Apply the mass-scaled force to the particle.
        particle.add_force(gravity * (particle.inverse_mass/1f32));
    }
} // close ParticleForceGenerator impl